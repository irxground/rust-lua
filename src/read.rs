extern crate lua53_sys as luac;

use lua::Lua;

pub trait Read: Sized {
    unsafe fn read_from_stack(lua: &Lua, idx: i32) -> (Option<Self>, u8);
}

impl Read for bool {
    unsafe fn read_from_stack(lua: &Lua, idx: i32) -> (Option<bool>, u8) {
        let value = luac::lua_toboolean(lua.ptr, idx) != 0;
        (Some(value), 1)
    }
}

impl Read for i64 {
    unsafe fn read_from_stack(lua: &Lua, idx: i32) -> (Option<i64>, u8) {
        let mut isnum = 0;
        let value = luac::lua_tointegerx(lua.ptr, idx, &mut isnum);
        let v = if isnum != 0 { Some(value) } else { None };
        (v, 1)
    }
}

impl Read for f64 {
    unsafe fn read_from_stack(lua: &Lua, idx: i32) -> (Option<f64>, u8) {
        let mut isnum = 0;
        let value = luac::lua_tonumberx(lua.ptr, idx, &mut isnum);
        let v = if isnum != 0 { Some(value) } else { None };
        (v, 1)
    }
}

impl<'a> Read for &'a str {
    unsafe fn read_from_stack(lua: &Lua, idx: i32) -> (Option<&'a str>, u8) {
        use std::slice::from_raw_parts;
        use std::str::from_utf8;

        let mut len = 0;
        let ptr = luac::luaL_tolstring(lua.ptr, idx, &mut len) as *const u8;
        let slice = from_raw_parts(ptr, len);
        let v = match from_utf8(slice) {
            Ok(x) => Some(x),
            Err(_) => None,
        };
        (v, 2)
    }
}
