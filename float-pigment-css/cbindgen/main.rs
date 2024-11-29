// use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
// use std::io::Read;
use std::path::{Path, PathBuf};

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
        Self {
            message: x.to_string(),
        }
    }
}

impl From<&str> for BuildError {
    fn from(x: &str) -> Self {
        Self {
            message: x.to_string(),
        }
    }
}

fn cargo_expand(crate_dir: &Path, module_path: &str) -> Result<Vec<u8>, BuildError> {
    use std::process::Command;
    let output = Command::new("cargo")
        .arg("expand")
        .arg("--lib")
        .arg(module_path)
        .current_dir(crate_dir)
        .output()
        .expect("Failed to execute `cargo-expand`");
    if !output.status.success() {
        #[cfg(debug_assertions)]
        println!(
            "{}",
            std::str::from_utf8(&output.stdout).unwrap_or_default()
        );
        #[cfg(debug_assertions)]
        println!(
            "{}",
            std::str::from_utf8(&output.stderr).unwrap_or_default()
        );
        Err("`cargo-expand` returned failed. Aborted.")?;
    }
    Ok(output.stdout)
}

fn update_cbindgen_bindings(crate_dir: &Path) -> Result<(), BuildError> {
    let input_path = {
        let mut p = PathBuf::new();
        p.push(crate_dir);
        p.push("cbindgen");
        p.push("src");
        p
    };
    let output_path = {
        let mut p = PathBuf::new();
        p.push(crate_dir);
        p.push("float_pigment_css.h");
        p
    };

    // copy src files
    {
        let mut src_path = PathBuf::new();
        src_path.push(crate_dir);
        src_path.push("src");
        let copy_options = fs_extra::dir::CopyOptions {
            overwrite: true,
            content_only: true,
            ..Default::default()
        };
        fs_extra::dir::copy(&src_path, &input_path, &copy_options).unwrap();
    }

    // expand files with macro
    let mut property_expand_result = cargo_expand(crate_dir, "property")?;
    let typing_expand_result = cargo_expand(crate_dir, "typing")?;

    // inject extra file content
    let mut extra_content = {
        let mut extra_path = PathBuf::new();
        extra_path.push(crate_dir);
        extra_path.push("cbindgen");
        extra_path.push("extra.rs");
        std::fs::read(&extra_path).unwrap()
    };
    {
        let mut property_content = vec![];
        property_content.append(&mut extra_content);
        property_content.append(&mut property_expand_result);
        let mut property_path = PathBuf::new();
        property_path.push(&input_path);
        property_path.push("property.rs");
        std::fs::write(&property_path, property_content).unwrap();
    }
    {
        let mut typing_path = PathBuf::new();
        typing_path.push(&input_path);
        typing_path.push("typing.rs");
        std::fs::write(&typing_path, typing_expand_result).unwrap();
    }

    // read extra header
    let extra_header = {
        let mut extra_path = PathBuf::new();
        extra_path.push(crate_dir);
        extra_path.push("cbindgen");
        extra_path.push("extra_header.h");
        std::fs::read(&extra_path).unwrap()
    };

    // call cbindgen to generate
    {
        use cbindgen::{Builder, Config};
        let bindings = Builder::new()
            .with_config(Config::from_file(&{
                let mut p = PathBuf::new();
                p.push(crate_dir);
                p.push("cbindgen");
                p.push("cbindgen.toml");
                p
            })?)
            // .with_crate(crate_dir)
            .with_src(&{
                let mut p = PathBuf::new();
                p.push(input_path);
                p.push("lib.rs");
                p
            })
            .generate()
            .map_err(|x| x.to_string())?;
        let mut output_file = std::fs::File::create(&output_path).map_err(|x| x.to_string())?;
        // write extra header
        output_file
            .write(&extra_header)
            .map_err(|x| x.to_string())?;

        bindings.write(&mut output_file);

        // HACK Methods

        // read before insert
        let file = std::fs::File::open(&output_path).unwrap();

        let extra_struct = {
            let mut path = PathBuf::new();
            path.push(crate_dir);
            path.push("cbindgen");
            path.push("extra_struct.h");
            std::fs::read_to_string(&path).unwrap()
        };
        // // method 1. split and replace
        // let mut s = String::new();
        // file.read_to_string(&mut s);
        // let mut a: Vec<&str> = s.split("namespace pigment {\n").collect();
        // let insert_body =
        //     format!("namespace pigment {{\nstruct Length;\nstruct SelectorFragment;\n");
        // {
        //     a.insert(1, &insert_body);
        // }

        // method 2. read lines and replace
        let reader = BufReader::new(file);
        let mut a: Vec<String> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            match line.trim() {
                "namespace float_pigment {" => {
                    a.push("namespace float_pigment {".into());
                    a.push(extra_struct.clone());
                }
                _ => {
                    a.push(line.replace(">>", "> >"));
                }
            }
        }

        let mut s = a.join("\n").to_string();
        s.push('\n');
        // write after insert
        let mut file = std::fs::File::create(&output_path).unwrap();
        file.write_all(s.as_bytes()).expect("Write Error");
    }
    Ok(())
}

fn main() -> Result<(), BuildError> {
    if let Ok(crate_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        update_cbindgen_bindings(Path::new(&crate_dir))?;
    } else {
        Err("CARGO_MANIFEST_DIR is not set. Skipped cbindgen step.")?;
    }
    Ok(())
}
