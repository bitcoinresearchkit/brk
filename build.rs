fn main() {
    let profile = std::env::var("PROFILE").unwrap_or_default();

    if profile == "release" {
        println!("cargo:rustc-flag=-C");
        println!("cargo:rustc-flag=target-cpu=native");

        #[cfg(target_arch = "x86_64")]
        {
            println!("cargo:rustc-flag=-C");
            println!("cargo:rustc-flag=target-feature=+bmi1,+bmi2,+avx2");
        }
    }
}
