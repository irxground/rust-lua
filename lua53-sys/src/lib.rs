extern crate libc;

use libc::{c_char, c_int};
// use std::ffi::CString;

#[repr(C)]
pub struct lua_State;

#[allow(non_camel_case_types)]
pub type lua_Number = f64;
#[allow(non_camel_case_types)]
pub type lua_KContext = isize;
#[allow(non_camel_case_types)]
pub type lua_KFunction = isize;

#[allow(improper_ctypes)]
extern {
    pub fn luaL_newstate() -> *mut lua_State;
    pub fn lua_close(L: *mut lua_State);

    pub fn luaL_loadstring(L: *mut lua_State, s: * const c_char) -> i32;
    pub fn lua_setglobal(L: *mut lua_State, name: * const c_char);
    pub fn lua_getglobal(L: *mut lua_State, name: * const c_char) -> bool;
    pub fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    pub fn lua_tonumberx(L: *mut lua_State, idx: c_int, isnum: *mut c_int) -> lua_Number;

    pub fn lua_pcallk(L: *mut lua_State, nargs: c_int, nresults: c_int, errfunc: c_int, ctx: lua_KContext, k: lua_KFunction);
}
