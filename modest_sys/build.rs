extern crate gcc;

use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;
use std::io::ErrorKind;

fn main() {
    let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    if !Path::new("Modest/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=modest");
    println!("cargo:root={}", dst.display());
    println!("cargo:include=Modest/include/modest");

    let cfg = gcc::Build::new();
    let compiler = cfg.get_compiler();

    let cmd_make = if cfg!(target_os = "freebsd") { "gmake" } else { "make" };
    let mut cmd = Command::new(cmd_make);

    let mut cflags = OsString::new();
    for arg in compiler.args() {
        cflags.push(arg);
        cflags.push(" ");
    }
    cflags.push("-Wno-unused-parameter -Wno-missing-field-initializers -Wno-sign-compare");

    cmd.env("CC", compiler.path())
        .env("CFLAGS", cflags)
        .env("LD", &which("ld").unwrap())
        .current_dir("Modest");
    if cfg!(target_os = "macos") {
        cmd.env("MODEST_BUILD_OBJECT_STATIC", "libtool -static -o $2 $1");
    }

    match &env::var("PROFILE").unwrap()[..] {
        "bench" | "release" => {
            cmd.env("MyCORE_BUILD_DEBUG", "NO");
            cmd.env("PROJECT_OPTIMIZATION_LEVEL", "-O2");
        }
        _ => {
            cmd.env("MyCORE_BUILD_DEBUG", "YES");
        }
    }

    cmd.arg("static");
    run(&mut cmd, "make");

    fs::copy("Modest/lib/libmodest_static.a", &dst.join("libmodest.a")).unwrap();
    fs::remove_dir_all("Modest/lib").unwrap();

    let mut cmd = Command::new(cmd_make);
    cmd.arg("clean");
}

fn run(cmd: &mut Command, program: &str) {
    println!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            fail(&format!("failed to execute command: {}\nis `{}` not installed?",
                          e, program));
        }
        Err(e) => fail(&format!("failed to execute command: {}", e)),
    };
    if !status.success() {
        fail(&format!("command did not execute successfully, got: {}", status));
    }
}

fn fail(s: &str) -> ! {
    panic!("\n{}\n\nbuild script failed, must exit now", s)
}

fn which(cmd: &str) -> Option<PathBuf> {
    let cmd = format!("{}{}", cmd, env::consts::EXE_SUFFIX);
    let paths = env::var_os("PATH").unwrap();
    env::split_paths(&paths).map(|p| p.join(&cmd)).find(|p| {
        fs::metadata(p).is_ok()
    })
}
