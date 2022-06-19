use std::{env, fs, io, path, process};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = path::PathBuf::from(&out_dir);
    // on windows use mc.exe and rc.exe to compile the message text files and link those
    // on linux use windmc and windres to compile the message text files and link those
    compile_message_files(&out_dir);

    // Replace explicit type definitions in header file so bindgen can read it
    let header =
        fs::read_to_string(out_path.join("messages.h")).expect("Couldn't read header file");
    fs::write(out_path.join("messages.h"), header.replace("(DWORD)", ""))
        .expect("Couldn't write header file");

    let bindings = bindgen::Builder::default()
        .header(format!("{}\\messages.h", out_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(&["-x", "c++"])
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file(out_path.join("messages.rs"))
        .expect("Couldn't write bindings");

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=dylib=messages");
}

const RESOURCE_DIR: &str = "resources/";

fn list_message_files(dir: &str) -> Vec<String> {
    let files = fs::read_dir(dir).unwrap();

    files.filter_map(is_message_file).collect()
}

fn is_message_file(file: io::Result<fs::DirEntry>) -> Option<String> {
    let path = file.unwrap().path();
    if let Some(extension) = path.extension() {
        if extension == "mc" {
            Some(path.to_string_lossy().to_string())
        } else {
            None
        }
    } else {
        None
    }
}

#[cfg(windows)]
fn compile_message_files(out_dir: &str) {
    let message_files = list_message_files(RESOURCE_DIR);
    process::Command::new("mc.exe")
        .args(["-h", out_dir, "-r", out_dir])
        .args(message_files)
        .output()
        .expect("Failed to compile message files");

    process::Command::new("rc.exe")
        .args([
            "/fo",
            &format!("{}/{}", out_dir, "messages.lib"),
            &format!("{}/{}", out_dir, "messages.rc"),
        ])
        .output()
        .expect("Failed to generate resource files");
}

#[cfg(not(windows))]
fn compile_message_files(out_dir: &str) {
    let message_files = list_message_files(RESOURCE_DIR);

    process::Command::new("windmc")
        .args(["-h", out_dir, "-r", out_dir])
        .args(message_files)
        .output()
        .expect("Failed to compile message files");

    process::Command::new("windres")
        .args([
            "-i",
            &format!("{}/{}", out_dir, "messages.rc"),
            "-o",
            &format!("{}/{}", out_dir, "messages.lib"),
        ])
        .output()
        .expect("Failed to generate resource files");
}
