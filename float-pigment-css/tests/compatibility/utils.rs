use float_pigment_css::length_num::LengthNum;
use float_pigment_css::StyleSheetGroup;
use float_pigment_css::{property::*, MediaQueryStatus, StyleQuery};
use std::process::Command;
use std::{
    fs::OpenOptions,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

#[allow(dead_code)]
pub fn query<const N: usize, const M: usize>(
    ssg: &StyleSheetGroup,
    tag_name: &str,
    id: &str,
    classes: [&str; N],
    attributes: [&str; M],
) -> NodeProperties {
    query_with_media(
        ssg,
        tag_name,
        id,
        classes,
        attributes,
        &MediaQueryStatus::<f32>::default_screen(),
    )
}

#[allow(dead_code)]
pub fn query_with_media<L: LengthNum, const N: usize, const M: usize>(
    ssg: &StyleSheetGroup,
    tag_name: &str,
    id: &str,
    classes: [&str; N],
    attributes: [&str; M],
    media_query_status: &MediaQueryStatus<L>,
) -> NodeProperties {
    let classes = classes
        .iter()
        .map(|x| (x.to_string(), None))
        .collect::<Vec<_>>();
    let attributes = attributes.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let query = StyleQuery::single(None, None, None, tag_name, id, &classes, &attributes);
    let mut node_properties = NodeProperties::new(None);
    ssg.query_single(&query, media_query_status, &mut node_properties);
    node_properties
}

pub fn dir_files_path(path: &PathBuf) -> Vec<PathBuf> {
    let paths = std::fs::read_dir(path).unwrap();
    paths.map(|x| x.unwrap().path()).collect()
}

fn create_file(path: &PathBuf) -> Result<std::fs::File, std::io::Error> {
    let mut options = OpenOptions::new();
    let file = options
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path);
    file
}

pub fn read_file_as_string(path: &PathBuf) -> String {
    let mut options = OpenOptions::new();
    let file = options
        .read(true)
        .open(path)
        .unwrap_or_else(|_| panic!("open file error. file path: {:?}", path.to_str()));
    let mut reader = BufReader::new(file);
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();
    string
}

pub fn read_bincode(path: &PathBuf) -> Vec<u8> {
    std::fs::read(path).unwrap_or_else(|_| panic!("{:?} is not exists.", path.to_str()))
}

pub fn write_bincode(path: &PathBuf, bincode: Vec<u8>) {
    let mut file = create_file(path)
        .unwrap_or_else(|_| panic!("create file error. file path: {:?}", path.to_str()));
    file.write_all(bincode.as_slice())
        .unwrap_or_else(|_| panic!("write bincode error. file path: {:?}", path.to_str()))
}

pub fn get_current_commit() -> String {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .expect("get current git commit error.");
    let ret = String::from_utf8_lossy(&output.stdout).to_string();
    let ret = ret.replace("\n", "").replace("\"", "");
    let ret: String = ret.trim().into();
    ret[..8].into()
}
