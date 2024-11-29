use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;

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

fn update_layout_bindings(crate_dir: &Path, css_typings: Option<String>) -> Result<(), BuildError> {
    let cbindgen_src = {
        let mut p = PathBuf::new();
        p.push(crate_dir);
        p.push("cbindgen");
        p.push("src");
        p
    };

    let layout_header = {
        let mut p = PathBuf::new();
        p.push(crate_dir);
        p.push("float_pigment_layout.h");
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
        fs_extra::dir::copy(&src_path, &cbindgen_src, &copy_options).unwrap();
    }

    // expand files with macro
    let typing_expand_result = cargo_expand(crate_dir, "ffi")?;
    let mut ffi_path = PathBuf::new();
    ffi_path.push(&cbindgen_src);
    ffi_path.push("ffi.rs");
    std::fs::write(&ffi_path, typing_expand_result).unwrap();

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
            .with_src(&{
                let mut p = PathBuf::new();
                p.push(cbindgen_src);
                p.push("lib.rs");
                p
            })
            .generate()
            .map_err(|x| x.to_string())?;
        let mut output_file = std::fs::File::create(&layout_header).map_err(|x| x.to_string())?;
        // write extra header
        output_file
            .write(&extra_header)
            .map_err(|x| x.to_string())?;
        bindings.write(&mut output_file);

        // HACK Methods

        // read before insert
        let file = std::fs::File::open(&layout_header).unwrap();

        let extra_struct = {
            let mut path = PathBuf::new();
            path.push(crate_dir);
            path.push("cbindgen");
            path.push("extra_struct.h");
            std::fs::read_to_string(&path).unwrap()
        };

        let reader = BufReader::new(file);
        let mut a: Vec<String> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            match line.trim() {
                "namespace float_pigment {" => {
                    a.push("namespace float_pigment {".into());
                    a.push(extra_struct.clone());
                    if let Some(css_typings) = css_typings.clone() {
                        a.push(css_typings);
                    }
                }
                _ => {
                    a.push(line.replace(">>", "> >"));
                }
            }
        }

        let mut s = a.join("\n").to_string();
        s.push('\n');
        // write after insert
        let mut file = std::fs::File::create(&layout_header).unwrap();
        file.write_all(s.as_bytes()).expect("Write Error");
    }
    Ok(())
}

fn main() -> Result<(), BuildError> {
    if let Ok(crate_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        // step 1. update css typing
        // let css_typings = update_css_typings(Path::new(&crate_dir))?;
        // step 2. update layout binding
        // update_layout_bindings(Path::new(&crate_dir), Some(css_typings))?;
        update_layout_bindings(Path::new(&crate_dir), None)?;
    } else {
        Err("CARGO_MANIFEST_DIR is not set. Skipped cbindgen step.")?;
    }
    Ok(())
}
