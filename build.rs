use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut lib_dir = manifest_dir.clone();
    lib_dir.push("bin");
    lib_dir.push(target.clone());
    println!("cargo:rustc-link-search=all={}", lib_dir.display());

    let mut dll_dir_path = manifest_dir.clone();
    dll_dir_path.push("dll");
    dll_dir_path.push(target);

    if let Ok(dll_dir) = std::fs::read_dir(dll_dir_path) {
        for entry in dll_dir {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                let file_name = file_name.to_str().unwrap();
                if file_name.ends_with(".dll") {
                    new_file_path.push(file_name);
                    std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
                }
            }
        }
    }

}