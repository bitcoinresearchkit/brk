# Mempool Projected Blocks - Design Document

## Goal

Efficiently maintain projected blocks for fee estimation without rebuilding everything on each mempool change.

## Core Idea

Instead of rebuilding all projected blocks on every insert/remove:
1. Insert new tx/package into the correct block (binary search by fee rate)
2. Cascade overflow: if block > 1MB, move lowest fee rate item to next block
3. On remove: cascade up to fill the gap

## Data Structures

### MempoolPackage

```rust,ignore
enum MempoolPackage {
    /// Tx with no unconfirmed parents - can be mined independently
    Independent(MempoolEntry),

    /// Tx(s) bundled for CPFP - ancestors + paying descendant
    Bundle {
        /// The descendant tx that "pays for" the ancestors
        child: Txid,
        /// All txs in topological order (ancestors first, child last)
        entries: Vec<MempoolEntry>,
        /// Sum of all fees
        total_fee: Sats,
        /// Sum of all vsizes
        total_vsize: VSize,
    },

    /// Tx waiting for unconfirmed parent(s) that are in a different Bundle
    /// Not placed in projected blocks until dependencies clear
    Pending {
        entry: MempoolEntry,
        /// Parent txids this tx is waiting for
        waiting_for: FxHashSet<Txid>,
    },
}
```

### ProjectedBlock

```rust,ignore
struct ProjectedBlock {
    /// Packages sorted by fee rate (highest first)
    packages: Vec<MempoolPackage>,
    /// Current total vsize
    total_vsize: VSize,
    /// Running fee total
    total_fee: Sats,
}
```

## Insert Algorithm

### Case 1: No unconfirmed parents
```
tx A (10 sat/vB), no unconfirmed inputs
```
→ Create `Independent(A)`
→ Binary search blocks by fee rate, insert, cascade overflow

