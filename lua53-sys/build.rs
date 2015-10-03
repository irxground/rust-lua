extern crate glob;
extern crate gcc;

use std::result::Result;

fn main() {
    let mut conf = gcc::Config::new();

    for path in glob::glob("../lua/src/*.c").unwrap().filter_map(Result::ok) {
        if path.ends_with("lua.c") || path.ends_with("luac.c") {
            continue
        }
        conf.file(path);
    }
    conf.compile("liblua.a");
}
