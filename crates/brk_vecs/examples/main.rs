use std::{fs, path::Path};

use brk_core::Result;
use brk_vecs::{File, PAGE_SIZE};

fn main() -> Result<()> {
    let _ = fs::remove_dir_all("vecs");

    let file = File::open(Path::new("vecs"))?;

    let region1_i = file.create_region_if_needed("region1")?;

    dbg!(region1_i);

    assert!(file.get_region(region1_i).unwrap().read().len() == 0);

    file.write_all(region1_i, &[0, 1, 2, 3, 4])?;

    {
        let opt = file.get_region(region1_i);
        let region = opt.as_ref().unwrap().read();
        assert!(region.start() == 0 && region.len() == 5 && region.reserved() == PAGE_SIZE);
    }

    assert!(file.mmap.read()[0..10] == [0, 1, 2, 3, 4, 0, 0, 0, 0, 0]);

    file.write_all(region1_i, &[5, 6, 7, 8, 9])?;

    assert!(file.mmap.read()[0..10] == [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);

    file.write_all_at(region1_i, &[1, 2], 0)?;

    assert!(file.mmap.read()[0..10] == [1, 2, 2, 3, 4, 5, 6, 7, 8, 9]);

    {
        let opt = file.get_region(region1_i);
        let region = opt.as_ref().unwrap().read();
        dbg!(&region);
        assert!(region.start() == 0 && region.len() == 10 && region.reserved() == PAGE_SIZE);
    }

    // file.set_min_len(PAGE_SIZE * 1_000_000)?;

    Ok(())
}
