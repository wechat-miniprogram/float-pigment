mod css_source;
mod utils;
use std::path::Path;

use css_source::{
    base::base_css_assert, padding_case::padding_case_assert, style::style_css_assert,
    style_v2::style_v2_css_assert,
};
use float_pigment_css::{StyleSheetGroup, StyleSheetResource};
use utils::{dir_files_path, get_current_commit};

use self::utils::{read_bincode, read_file_as_string, write_bincode};

fn compile_bincode() {
    gen_bincode_with_current_compiler();
    deserialize_bincode_source_test()
}

fn gen_bincode_with_current_compiler() {
    let current_dir =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests/compatibility");
    let css_source_dir = current_dir.join("css_source");
    dir_files_path(&css_source_dir).iter().for_each(|p| {
        if p.file_name().unwrap().to_string_lossy().ends_with(".rs") {
            return;
        }
        let css_text = read_file_as_string(p);
        let origin_file_name = p.file_name().unwrap();
        let file_name = format!(
            "{}_{}.fpcssb",
            get_current_commit(),
            origin_file_name.to_str().unwrap()
        );
        // generate bincode & write bincode source
        let current_bincode_path = current_dir.join("bincode_source").join(&file_name);
        let current_bincode =
            float_pigment_css::compile_style_sheet_to_bincode(&file_name, &css_text);
        write_bincode(&current_bincode_path, current_bincode);
    });
}

fn deserialize_bincode_source_test() {
    let current_dir =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests/compatibility");
    // test bincode source with current deserializer
    let bincode_source_path = current_dir.join("bincode_source");
    dir_files_path(&bincode_source_path).iter().for_each(|p| {
        let file_name = p.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".fpcssb") {
            let bincode = read_bincode(p);
            let mut ssg = StyleSheetGroup::new();
            let mut resource = StyleSheetResource::new();
            resource.add_bincode(file_name, bincode);
            ssg.append_from_resource(&resource, file_name, None);
            // drop commit hash & .fpcssb
            let file_name = unsafe { file_name.get_unchecked(9..file_name.len() - 7) };
            match file_name {
                "base.css" => base_css_assert(ssg),
                "style-v2.css" => style_v2_css_assert(ssg),
                "style.css" => style_css_assert(ssg),
                "padding_case" => padding_case_assert(ssg),
                _ => {
                    println!("file_name: {:?}\n {:?}\n", file_name, ssg.style_sheet(0))
                }
            }
        }
    });
}

#[test]
fn deserialize_bincode_test() {
    deserialize_bincode_source_test()
}

#[test]
#[ignore = "for temp test"]
fn deserialize_temp_bincode() {
    let current_dir =
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests/compatibility");
    let bincode_source_path = current_dir.join("temp_bincode");
    dir_files_path(&bincode_source_path).iter().for_each(|p| {
        let file_name = p.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".fpcssb") {
            let bincode = read_bincode(p);
            let mut ssg = StyleSheetGroup::new();
            let mut resource = StyleSheetResource::new();
            resource.add_bincode(file_name, bincode);
            ssg.append_from_resource(&resource, file_name, None);
            println!("file_name: {:?}\n {:?}\n", file_name, ssg.style_sheet(0))
        }
    });
}

fn main() {
    compile_bincode()
}

// #[test]
// fn test_iib() {
//     let current_dir =
//         Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("tests/compatibility");
//     let bincode = read_bincode(&current_dir.join("app.fpiib"));
//     let ii = StyleSheetImportIndex::deserialize_bincode(bincode);
//     println!("{:?}", ii);
// }