### Case 2: Has unconfirmed parent(s), CPFP beneficial
```
tx P (2 sat/vB) → tx C (50 sat/vB)
Package rate = (P.fee + C.fee) / (P.vsize + C.vsize) = 26 sat/vB
26 > max(2, 50)? No, 26 < 50
```
Wait, need to reconsider: CPFP is beneficial when child NEEDS parent to be mined.
The package rate matters for when C will be mined (C can't be mined without P).

Actually: C's effective rate = package rate, always. Because C cannot be mined without P.

So: Create `Bundle { child: C, entries: [P, C], rate: 26 }`

If P was already `Independent(P)` in a block:
1. Remove P from its block
2. Create Bundle
3. Insert Bundle at rate 26 sat/vB

### Case 3: Chain of unconfirmed txs
```
tx A (2 sat/vB) → tx B (3 sat/vB) → tx C (100 sat/vB)
```
C's package = A + B + C
→ `Bundle { child: C, entries: [A, B, C] }`

If A and B were already placed:
1. Remove A, B from their positions
2. Create Bundle with all three
3. Insert at package fee rate

### Case 4: Diamond dependency
```
    tx A (5 sat/vB)
   ↙            ↘
tx B (2)       tx C (2)
   ↘            ↙
    tx D (100 sat/vB)
```
D's ancestors = {A, B, C}
→ `Bundle { child: D, entries: [A, B, C, D] }` (topological order)

### Case 5: Multiple children competing for same parent
```
tx P (2 sat/vB)
    ├→ tx C1 (50 sat/vB)  → package rate 26 sat/vB
    └→ tx C2 (100 sat/vB) → package rate 51 sat/vB
```
P can only be in ONE package (can only be mined once).

Choose the highest package fee rate: P + C2 at 51 sat/vB
→ `Bundle { child: C2, entries: [P, C2] }`

What happens to C1?
- C1 cannot be mined until P is mined
- C1 becomes `Pending { entry: C1, waiting_for: {P} }`
- C1 is NOT in projected blocks

When Bundle(P, C2) is mined:
- P leaves mempool
- C1's waiting_for becomes empty
- C1 converts to `Independent(C1)` at 50 sat/vB
- C1 gets inserted into projected blocks

### Case 6: New tx improves existing Bundle
```
Existing: Bundle { child: C, entries: [P, C], rate: 26 }
New tx D spends C's output, D has very high fee
New package rate (P + C + D) = 40 sat/vB
```
Since 40 > 26:
1. Remove old Bundle from its block
2. Create new `Bundle { child: D, entries: [P, C, D] }`
3. Insert at new rate

### Case 7: New tx doesn't improve, becomes Pending
```
Existing: Bundle { child: C, entries: [P, C], rate: 26 }
New tx D spends C's output, D has low fee
New package rate (P + C + D) = 20 sat/vB
```
Since 20 < 26, D doesn't help:
→ `Pending { entry: D, waiting_for: {C} }`

When Bundle(P, C) is mined:
→ D converts to `Independent(D)`

## Remove Algorithm

### Case 1: Remove an `Independent`
```
Remove Independent(A) from block 3
```
1. Remove A from block 3
2. Block 3 now has space
3. Pull highest fee rate package from block 4 into block 3
4. Cascade: pull from block 5 to block 4, etc.
5. Stop when a block has no underflow or no more blocks

### Case 2: Remove the "child" (paying tx) of a Bundle
```
Bundle { child: C, entries: [P, C] } in block 2
C gets dropped/RBF'd (NOT confirmed - if confirmed, P would be too)
```
1. Remove Bundle from block 2
2. P has no more CPFP boost
3. P becomes `Independent(P)` at its own rate (2 sat/vB)
4. Insert P into appropriate block (probably much later)
5. Cascade to fill block 2's gap

### Case 3: Remove an ancestor from a Bundle (confirmation)
```
Bundle { child: C, entries: [P, C], rate: 26 } in block 2
P gets confirmed (separate from C? unusual but possible in reorg)
```
1. Remove Bundle from block 2
2. C no longer needs P
3. C becomes `Independent(C)` at 50 sat/vB
4. Insert C into earlier block (higher rate now)
5. Cascade as needed

### Case 4: Remove tx that has Pending descendants
```
Independent(P) in block 5
Pending { entry: C, waiting_for: {P} }
P gets confirmed
```
1. Remove P from block 5
2. Find all Pending txs waiting for P
3. For each: remove P from waiting_for
4. If waiting_for is empty: convert to Independent, insert into blocks
5. Cascade to fill block 5's gap

### Case 5: Remove middle of a chain (tx dropped/invalid)
```
Bundle { child: D, entries: [A, B, C, D] }
B gets dropped (double-spend, RBF, etc.)
```
B invalid means C and D are invalid too (missing input):
1. Remove entire Bundle
2. A becomes `Independent(A)` if still valid
3. C, D removed from mempool entirely

### Case 6: RBF replacement
```
Independent(A) in block 3
New tx A' replaces A (same inputs, higher fee)
```
1. Remove A (and any descendants - they're now invalid)
2. Insert A' as new Independent or Bundle

## Pending → Active Transitions

When a tx is removed (confirmed), check all `Pending` entries:

```rust,ignore
fn on_tx_removed(&mut self, txid: Txid) {
    let pending_to_update: Vec<_> = self.pending
        .iter()
        .filter(|(_, p)| p.waiting_for.contains(&txid))
        .map(|(id, _)| *id)
        .collect();

    for pending_txid in pending_to_update {
        let pending = self.pending.get_mut(&pending_txid).unwrap();
        pending.waiting_for.remove(&txid);

        if pending.waiting_for.is_empty() {
            // Convert to Independent and insert
            let entry = self.pending.remove(&pending_txid).unwrap();
            self.insert_independent(entry.entry);
        }
    }
}
```

## Cascade Algorithm

### Cascade Down (after insert causes overflow)

```rust,ignore
fn cascade_down(&mut self, starting_block: usize) {
    let mut block_idx = starting_block;

    while block_idx < self.blocks.len() {
        let block = &mut self.blocks[block_idx];

        if block.total_vsize <= BLOCK_VSIZE_TARGET {
            break; // No overflow
        }

        // Pop lowest fee rate package
        let overflow = block.packages.pop().unwrap();
        block.total_vsize -= overflow.vsize();
        block.total_fee -= overflow.fee();

        // Push to next block
        if block_idx + 1 >= self.blocks.len() {
            self.blocks.push(ProjectedBlock::new());
        }

        let next_block = &mut self.blocks[block_idx + 1];
        // Insert at beginning (it's the highest fee rate in this block)
        next_block.packages.insert(0, overflow);
        next_block.total_vsize += overflow.vsize();
        next_block.total_fee += overflow.fee();

        block_idx += 1;
    }
}
```

### Cascade Up (after remove causes underflow)

```rust,ignore
fn cascade_up(&mut self, starting_block: usize) {
    let mut block_idx = starting_block;

    while block_idx < self.blocks.len() {
        let current_vsize = self.blocks[block_idx].total_vsize;

        if current_vsize >= BLOCK_VSIZE_TARGET {
            break; // Block is full enough
        }

        if block_idx + 1 >= self.blocks.len() {
            break; // No more blocks to pull from
        }

        let next_block = &mut self.blocks[block_idx + 1];
        if next_block.packages.is_empty() {
            // Remove empty block
            self.blocks.remove(block_idx + 1);
            break;
        }

        // Pull highest fee rate package from next block
        let pulled = next_block.packages.remove(0);
        next_block.total_vsize -= pulled.vsize();
        next_block.total_fee -= pulled.fee();

        // Add to current block
        let current_block = &mut self.blocks[block_idx];
        current_block.packages.push(pulled);
        current_block.total_vsize += pulled.vsize();
        current_block.total_fee += pulled.fee();

        block_idx += 1;
    }

    // Clean up empty trailing blocks
    while self.blocks.last().map(|b| b.packages.is_empty()).unwrap_or(false) {
        self.blocks.pop();
    }
}
```

## Data Structure for Efficient Operations

Need to track:
1. `txid → MempoolPackage` - which package contains a tx
2. `txid → block_index` - which block a tx/package is in
3. `txid → Vec<Txid>` - descendants waiting for this tx (Pending)
4. Per-block: sorted packages by fee rate

```rust,ignore
struct MempoolState {
    /// All packages (Independent, Bundle, or Pending)
    packages: FxHashMap<Txid, MempoolPackage>,

    /// For Bundle: maps each txid in bundle to the child txid (package key)
    tx_to_package: FxHashMap<Txid, Txid>,

    /// Maps package key (txid) to block index, None if Pending
    package_to_block: FxHashMap<Txid, Option<usize>>,

    /// Maps txid to list of Pending txids waiting for it
    waiting_on: FxHashMap<Txid, FxHashSet<Txid>>,

    /// The projected blocks
    blocks: Vec<ProjectedBlock>,
}
```

## Open Questions

1. **Block fullness threshold**: Should we cascade when exactly at 1MB, or allow slight overflow?

2. **Minimum fee rate**: Packages below minimum relay fee should be excluded?

3. **Maximum ancestors**: Bitcoin Core limits ancestor count (25). Should we?

4. **Memory bounds**: For huge mempools, should we limit projected blocks count?

## mempool.space Implementation Analysis

Source: [mempool/mempool rust/gbt/src](https://github.com/mempool/mempool/tree/master/rust/gbt/src)

### Key Files
- `gbt.rs` - Main block building algorithm
- `audit_transaction.rs` - Transaction wrapper with ancestor tracking
- `thread_transaction.rs` - Lightweight tx representation

### Their Approach

**Not incremental!** They rebuild from scratch but optimize heavily:

1. **Use numeric UIDs** instead of 32-byte txids - massive memory/hash savings
2. **Ancestor score** = (fee + ancestor_fees) / (weight + ancestor_weights)
3. **Two-source selection**:
   - `mempool_stack`: Original sorted order
   - `modified` priority queue: Txs whose scores changed due to parent selection
4. **Greedy selection loop**:
   - Pick highest ancestor score from either source
   - Include all ancestors first
   - Update all descendants' scores (via `update_descendants()`)
   - Move affected descendants to `modified` queue

### Why They Don't Do Incremental

The `update_descendants()` cascade is the key insight: when you select a tx, ALL its descendants need score recalculation because their ancestor set changed. This cascade can touch a huge portion of the mempool.

Their solution: rebuild fast rather than update incrementally.
- Use u32 UIDs (4 bytes vs 32 bytes)
- Pre-allocate with capacity 1,048,576
- Custom u32-based hasher
- Run in separate thread via `spawn_blocking()`

### Should We Follow Their Approach?

**Pros of their approach:**
- Simpler code, fewer edge cases
- Proven correct (Bitcoin Core algorithm port)
- Fast enough with optimizations (sub-100ms for full rebuild)

**Pros of incremental:**
- Lower latency for small changes
- Less CPU spike on each update
- Better for very high tx throughput

**Recommendation:** Start with their approach (rebuild with optimizations), measure performance. Only add incremental if needed.

### Simplified Algorithm (from mempool.space)

```
1. Build audit_pool: Map<uid, AuditTransaction>
2. set_relatives(): compute ancestors for each tx
3. Sort by ancestor_score descending
4. while mempool not empty:
     a. Pick best from (stack, modified_queue)
     b. If ancestors not yet selected, select them first
     c. Add to current block
     d. If block full, start new block
     e. update_descendants(): recalc scores, add to modified_queue
5. Return blocks
```

### Key Optimization: Ancestor Score

```rust,ignore
ancestor_score = (tx.fee + sum(ancestor.fee)) / (tx.weight + sum(ancestor.weight))
```

This single metric captures CPFP naturally - a high-fee child boosts its low-fee parents by being evaluated as a package.

## Revised Implementation Plan

Given mempool.space's approach works well, simplify our design:

### Phase 1: Fast Rebuild (MVP)
- Use u32 UIDs for txs
- Compute ancestor scores
- Greedy selection with modified queue
- Rebuild on each change (with spawn_blocking)

### Phase 2: Incremental (If Needed)
- Track which txs changed
- Only rebuild affected portions
- Cascade updates through dependency graph

### Phase 3: Further Optimizations
- Batch updates (coalesce rapid changes)
- Dirty flag + lazy rebuild
- Background thread continuous updates

## References

- Bitcoin Core's block assembly: `src/node/miner.cpp`
- [mempool.space Rust GBT](https://github.com/mempool/mempool/tree/master/rust/gbt/src)
- [mempool-blocks.ts](https://github.com/mempool/mempool/blob/master/backend/src/api/mempool-blocks.ts)
