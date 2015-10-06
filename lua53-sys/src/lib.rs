#![allow(non_camel_case_types)]
extern crate libc;

#[repr(C)]
pub struct lua_State;

pub type lua_Number = f64;
pub type lua_Integer = i64;
pub type lua_Unsigned = u64;
pub type lua_KContext = isize;
pub type lua_CFunction = Option<extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_KFunction = Option<extern "C" fn(*mut lua_State, libc::c_int, lua_KContext) -> *mut libc::c_int>;
pub type lua_Reader = Option<extern "C" fn(*mut lua_State, *mut libc::c_void, *mut libc::size_t) -> *const libc::c_char>;
pub type lua_Writer = Option<extern "C" fn(*mut lua_State, *const libc::c_void, libc::size_t, *mut libc::c_void) -> libc::c_int>;


// TODO lua_Alloc
// TODO lua_Debug
// TODO lua_Hook
// TODO lua_Writer

include!(concat!(env!("OUT_DIR"), "/lua.rs"));
include!(concat!(env!("OUT_DIR"), "/lauxlib.rs"));
include!(concat!(env!("OUT_DIR"), "/lualib.rs"));


pub const LUA_TNONE          : libc::c_int = -1;
pub const LUA_TNIL           : libc::c_int = 0;
pub const LUA_TBOOLEAN       : libc::c_int = 1;
pub const LUA_TLIGHTUSERDATA : libc::c_int = 2;
pub const LUA_TNUMBER        : libc::c_int = 3;
pub const LUA_TSTRING        : libc::c_int = 4;
pub const LUA_TTABLE         : libc::c_int = 5;
pub const LUA_TFUNCTION      : libc::c_int = 6;
pub const LUA_TUSERDATA      : libc::c_int = 7;
pub const LUA_TTHREAD        : libc::c_int = 8;
pub const LUA_NUMTAGS        : libc::c_int = 9;
