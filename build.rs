use std::{env, path::PathBuf};
	
	
fn main() {
    let _target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    //println!("cargo:warning=target_os: {}", target_os);

    let _oal_soft_dir = PathBuf::from("openal-soft");
    //println!("cargo:warning=oal_soft_dir: {:?}", oal_soft_dir);

    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    println!(r"cargo:rustc-link-search=/opt/homebrew/opt/openal-soft/lib");

    #[cfg(not(target_arch = "aarch64"))]
    println!(r"cargo:rustc-link-search=/usr/local/opt/openal-soft/lib");

    println!("cargo:rustc-link-lib=dylib=openal");
    //println!("cargo:warning=This is a custom build script!");
}
