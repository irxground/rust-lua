extern crate libc;

#[repr(C)]
pub struct lua_State;

#[allow(non_camel_case_types)]
pub type lua_Number = f64;

#[allow(non_camel_case_types)]
pub type lua_KContext = isize;

#[allow(non_camel_case_types)]
pub type lua_KFunction = isize;

include!(concat!(env!("OUT_DIR"), "/lua.rs"));
include!(concat!(env!("OUT_DIR"), "/lauxlib.rs"));
