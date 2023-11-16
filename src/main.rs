use std::process::Command;
use std::ffi::{OsString, OsStr};
use std::path::PathBuf;

fn main() {
    let mut args = std::env::args_os();

    if args.len() > 2 {
        panic!("Error, cmaek take at most one argument : the path of the root of the project");
    }

    let project_root = PathBuf::from(args.nth(1).unwrap_or(".".into()));
//    std::fs::canonicalize(args.nth(1).unwrap())
//        .expect("Failed to find absolute path of the project root.");

    let src_dir = project_root.join("src");
    let build_dir = project_root.join("build");
    let lib_dir = project_root.join("lib");
    let inc_dir = project_root.join("inc");

    if build_dir.exists() {
        if build_dir.is_file() {
            panic!("build should be a directory, not a file !");
        }
    } else {
        std::fs::create_dir(&build_dir).expect("Failed to create build directory.");
    }

    #[cfg(target_os = "windows")]
    let mut command = Command::new("g++.exe");
    #[cfg(target_os = "linux")]
    let mut command = Command::new("g++");

    let comp = command
//        .current_dir(build_dir.canonicalize().expect("Could not find the absolute path of build directory"))
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .args([
            OsString::from("-o"),
            OsString::from(
                build_dir.join(project_root
                    .canonicalize()
                    .expect("Can't find absolute path of build directory.")
                    .file_name()
                    .expect("Pas de nom"))),
            OsString::from("-fmodules-ts"),
            OsString::from("-std=c++20"),
            OsString::from("-I"), OsString::from(inc_dir),
            OsString::from("-I"), OsString::from(lib_dir)
        ]
    );

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
}
