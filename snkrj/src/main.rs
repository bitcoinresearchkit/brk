use snkrj::DatabaseUnique;

fn main() {
    let path = std::env::temp_dir().join("./db");

    // let database: DatabaseUnique<i32, i32> = DatabaseUnique::open(path.clone()).unwrap();
    // let _ = database.destroy();

    let mut database: DatabaseUnique<i32, i32> = DatabaseUnique::open(path.clone()).unwrap();
    database.insert(64, 128);
    database.export().unwrap();

    let mut database: DatabaseUnique<i32, i32> = DatabaseUnique::open(path).unwrap();
    database.insert(1, 2);
    database.insert(128, 256);
    println!("iter_ram:");
    database.iter_ram().for_each(|pair| {
        println!("{:?}", pair);
    });
    println!("iter_disk:");
    database.iter_disk().unwrap().for_each(|pair| {
        println!("{:?}", pair.unwrap());
    });
    println!("iter_ram_then_disk:");
    database.iter_ram_then_disk().unwrap().for_each(|pair| {
        println!("{:?}", pair);
    });
    database.export().unwrap();
}
