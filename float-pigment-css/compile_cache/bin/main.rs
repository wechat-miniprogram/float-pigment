use serde::Serialize;
use std::env::VarError;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::*;

#[derive(Serialize)]
struct CacheConfig {
    version: String,
    enum_total: i32,
    struct_total: i32,
}

fn get_version_from_cargo() -> Result<String, VarError> {
    std::env::var("CARGO_PKG_VERSION")
}
fn write_version_to_cache(cfg: CacheConfig) {
    let mut pb = PathBuf::new();
    pb.push(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    pb.push("compile_cache");
    pb.push("publish");
    pb.push("version.toml");
    let mut options = OpenOptions::new();
    let mut file = options
        .write(true)
        .read(true)
        .truncate(true)
        .create(true)
        .open(&pb)
        .unwrap();
    let cfg_toml = toml::to_string(&cfg).unwrap();
    file.write_all(cfg_toml.as_bytes())
        .expect("write version error");
}

fn folder_copy(path: &str, target_path: &str) -> i32 {
    let mut count = 0;
    for entry in std::fs::read_dir(path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
    {
        let entry = entry.unwrap();
        let file_name = entry.file_name().unwrap().to_str().unwrap();
        let mut pb = PathBuf::new();
        pb.push(target_path);
        pb.push(file_name);
        let target = pb.to_str().unwrap();
        std::fs::copy(entry, target).expect("copy error");
        count += 1;
    }
    count
}

fn main() {
    // check version
    let version = match get_version_from_cargo() {
        Ok(ver) => ver,
        Err(_) => panic!("Check pkg version in Cargo.toml"),
    };
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut pb = PathBuf::new();
    pb.push(root);
    pb.push("compile_cache");
    pb.push("enum");
    let __pb = pb.clone();
    let cur_enum_path = __pb.to_str().unwrap();
    pb.pop();
    pb.push("struct");
    let __pb = pb.clone();
    let cur_struct_path = __pb.to_str().unwrap();
    pb.pop();
    pb.push("publish");
    pb.push("enum");
    let __pb = pb.clone();
    let publish_enum_path = __pb.to_str().unwrap();
    pb.pop();
    pb.push("struct");
    let __pb = pb.clone();
    let publish_struct_path = __pb.to_str().unwrap();

    let enum_total = folder_copy(cur_enum_path, publish_enum_path);
    let struct_total = folder_copy(cur_struct_path, publish_struct_path);
    let cfg = CacheConfig {
        version,
        enum_total,
        struct_total,
    };
    // write version
    write_version_to_cache(cfg);
}
