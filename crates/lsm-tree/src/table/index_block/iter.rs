// Copyright (c) 2024-present, fjall-rs
// This source code is licensed under both the Apache 2.0 and MIT License
// (found in the LICENSE-* files in the repository)

use std::cmp::Ordering;

use crate::{
    double_ended_peekable::{DoubleEndedPeekable, DoubleEndedPeekableExt},
    table::{KeyedBlockHandle, block::Decoder, index_block::IndexBlockParsedItem},
};

pub struct Iter<'a> {
    decoder: DoubleEndedPeekable<
        IndexBlockParsedItem,
        Decoder<'a, KeyedBlockHandle, IndexBlockParsedItem>,
    >,
}

impl<'a> Iter<'a> {
    #[must_use]
    pub fn new(decoder: Decoder<'a, KeyedBlockHandle, IndexBlockParsedItem>) -> Self {
        let decoder = decoder.double_ended_peekable();
        Self { decoder }
    }

    pub fn seek(&mut self, needle: &[u8], seqno: u64) -> bool {
        self.decoder.inner_mut().seek(
            |end_key, s| match end_key.cmp(needle) {
                Ordering::Greater => false,
                Ordering::Less => true,
                Ordering::Equal => s >= seqno,
            },
            true,
        )
    }

    pub fn seek_upper(&mut self, needle: &[u8], _seqno: u64) -> bool {
        self.decoder
            .inner_mut()
            .seek_upper(|end_key, _s| end_key <= needle, true)
    }
}

impl Iterator for Iter<'_> {
    type Item = IndexBlockParsedItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.decoder.next()
    }
}

