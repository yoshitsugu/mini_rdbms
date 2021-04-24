pub mod buffer;
pub mod disk;

use disk::*;
use std::io;

fn main() -> io::Result<()> {
    dbg!(PageId(333));
    let mut heap = DiskManager::open("/tmp/heap1")?;
    heap.write_page_data(PageId(0), &[10u8, 9u8, 3u8])?;
    heap.write_page_data(PageId(1), &[20u8, 11u8, 7u8])?;
    heap.write_page_data(PageId(2), &[1u8, 4u8, 21u8])?;
    let mut b = [0; 4];
    heap.read_page_data(PageId(1), &mut b)?;
    dbg!(b);
    return Ok(());
}
