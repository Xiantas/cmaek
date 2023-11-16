use std::process::Command;
use std::ffi::{OsString, OsStr};

fn main() {
    let mut args = std::env::args_os();

    if args.len() != 2 {
        panic!("Error, cmaek take only one argument : the path of the root of the project");
    }

    let project_root = std::fs::canonicalize(args.nth(1).unwrap())
        .expect("Failed to find absolute path of the project root.");

    let src_dir = {
        let mut temp = project_root.clone();
        temp.push("src");
        temp
    };

    let mut command = Command::new("g++");
    let comp = command
        .args([
            OsString::from("-o"), OsString::from({
                let mut temp = project_root.clone();
                temp.push(project_root.file_name().unwrap());
                temp
            })
        ]);

    let cpps = std::fs::read_dir(src_dir)
        .expect("No src found");

    for cpp in cpps {
        let cpp = cpp.unwrap().path();
        if cpp.extension().map(|e| e.to_str()) == Some(Some("cpp")) {
            comp.arg(cpp);
        }
    }

    comp.output()
        .expect("Unable to find g++");

/*
    Command::new("g++")
        .arg("test.cpp")
        .arg("-o")
        .arg("test")
        .output()
        .expect("Unable to g++");
*/
}
