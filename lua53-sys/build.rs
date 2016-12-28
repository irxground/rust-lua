extern crate libbindgen;
extern crate gcc;
extern crate glob;

use std::env;
use std::path::Path;

fn gen_header() {
  let out_dir = env::var("OUT_DIR").unwrap();
  let _ = libbindgen::builder()
    .header("wrapper.h")
    .no_unstable_rust()
    // .use_core()
    .generate().unwrap()
    .write_to_file(Path::new(&out_dir).join("lua.rs"));
}

fn gen_binary() {
    let mut conf = gcc::Config::new();

    for path in glob::glob("../lua/*.c").unwrap().filter_map(Result::ok) {
        if path.ends_with("lua.c") || path.ends_with("luac.c") {
            continue
        }
        conf.file(path);
    }
    conf.compile("liblua.a");
}

fn main() {
    gen_header();
    gen_binary();
}
