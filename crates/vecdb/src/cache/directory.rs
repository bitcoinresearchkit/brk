use crate::VecValue;

use super::{Request, Shared, Table, Value};

const MERGE_BYTES: usize = 64 * 1024;

impl<T: VecValue> Shared<T> {
    // Fills contain only missing ranges, so existing entries never overlap.
    pub(super) fn insert(&self, mut start: usize, mut value: Value<T>) {
        if value
            .allocation_bytes()
            .and_then(|bytes| bytes.checked_add(Table::<T>::charge_for(1)))
            .is_none_or(|bytes| bytes > self.budget.limit())
        {
            return;
        }
        let end = start + value.len();
        let limit = MERGE_BYTES / size_of::<T>().max(1);
        let mut table = self.table.write();
        let previous = table
            .range(..start)
            .next_back()
            .and_then(|(&at, previous)| {
                debug_assert!(at + previous.len() <= start);
                (at + previous.len() == start && previous.len() + value.len() <= limit)
                    .then_some(at)
            });
        debug_assert!(table.range(start..end).next().is_none());
        let previous = previous.map(|at| (at, table.remove(&at).unwrap()));
        let combined_len = value.len() + previous.as_ref().map_or(0, |(_, value)| value.len());
        let next = table
            .get(&end)
            .filter(|next| combined_len + next.len() <= limit)
            .map(|_| end);
        let next = next.and_then(|at| table.remove(&at));
        table.shrink_charge();
        drop(table);

        if let Some((at, mut previous)) = previous {
            if Self::append(&mut previous, &value, limit) {
                start = at;
                value = previous;
            } else {
                self.store(at, previous);
            }
        }
        if let Some(mut next) = next {
            let merged = if next.len() > value.len() {
                let len = value.len() + next.len();
                let buffer = next.mutable();
                if buffer.reserve(len, limit) {
                    buffer.values.splice(..0, value.slice().iter().cloned());
                    value = next;
                    None
                } else {
                    Some(next)
                }
            } else if Self::append(&mut value, &next, limit) {
                None
            } else {
                Some(next)
            };
            if let Some(next) = merged {
                self.store(end, next);
            }
        }
        self.store(start, value);
    }

    fn append(left: &mut Value<T>, right: &Value<T>, limit: usize) -> bool {
        let len = left.len() + right.len();
        let buffer = left.mutable();
        if !buffer.reserve(len, limit) {
            return false;
        }
        buffer.values.extend_from_slice(right.slice());
        true
    }

    fn store(&self, start: usize, mut value: Value<T>) {
        let budget = self.budget;
        if !value.admit(budget) {
            return;
        }
        let Some(mut charge) = budget.reserve(Table::<T>::entry_bytes()) else {
            return;
        };
        let mut table = self.table.write();
        if table.is_empty() {
            drop(table);
            let Some(root) = budget.reserve(Table::<T>::root_bytes()) else {
                return;
            };
            charge.merge(root);
            table = self.table.write();
        }
        debug_assert!(!table.contains_key(&start));
        table.insert(start, value);
        table.add_charge(charge);
    }

    pub(super) fn invalidate_from(&self, from: usize) {
        let mut table = self.table.write();
        let prefix = table
            .range(..from)
            .next_back()
            .and_then(|(&at, value)| (at + value.len() > from).then_some(at));
        let removed: Vec<_> = table.extract_if(from.., |_, _| true).collect();
        let prefix = prefix.map(|at| (at, table.remove(&at).unwrap()));
        table.shrink_charge();
        drop(table);
        drop(removed);
        if let Some((at, value)) = prefix {
            self.store(at, value.truncate(from - at));
        }
    }

    pub(super) fn borrowed(&self, request: Request<'_>) -> Vec<(usize, Value<T>)> {
        let table = self.table.read();
        match request {
            Request::Range(from, to) => {
                if from >= to {
                    return Vec::new();
                }
                let first = table.range(..=from).next_back().map_or(from, |(&at, _)| at);
                table
                    .range(first..to)
                    .filter(|(at, value)| **at + value.len() > from)
                    .map(|(&at, value)| (at, value.clone()))
                    .collect()
            }
            Request::Sorted(indices) => {
                let mut values = Vec::new();
                for &index in indices {
                    if values
                        .last()
                        .is_some_and(|(at, value): &(usize, Value<T>)| index < *at + value.len())
                    {
                        continue;
                    }
                    if let Some((&at, value)) = table.range(..=index).next_back()
                        && index - at < value.len()
                    {
                        values.push((at, value.clone()));
                    }
                }
                values
            }
        }
    }

    /// An all-or-nothing probe: a miss does not clone values or change output.
    pub(super) fn copy_range(&self, from: usize, to: usize, out: &mut Vec<T>) -> bool {
        if from >= to {
            return true;
        }
        let table = self.table.read();
        let first = table.range(..=from).next_back().map_or(from, |(&at, _)| at);
        let ranges = table.range(first..to);
        let mut at = from;
        for (&start, values) in ranges.clone() {
            if start > at {
                return false;
            }
            at = at.max(start + values.len());
        }
        if at < to {
            return false;
        }
        out.reserve(to - from);
        for (&start, values) in ranges {
            let end = (to - start).min(values.len());
            out.extend_from_slice(&values.slice()[from.saturating_sub(start)..end]);
        }
        true
    }

    /// Copies the retained prefix once and returns only the unfulfilled request.
    pub(super) fn copy_prefix<'a>(
        &self,
        request: Request<'a>,
        out: &mut Vec<T>,
    ) -> Option<Request<'a>> {
        let table = self.table.read();
        match request {
            Request::Range(from, to) => {
                if from >= to {
                    return None;
                }
                let first = table
                    .range(..=from)
                    .next_back()
                    .map_or(from, |(&start, _)| start);
                let mut at = from;
                for (&start, values) in table.range(first..to) {
                    if start > at {
                        break;
                    }
                    let end = (start + values.len()).min(to);
                    if at < end {
                        out.extend_from_slice(&values.slice()[at - start..end - start]);
                        at = end;
                    }
                    if at == to {
                        break;
                    }
                }
                (at < to).then_some(Request::Range(at, to))
            }
            Request::Sorted(indices) => {
                if indices.is_empty() {
                    return None;
                }
                let first = table.range(..=indices[0]).next_back();
                // One retained range proves coverage for the entire sorted request.
                if let Some((&at, value)) = first
                    && indices[indices.len() - 1] - at < value.len()
                {
                    let values = value.slice();
                    out.extend(indices.iter().map(|&index| values[index - at].clone()));
                    return None;
                }
                let first = first.map_or(indices[0], |(&at, _)| at);
                let mut iter = table.range(first..);
                let mut current = iter.next();
                let mut remaining = indices;
                while let Some((&index, rest)) = remaining.split_first() {
                    if current.is_some_and(|(&at, value)| at + value.len() <= index) {
                        current = iter.next();
                        if current.is_some_and(|(&at, value)| at + value.len() <= index) {
                            let start = *table.range(..=index).next_back().unwrap().0;
                            iter = table.range(start..);
                            current = iter.next();
                        }
                    }
                    let Some(value) = current.and_then(|(&at, value)| {
                        index.checked_sub(at).and_then(|offset| value.get(offset))
                    }) else {
                        break;
                    };
                    out.push(value);
                    remaining = rest;
                }
                (!remaining.is_empty()).then_some(Request::Sorted(remaining))
            }
        }
    }
}