impl DoubleEndedIterator for Iter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.decoder.next_back()
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::{
        Result,
        table::{
            Block, BlockHandle, BlockOffset, IndexBlock, KeyedBlockHandle,
            block::{BlockType, Header, ParsedItem},
        },
    };

    #[test]
    fn index_block_iter_seek_before_start() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"b".into(), 0, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(
                b"bcdef".into(),
                0,
                BlockHandle::new(BlockOffset(6_000), 7_000),
            ),
            KeyedBlockHandle::new(
                b"def".into(),
                0,
                BlockHandle::new(BlockOffset(13_000), 5_000),
            ),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        assert_eq!(index_block.len(), items.len());

        let mut iter = index_block.iter();
        assert!(iter.seek(b"a", 0), "should seek");

        let iter = index_block
            .iter()
            .map(|item| item.materialize(&index_block.inner.data));

        let real_items: Vec<_> = iter.collect();

        assert_eq!(items, &*real_items);

        Ok(())
    }

    #[test]
    fn index_block_iter_seek_start() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"b".into(), 0, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(
                b"bcdef".into(),
                0,
                BlockHandle::new(BlockOffset(6_000), 7_000),
            ),
            KeyedBlockHandle::new(
                b"def".into(),
                0,
                BlockHandle::new(BlockOffset(13_000), 5_000),
            ),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        assert_eq!(index_block.len(), items.len());

        let mut iter = index_block.iter();
        assert!(iter.seek(b"b", 1), "should seek");

        let real_items: Vec<_> = iter
            .map(|item| item.materialize(&index_block.inner.data))
            .collect();

        assert_eq!(items, &*real_items);

        Ok(())
    }

    #[test]
    fn index_block_iter_seek_middle() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"b".into(), 0, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(
                b"bcdef".into(),
                0,
                BlockHandle::new(BlockOffset(6_000), 7_000),
            ),
            KeyedBlockHandle::new(
                b"def".into(),
                0,
                BlockHandle::new(BlockOffset(13_000), 5_000),
            ),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        assert_eq!(index_block.len(), items.len());

        let mut iter = index_block.iter();
        assert!(iter.seek(b"c", 0), "should seek");

        let real_items: Vec<_> = iter
            .map(|item| item.materialize(&index_block.inner.data))
            .collect();

        assert_eq!(
            items.iter().skip(2).cloned().collect::<Vec<_>>(),
            &*real_items,
        );

        Ok(())
    }

    #[test]
    fn index_block_iter_rev_seek() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"b".into(), 0, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(
                b"bcdef".into(),
                0,
                BlockHandle::new(BlockOffset(6_000), 7_000),
            ),
            KeyedBlockHandle::new(
                b"def".into(),
                0,
                BlockHandle::new(BlockOffset(13_000), 5_000),
            ),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        assert_eq!(index_block.len(), items.len());

        let mut iter = index_block.iter();
        assert!(iter.seek_upper(b"c", 0), "should seek");

        let real_items: Vec<_> = iter
            .map(|item| item.materialize(&index_block.inner.data))
            .collect();

        assert_eq!(items.to_vec(), &*real_items);

        Ok(())
    }

    #[test]
    fn index_block_iter_rev_seek_2() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"b".into(), 0, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(
                b"bcdef".into(),
                0,
                BlockHandle::new(BlockOffset(6_000), 7_000),
            ),
            KeyedBlockHandle::new(
                b"def".into(),
                0,
                BlockHandle::new(BlockOffset(13_000), 5_000),
            ),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        assert_eq!(index_block.len(), items.len());

        let mut iter = index_block.iter();
        assert!(iter.seek_upper(b"e", 0), "should seek");

        let real_items: Vec<_> = iter
            .map(|item| item.materialize(&index_block.inner.data))
            .collect();

        assert_eq!(items.to_vec(), &*real_items);

        Ok(())
    }

    #[test]
    fn index_block_iter_rev_seek_3() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"b".into(), 0, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(
                b"bcdef".into(),
                0,
                BlockHandle::new(BlockOffset(6_000), 7_000),
            ),
            KeyedBlockHandle::new(
                b"def".into(),
                0,
                BlockHandle::new(BlockOffset(13_000), 5_000),
            ),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        assert_eq!(index_block.len(), items.len());

        let mut iter = index_block.iter();
        assert!(iter.seek_upper(b"b", 1), "should seek");

        let real_items: Vec<_> = iter
            .map(|item| item.materialize(&index_block.inner.data))
            .collect();

        assert_eq!(
            items.iter().take(2).cloned().collect::<Vec<_>>(),
            &*real_items,
        );

        Ok(())
    }

    #[test]
    fn index_block_iter_too_far() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"b".into(), 0, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(
                b"bcdef".into(),
                0,
                BlockHandle::new(BlockOffset(6_000), 7_000),
            ),
            KeyedBlockHandle::new(
                b"def".into(),
                0,
                BlockHandle::new(BlockOffset(13_000), 5_000),
            ),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        assert_eq!(index_block.len(), items.len());

        let mut iter = index_block.iter();
        assert!(!iter.seek(b"zzz", 0), "should not seek");

        let real_items: Vec<_> = iter
            .map(|item| item.materialize(&index_block.inner.data))
            .collect();

        assert_eq!(&[] as &[KeyedBlockHandle], &*real_items);

        Ok(())
    }

    #[test]
    fn index_block_iter_too_far_next_back() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"b".into(), 0, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(
                b"bcdef".into(),
                0,
                BlockHandle::new(BlockOffset(6_000), 7_000),
            ),
            KeyedBlockHandle::new(
                b"def".into(),
                0,
                BlockHandle::new(BlockOffset(13_000), 5_000),
            ),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        let mut iter = index_block.iter();
        assert!(!iter.seek(b"zzz", 0), "should not seek");

        assert!(iter.next().is_none(), "iterator should be exhausted");
        assert!(
            iter.next_back().is_none(),
            "reverse iterator should also be exhausted"
        );

        Ok(())
    }

    #[test]
    fn index_block_mvcc_slab() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"a".into(), 3, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(b"a".into(), 1, BlockHandle::new(BlockOffset(6_000), 7_000)),
            KeyedBlockHandle::new(b"b".into(), 4, BlockHandle::new(BlockOffset(13_000), 5_000)),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        assert_eq!(index_block.len(), items.len());

        {
            let mut iter = index_block.iter();
            assert!(iter.seek(b"a", 5), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(items, &*real_items);
        }

        {
            let mut iter = index_block.iter();
            assert!(iter.seek(b"a", 4), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(items, &*real_items);
        }

        {
            let mut iter = index_block.iter();
            assert!(iter.seek(b"a", 3), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(
                items.iter().skip(1).cloned().collect::<Vec<_>>(),
                &*real_items,
            );
        }

        {
            let mut iter = index_block.iter();
            assert!(iter.seek(b"a", 2), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(
                items.iter().skip(1).cloned().collect::<Vec<_>>(),
                &*real_items,
            );
        }

        {
            let mut iter = index_block.iter();
            assert!(iter.seek(b"a", 1), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(
                items.iter().skip(2).cloned().collect::<Vec<_>>(),
                &*real_items,
            );
        }

        {
            let mut iter = index_block.iter();
            assert!(iter.seek(b"a", 0), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(
                items.iter().skip(2).cloned().collect::<Vec<_>>(),
                &*real_items,
            );
        }

        Ok(())
    }

    #[test]
    fn index_block_iter_span() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"a".into(), 1, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(b"a".into(), 0, BlockHandle::new(BlockOffset(6_000), 7_000)),
            KeyedBlockHandle::new(b"b".into(), 0, BlockHandle::new(BlockOffset(13_000), 5_000)),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        assert_eq!(index_block.len(), items.len());

        {
            let mut iter = index_block.iter();
            assert!(iter.seek(b"a", 2), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(items.to_vec(), &*real_items);
        }

        {
            let mut iter = index_block.iter();
            assert!(iter.seek(b"b", 1), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(
                items.iter().skip(2).cloned().collect::<Vec<_>>(),
                &*real_items,
            );
        }

        Ok(())
    }

    #[test]
    fn index_block_iter_rev_span() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"a".into(), 1, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(b"a".into(), 0, BlockHandle::new(BlockOffset(6_000), 7_000)),
            KeyedBlockHandle::new(b"b".into(), 0, BlockHandle::new(BlockOffset(13_000), 5_000)),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        assert_eq!(index_block.len(), items.len());

        {
            let mut iter = index_block.iter();
            assert!(iter.seek_upper(b"a", 2), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(items.to_vec(), &*real_items);
        }

        {
            let mut iter = index_block.iter();
            assert!(iter.seek_upper(b"b", 1), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(items.to_vec(), &*real_items);
        }

        Ok(())
    }

    #[test]
    fn index_block_iter_range_1() -> Result<()> {
        let items = [
            KeyedBlockHandle::new(b"a".into(), 0, BlockHandle::new(BlockOffset(0), 6_000)),
            KeyedBlockHandle::new(b"b".into(), 0, BlockHandle::new(BlockOffset(13_000), 5_000)),
            KeyedBlockHandle::new(b"c".into(), 0, BlockHandle::new(BlockOffset(13_000), 5_000)),
            KeyedBlockHandle::new(b"d".into(), 0, BlockHandle::new(BlockOffset(13_000), 5_000)),
            KeyedBlockHandle::new(b"e".into(), 0, BlockHandle::new(BlockOffset(13_000), 5_000)),
        ];

        let bytes = IndexBlock::encode_into_vec(&items)?;

        let index_block = IndexBlock::new(Block {
            data: bytes.into(),
            header: Header {
                block_type: BlockType::Index,
                data_length: 0,
                uncompressed_length: 0,
            },
        });

        assert_eq!(index_block.len(), items.len());

        {
            let mut iter = index_block.iter();
            assert!(iter.seek(b"b", 1), "should seek");
            assert!(iter.seek_upper(b"c", 1), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(
                items.iter().skip(1).take(3).cloned().collect::<Vec<_>>(),
                &*real_items,
            );
        }

        {
            let mut iter = index_block.iter();
            assert!(iter.seek(b"b", 1), "should seek");
            assert!(iter.seek_upper(b"c", 1), "should seek");

            let real_items: Vec<_> = iter
                .map(|item| item.materialize(&index_block.inner.data))
                .collect();

            assert_eq!(
                items
                    .iter()
                    .skip(1)
                    .take(3)
                    .rev()
                    .cloned()
                    .collect::<Vec<_>>(),
                &*real_items,
            );
        }

        Ok(())
    }
}
