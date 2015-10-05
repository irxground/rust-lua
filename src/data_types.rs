extern crate libc;
extern crate lua53_sys as luac;

use Lua;
use std::slice;
use libc::{ c_char, size_t };

pub trait Read {
    fn read(lua: &mut Lua, idx: i32) -> Option<Self>;
}

pub trait Write {
    fn write_top(&self, lua: &mut Lua);
}

impl Write for () {
    fn write_top(&self, lua: &mut Lua) {
        unsafe { luac::lua_pushnil(lua.ptr) };
    }
}

impl Read for bool {
    fn read(lua: &mut Lua, idx: i32) -> Option<bool> {
        Some(unsafe { luac::lua_toboolean(lua.ptr, idx) != 0 })
    }
}

impl Write for bool {
    fn write_top(&self, lua: &mut Lua) {
        unsafe { luac::lua_pushboolean(lua.ptr, if *self { 1 } else { 0 }) };
    }
}

impl Read for f64 {
    fn read(lua: &mut Lua, idx: i32) -> Option<f64> {
        let mut isnum = 0;
        let lua_number = unsafe { luac::lua_tonumberx(lua.ptr, idx, &mut isnum) };
        if isnum == 0 {
            return None;
        }
        return Some(lua_number);
    }
}

impl Write for f64 {
    fn write_top(&self, lua: &mut Lua) {
        unsafe { luac::lua_pushnumber(lua.ptr, *self) };
    }
}

impl Read for i64 {
    fn read(lua: &mut Lua, idx: i32) -> Option<i64> {
        let mut isnum = 0;
        let lua_integer = unsafe { luac::lua_tointegerx(lua.ptr, idx, &mut isnum) };
        if isnum == 0 {
            return None;
        }
        return Some(lua_integer)
    }
}

impl Write for i64 {
    fn write_top(&self, lua: &mut Lua) {
        unsafe { luac::lua_pushinteger(lua.ptr, *self) };
    }
}

impl Read for String {
    fn read(lua: &mut Lua, idx: i32) -> Option<String> {
        unsafe {
            let mut len: size_t = 0;
            let str_ptr = luac::lua_tolstring(lua.ptr, idx, &mut len);
            let bytes = slice::from_raw_parts(str_ptr as *const u8, len as usize);
            let str = String::from_utf8_lossy(bytes);
            return Some(str.to_string());
        }
    }
}

impl Write for String {
    fn write_top(&self, lua: &mut Lua) {
        let str: &str = self.as_ref();
        str.write_top(lua);
    }
}

impl Write for str {
    fn write_top(&self, lua: &mut Lua) {
        let bytes = self.as_bytes();
        unsafe { luac::lua_pushlstring(lua.ptr, bytes.as_ptr() as *const c_char, bytes.len() as size_t) };
    }
}

// TODO Write for lua_pushstring
// TODO Write for lua_pushcclosure
// TODO Read for lua_tocfunction
// TODO Read for lua_touserdata
// TODO Write for lua_pushlightuserdata
// TODO Read for lua_tothread
// TODO Write for lua_pushthread
// TODO Read for lua_topointer
