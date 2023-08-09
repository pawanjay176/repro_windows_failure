use std::path::PathBuf;

fn main() {
    let mut cc = cc::Build::new().file("data.c").compile("data");

    let pwd = PathBuf::from(".");
    dbg!(pwd.display());
    println!("cargo:rustc-link-search={}", pwd.display());
    println!("cargo:rustc-link-lib=data");
    
}
