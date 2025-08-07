fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_default();

    if profile == "release" {
        println!("cargo:rustc-flag=-C");
        println!("cargo:rustc-flag=target-cpu=native");
    }
}
