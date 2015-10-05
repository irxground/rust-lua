extern crate lua53_sys as luac;

use Lua;

/// Luaスタックから読み込み可能
pub trait Read {
    fn read(lua: &mut Lua, idx: i32) -> Option<Self>;
}

/// Luaスタックに書き込み可能
pub trait Write {
    fn write_top(&self, lua: &mut Lua);
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
