extern crate glob;
extern crate gcc;
extern crate regex;

use std::result::Result;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::Write;
use std::path::Path;

macro_rules! get {
    ($expr:expr) => (match $expr {
        Some(val) => val,
        None => return None
    })
}

fn build_native() {
    let mut conf = gcc::Config::new();

    for path in glob::glob("../lua/src/*.c").unwrap().filter_map(Result::ok) {
        if path.ends_with("lua.c") || path.ends_with("luac.c") {
            continue
        }
        conf.file(path);
    }
    conf.compile("liblua.a");
}

fn avoid_keyword(name: &str) -> &str {
    match name {
        "ref" => "ref_",
        _ => name
    }
}

fn to_ctype(token: &str) -> Option<&str> {
    let defined_type = vec![
        "lua_State",
        "lua_Number",
        "lua_Integer",
        "lua_KContext",
        "lua_KFunction",
    ];
    if defined_type.iter().any(|t| *t == token ) {
        return Some(token);
    }
    match token {
        "int" => Some("libc::c_int"),
        "char" => Some("libc::c_char"),
        "size_t" => Some("libc::size_t"),
        _ => None
    }
}

fn to_rust_type(token: &str) -> Option<String> {
    let token = token.trim();
    let re_simple = regex::Regex::new(r"^[:word:]+$").unwrap();
    let re_ptr = regex::Regex::new(r"^([:word:]+)\s*\*$").unwrap();
    let re_const_ptr = regex::Regex::new(r"^const ([:word:]+)\s*\*$").unwrap();
    if let Some(cap) = re_simple.captures(token) {
        let t = get!{ to_ctype(cap.at(0).unwrap()) };
        return Some(format!("{}", t));
    }
    if let Some(cap) = re_ptr.captures(token) {
        let t = get!{ to_ctype(cap.at(1).unwrap()) };
        return Some(format!("*mut {}", t));
    }
    if let Some(cap) = re_const_ptr.captures(token) {
        let t = get!{ to_ctype(cap.at(1).unwrap()) };
        return Some(format!("*const {}", t));
    }
    return None;
}

fn parse_args(c_arg_str: &str) -> Option<String> {
    if c_arg_str == "void" {
        return Some(format!(""));
    }
    let re = regex::Regex::new(r"^\s*(.+[:^word:])([:word:]+)\s*$").unwrap();
    let mut args = Vec::<String>::new();
    for token in c_arg_str.split(",") {
        if let Some(cap) = re.captures(token) {
            let name = avoid_keyword(cap.at(2).unwrap());
            let type_name = get!{ to_rust_type(cap.at(1).unwrap().trim()) };
            args.push(format!("{}: {}", name, type_name));
        }
    }
    return Some(args.join(", "));
}

fn to_rust_style(str: String) -> String {
    let re = regex::Regex::new(r"^[:word:]+ (.+)\(([:word:]+)\)\s*\((.+)\);$").unwrap();
    if let Some(cap) = re.captures(str.as_ref()) {
        let ret_type = cap.at(1).unwrap().trim();
        let fn_name = cap.at(2).unwrap();
        if let Some(args) = parse_args(cap.at(3).unwrap()) {
            if ret_type == "void" {
                return format!("pub fn {} ({});", fn_name, args);
            }
            if let Some(ret_type) = to_rust_type(ret_type) {
                return format!("pub fn {} ({}) -> {};", fn_name, args, ret_type);
            }
        }
    }
    return format!("// PARSE ERR! {}", str);
}

fn gen_definition(api_prefix: &str, src_file: &str, dst_file: &str) {
    let src = File::open(Path::new("../lua/src").join(src_file)).unwrap();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join(dst_file);
    let mut dst = File::create(&dest_path).unwrap();

    dst.write_all("#[allow(improper_ctypes)]\n".as_bytes()).unwrap();
    dst.write_all("extern {\n".as_bytes()).unwrap();

    let file = BufReader::new(&src);
    let mut last_line: Option<String> = None;
    for line in file.lines().filter_map(Result::ok) {
        let mut next_last_line = None;
        if let Some(x) = last_line {
            let line = line.trim();
            let current_line = x + " " + line.as_ref();

            if line.ends_with(";") {
                let str = to_rust_style(current_line);
                dst.write_all(str.as_bytes()).unwrap();
                dst.write_all("\n".as_bytes()).unwrap();
            } else {
                next_last_line = Some(current_line);
            }
        }
        else if line.starts_with(api_prefix) {
            if line.ends_with(";") {
                let str = to_rust_style(line);
                dst.write_all(str.as_bytes()).unwrap();
                dst.write_all("\n".as_bytes()).unwrap();
            } else {
                next_last_line = Some(line);
            }
        }
        last_line = next_last_line;
    }
    dst.write_all("}\n".as_bytes()).unwrap();
    assert_eq!(None, last_line);
}

fn main() {
    build_native();
    gen_definition("LUA_API", "lua.h", "lua.rs");
    gen_definition("LUALIB_API", "lauxlib.h", "lauxlib.rs");
    gen_definition("LUAMOD_API", "lualib.h", "lualib.rs");
}
