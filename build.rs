use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_server(true)
        .out_dir("src/server")
        .file_descriptor_set_path(out_dir.join("yelp.bin"))
        .compile_protos(&["proto/yelp.proto"], &["proto"])
        .unwrap();

    println!("cargo:rerun-if-changed=proto/business.proto");
}
