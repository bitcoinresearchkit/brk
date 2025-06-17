use log::info;

pub fn pause() {
    info!("Press enter to continue...");
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
}
