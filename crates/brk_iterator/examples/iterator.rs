use brk_iterator::BlockIterator;
use brk_reader::Reader;

fn main() {
    let reader = Reader::new(blocks_dir, rpc);
    BlockIterator::last(10).reader(reader, client);
}
