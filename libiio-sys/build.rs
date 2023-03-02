// libiio-sys/build.rs
//
// The builder for the Linux Industrial I/O wrapper crate.
//
// Copyright (c) 2018-2022, Frank Pagliughi
//
// Licensed under the MIT license:
//   <LICENSE or http://opensource.org/licenses/MIT>
// This file may not be copied, modified, or distributed except according
// to those terms.
//

use std::env;

#[cfg(target_os = "macos")]
fn config_macos() {
    println!("cargo:rustc-link-lib=framework=iio");

    if cfg!(target_arch = "x86_64") {
        println!(r"cargo:rustc-link-search=framework=/usr/local/Frameworks/");
    }
    else {
        println!(r"cargo:rustc-link-search=framework=/opt/homebrew/Frameworks/");
    }
}

fn main() {
    // TODO: We should eventually find or regenerate the
    //      bindings file for the specific target.
    let tgt = env::var("TARGET").unwrap();
    println!("debug: Building for target: '{}'", tgt);

    let library_name = env::var("LIBIIO_LIBRARY_NAME").unwrap_or("iio".into());
    println!("debug: Building with name: '{}'", library_name);

    if let Ok(library_path) = env::var("LIBIIO_LIBRARY_PATH"){
        println!("debug: Building with path: '{}'", library_path);
        println!(r"cargo:rustc-link-search={}", library_path);
    }

    if let Ok(raw) = env::var("LIBIIO_EXTEND_LINK_LIBS"){
        let extend_link_libs = raw.split(",");
        for lib in extend_link_libs{
            println!("cargo:rustc-link-lib={}", lib);
        }
    }

    if env::var("LIBIIO_LINK_PTHREAD").is_ok() {
        println!("cargo:rustc-link-arg=-pthread");
    }

    #[cfg(feature = "libiio_v0_24")]
    println!("debug: Using bindings for libiio v0.24");

    #[cfg(feature = "libiio_v0_21")]
    println!("debug: Using bindings for libiio v0.21");

    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib={}", library_name);

    #[cfg(target_os = "macos")]
    config_macos();
}