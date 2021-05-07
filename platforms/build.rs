fn main() {
    let target = std::env::var("TARGET").expect("TARGET env var not set");
    println!("cargo:rustc-env=TARGET={}", target);
}
