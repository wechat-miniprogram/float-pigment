use colorful::Color;
use colorful::Colorful;

use std::env;
use std::{path::PathBuf, process::Command};

const PUBLISH_BRANCH: &str = "master";

fn get_root_path() -> String {
    let mut pb = PathBuf::new();
    pb.push(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    pb.push("../");
    pb.to_str().unwrap().to_string()
}

fn generate_c_header() {
    println!("{}", "generate C-header: waiting...".color(Color::Yellow));
    let root = get_root_path();
    let _ret = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("float_pigment_css_cpp_binding_gen_tool")
        .arg("--features")
        .arg("build-cpp-header")
        .current_dir(root)
        .output()
        .expect("failed to generate C Header.");
    println!("{}", "generate C-header: done!".color(Color::Green))
}

struct GitCommand {
    pub logger: bool,
}

impl GitCommand {
    fn new() -> Self {
        Self { logger: false }
    }
    fn set_logger(&mut self, v: bool) {
        self.logger = v;
    }
    fn git_branch_checker(&self, target_branch: &str) {
        let output = Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .output()
            .expect("git branch checker error");
        let ret = String::from_utf8_lossy(&output.stdout).to_string();
        let ver = ret.replace('\n', "");
        println!("current branch: {}", ver.clone().color(Color::Green));
        if ver != target_branch {
            println!(
                "{}",
                format!(
                    "publish must on branch {}! current branch: {}",
                    target_branch, ver
                )
                .color(Color::Red)
            );
            panic!();
        }
    }

    fn git_commit(&self, commit: &str) {
        println!("{}", "git commit: waiting...".color(Color::Yellow));
        let root = get_root_path();
        let ret = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(commit)
            .current_dir(root)
            .output()
            .expect("failed to git commit");
        if self.logger {
            println!(
                "{}",
                String::from_utf8(ret.stdout).unwrap().color(Color::Yellow)
            );
        }
        println!("{}", "git commit: done!".color(Color::Green));
    }

    fn git_add(&self) {
        println!("{}", "git add: waiting...".color(Color::Yellow));
        let root = get_root_path();
        let ret = Command::new("git")
            .arg("add")
            .arg(".")
            .current_dir(root)
            .output()
            .expect("failed to git add .");
        if self.logger {
            println!(
                "{}",
                String::from_utf8(ret.stdout).unwrap().color(Color::Yellow)
            );
        }
        println!("{}", "git add: done!".color(Color::Green));
    }

    fn git_tag(&self, tag: &str) {
        println!("{}", "git tag: waiting...".color(Color::Green));
        let root = get_root_path();
        let ret = Command::new("git")
            .arg("tag")
            .arg(tag)
            .current_dir(root)
            .output()
            .expect("failed to git tag");
        if self.logger {
            println!(
                "{}",
                String::from_utf8(ret.stdout).unwrap().color(Color::Yellow)
            );
        }
        println!("{}", format!("tag: {}", tag).color(Color::Red));
        println!("{}", "git tag: done!".color(Color::Green));
    }

    fn git_push(&self, tag: &str) {
        println!(
            "[{}] {}",
            tag.color(Color::Red),
            "git push: waiting...".color(Color::Yellow)
        );
        let root = get_root_path();
        // tag
        let ret = Command::new("git")
            .arg("push")
            .arg("--set-upstream")
            .arg("origin")
            .arg(tag)
            .current_dir(root.clone())
            .output()
            .expect("failed to git push");
        if self.logger {
            println!(
                "{}",
                String::from_utf8(ret.stdout).unwrap().color(Color::Yellow)
            );
        }
        println!(
            "[{}] {}",
            tag.color(Color::Red),
            "git push: done!".color(Color::Green)
        );
        println!(
            "[{}] {}",
            PUBLISH_BRANCH.color(Color::Red),
            "git push: waiting...".color(Color::Yellow)
        );
        // publish branch
        let ret = Command::new("git")
            .arg("push")
            .arg("--set-upstream")
            .arg("origin")
            .arg(PUBLISH_BRANCH)
            .current_dir(root)
            .output()
            .expect("failed to git push");
        if self.logger {
            println!(
                "{}",
                String::from_utf8(ret.stdout).unwrap().color(Color::Yellow)
            );
        }
        println!(
            "[{}] {}",
            PUBLISH_BRANCH.color(Color::Red),
            "git push: done!".color(Color::Green)
        );
    }
}

fn main() {
    // generate C-Header
    #[allow(clippy::needless_collect)]
    let args: Vec<String> = env::args().collect();
    let mut git_cmd = GitCommand::new();
    if args.contains(&"--logger".to_string()) {
        git_cmd.set_logger(true);
    }
    git_cmd.git_branch_checker(PUBLISH_BRANCH);
    generate_c_header();
    git_cmd.git_add();
    let tag = format!("v{:?}", std::env::var("CARGO_PKG_VERSION").unwrap());
    let tag = tag.replace('"', "");
    git_cmd.git_commit(&tag);
    git_cmd.git_tag(&tag);
    git_cmd.git_push(&tag);
}
