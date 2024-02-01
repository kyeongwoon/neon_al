use std::{env, path::PathBuf};
	
	
fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    println!("cargo:warning=target_os: {}", target_os);

    let oal_soft_dir = PathBuf::from("openal-soft");
    println!("cargo:warning=oal_soft_dir: {:?}", oal_soft_dir);

    #[cfg(any(target_os = "macos", target_os = "macos"))]
    println!(r"cargo:rustc-link-search=/opt/homebrew/opt/openal-soft/lib");

    println!("cargo:rustc-link-lib=dylib=openal");


    // 빌드 과정 중에 사용자 정의 작업 수행
    println!("cargo:warning=This is a custom build script!");
}
