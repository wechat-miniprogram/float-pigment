use std::io::prelude::*;
use std::process::Command;
use std::{fs::copy, io::BufReader, path::Path};

#[derive(Debug)]
struct BuildError {
    message: String,
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BuildError {}

impl From<String> for BuildError {
    fn from(x: String) -> Self {
        Self { message: x }
    }
}

impl From<&str> for BuildError {
    fn from(x: &str) -> Self {
        Self {
            message: x.to_string(),
        }
    }
}

fn main() -> Result<(), BuildError> {
    // env
    let cargo_manifest_dir = &std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_dir = Path::new(cargo_manifest_dir).join("..");
    let current_crate_dir = Path::new(cargo_manifest_dir);

    // gen layout binding
    Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("float_pigment_layout_cpp_binding_gen_tool")
        .arg("--features")
        .arg("build-cpp-header")
        .current_dir(&workspace_dir)
        .output()
        .expect("gen layout typings error");

    // copy layout binding
    let layout_target_path = current_crate_dir.join("float_pigment_layout.h");
    // let layout_source_path = workspace_dir.join("float-pigment-arena/float_pigment_layout.h");
    let layout_source_path = workspace_dir.join("float-pigment-forest/float_pigment_layout.h");
    copy(layout_source_path, layout_target_path).map_err(|_| "copy layout error")?;

    // gen css binding
    Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("float_pigment_css_cpp_binding_gen_tool")
        .arg("--features")
        .arg("build-cpp-header")
        .current_dir(&workspace_dir)
        .output()
        .expect("gen css typings error");

    // copy css binding
    let css_target_path = current_crate_dir.join("float_pigment_css.h");
    let css_source_path = workspace_dir.join("float-pigment-css/float_pigment_css.h");
    copy(css_source_path, &css_target_path).map_err(|_| "copy css error")?;

    // gen types
    {
        let typings_target_path = current_crate_dir.join("float_pigment_types.h");
        let css_target_file = std::fs::File::open(&css_target_path).unwrap();
        let reader = BufReader::new(css_target_file);
        let mut typings: Vec<String> = vec![];
        let mut end = false;
        for line in reader.lines() {
            let line = line.unwrap();
            match line.trim() {
                r#"extern "C" {"# => {
                    end = true;
                }
                r#"} // extern "C""# => {
                    end = false;
                }
                _ => {
                    if !end {
                        typings.push(line.replace(">>", "> >"));
                    }
                }
            }
        }
        let mut typings = typings.join("\n");
        typings.push('\n');
        let mut typings_target_file =
            std::fs::File::create(typings_target_path).map_err(|x| x.to_string())?;
        typings_target_file
            .write(typings.as_bytes())
            .map_err(|x| x.to_string())?;

        // clear css binding
        let css_target_file = std::fs::File::open(&css_target_path).unwrap();
        let reader = BufReader::new(css_target_file);
        let mut css_bindings: Vec<String> = vec![];
        let mut start = true;
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.trim();
            match line {
                r#"namespace float_pigment {"# => {
                    start = false;
                    css_bindings.push(line.to_string());
                }
                r#"extern "C" {"# => {
                    start = true;
                    css_bindings.push(line.to_string());
                }
                _ => {
                    if start {
                        css_bindings.push(line.replace(">>", "> >"));
                    }
                }
            }
        }
        let mut css_bindings = css_bindings.join("\n");
        css_bindings.push('\n');
        let mut css_target_file =
            std::fs::File::create(&css_target_path).map_err(|x| x.to_string())?;
        css_target_file
            .write(css_bindings.as_bytes())
            .map_err(|x| x.to_string())?;
    }
    Ok(())
}
