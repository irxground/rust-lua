extern crate lua53_sys as luac;

use lua::Lua;

pub trait Write: Sized {
    unsafe fn write_to_stack(self, lua: &Lua);
}

impl Write for () {
    unsafe fn write_to_stack(self, lua: &Lua) {
        luac::lua_pushnil(lua.ptr);
    }
}

impl Write for bool {
    unsafe fn write_to_stack(self, lua: &Lua) {
        luac::lua_pushboolean(lua.ptr, self as i32);
    }
}

impl Write for i64 {
    unsafe fn write_to_stack(self, lua: &Lua) {
        luac::lua_pushinteger(lua.ptr, self);
    }
}

impl Write for f64 {
    unsafe fn write_to_stack(self, lua: &Lua) {
        luac::lua_pushnumber(lua.ptr, self);
    }
}

impl<'a> Write for &'a str {
    unsafe fn write_to_stack(self, lua: &Lua) {
        luac::lua_pushlstring(lua.ptr, self.as_ptr() as *const i8, self.len());
    }
}
