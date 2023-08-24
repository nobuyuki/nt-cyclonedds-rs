use std::fs::canonicalize;
use std::path::{Path, PathBuf};
use std::process::Command;

const BUILD_IN_TMP_DIR: bool = false;

// "libcycloneddsidl.0.11.0.dylib"
// "libcycloneddsidl.0.dylib"
// "libcycloneddsidl.dylib"
// "libcycloneddsidlc.0.11.0.dylib"
// "libcycloneddsidlc.0.dylib"
// "libcycloneddsidlc.dylib"
// "libdds_security_ac.dylib"
// "libdds_security_auth.dylib"
// "libdds_security_crypto.dylib"
// "libddsc.0.11.0.dylib"
// "libddsc.0.dylib"
// "libddsc.dylib"
// "libiceoryx_binding_c.a"
// "libiceoryx_dust.a"
// "libiceoryx_hoofs.a"
// "libiceoryx_platform.a"
// "libiceoryx_posh.a"
// "libiceoryx_posh_config.a"
// "libiceoryx_posh_gateway.a"
// "libiceoryx_posh_roudi.a"

//"pam"
//"pkgconfig"

fn run(command: &mut Command) {
    println!("Running: {:?}", command);
    match command.status() {
        Ok(status) => {
            if !status.success() {
                panic!("`{:?}` failed: {}", command, status);
            }
        }
        Err(error) => {
            panic!("failed to execute `{:?}`: {}", command, error);
        }
    }
}

fn is_feature_enabled(f: &str) -> bool {
    let varname = format!("CARGO_FEATURE_{}", f.to_uppercase());

    if let Ok(val) = std::env::var(varname) {
        let val: String = val.to_uppercase();

        match val.as_str() {
            "1" => true,
            _ => false,
        }
    } else {
        false
    }
}

const FEATURES: [(&str, &str); 13] = [
    ("serucity", "-DENABLE_SECURITY"),
    // -DBUILD_EXAMPLES=ON
    // -DBUILD_TESTING=ON
    ("idlc", "-DBUILD_IDLC"),
    ("ddsperf", "-DBUILD_DDSPERF"),
    ("ssl", "-DENABLE_SSL"),
    ("shm", "-DENABLE_SHM"),
    ("security", "-DENABLE_SECURITY"),
    ("lifespan", "-DENABLE_LIFESPAN"),
    ("deadline_missed", "-DENABLE_DEADLINE_MISSED"),
    ("typelib", "-DENABLE_TYPELIB"),
    ("type_discovery", "-DENABLE_TYPE_DISCOVERY"),
    ("topic_discovery", "-DENABLE_TOPIC_DISCOVERY"),
    (
        "source_specific_multicast",
        "-DENABLE_SOURCE_SPECIFIC_MULTICAST",
    ),
    ("ipv6", "-DENABLE_IPV6"),
    // -DBUILD_IDLC_XTESTS=NO
];

fn bulid_cmake_options() -> String {
    let mut s = "".to_string();

    for (f, d) in &FEATURES {
        s.push_str(d);
        s.push_str(if is_feature_enabled(f) { "=ON" } else { "=NO" });
        s.push_str(" ");
    }

    let _ = s.pop();

    s
}

fn build_clib(src_dir: &Path, tmp_dir: &Path, out_dir: &Path) {
    run(&mut Command::new(
        canonicalize(src_dir.join("build-lib.sh")).unwrap(),
    ));
}

fn build_unix(src_dir: &Path, tmp_dir: &Path, out_dir: &Path) {
    //    if !(out_dir
    //        .join("lib")
    //        .join("libkvsWebrtcSignalingClient.a")
    //        .exists()
    //        & out_dir.join("lib").join("libkvsWebrtcClient.a").exists())
    //    {
    build_clib(src_dir, tmp_dir, out_dir);
    //    }
}

fn main() {
    let src_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let tmp_dir = if BUILD_IN_TMP_DIR {
        std::env::set_var("IN_TMP", "true");
        src_dir.join("tmp")
    } else {
        out_dir.clone()
    };

    // Windows not supported, at least, now.
    build_unix(&src_dir, &tmp_dir, &out_dir);

    println!("cargo:rustc-link-search={}", out_dir.display());

    //  println!("cargo:rustc-link-search=/opt/homebrew/Cellar/openssl@1.1/1.1.1q/lib"); // TODO
    //    println!("cargo:rustc-link-search=/opt/homebrew/opt/openssl@1.1/lib"); // TODO

    // Local SSL stuffs.
    //    println!("cargo:rustc-link-lib=static=ssl");
    //    println!("cargo:rustc-link-lib=static=crypto");
    // TODO: Proper handling of pkg_config.

    println!("nobuyuki:{}", bulid_cmake_options());

    //    println!("cargo:rustc-link-search=/path/to/lib");
    //    println!("cargo:rustc-link-lib=bz2");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}/usr/local/include", tmp_dir.display()))
        .clang_arg(format!(
            "-I{}/usr/local/include/iceoryx/v2.90.0/",
            tmp_dir.display()
        ))
        .clang_arg(format!(
            "-I{}/iceoryx/iceoryx_binding_c/include",
            tmp_dir.display()
        ))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
