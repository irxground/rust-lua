extern crate libc;
extern crate lua53_sys as luac;

use Lua;
use data_types::{ Read, Write };

pub enum Variant {
    Nil,
    Bool(bool),
    Int(i64),
    Num(f64),
    Str(String),
    Unknown,

    /*
        TODO support following types
        lua_iscfunction
        lua_isfunction
        lua_islightuserdata
        lua_istable
        lua_isthread
        lua_isuserdata
        lua_isyieldable
    */
}

impl Read for Variant {
    fn read(lua: &mut Lua, idx: i32) -> Option<Variant> {
        use Variant::*;

        let lua_type = unsafe { luac::lua_type(lua.ptr, idx) };
        match lua_type {
            luac::LUA_TNONE => None,
            luac::LUA_TNIL => Some(Nil),
            luac::LUA_TBOOLEAN => bool::read(lua, idx).map(|x| Bool(x)),
            luac::LUA_TNUMBER => {
                if unsafe { luac::lua_isinteger(lua.ptr, idx) } != 0 {
                    i64::read(lua, idx).map(|x| Int(x))
                } else {
                    f64::read(lua, idx).map(|x| Num(x))
                }
            },
            luac::LUA_TSTRING => String::read(lua, idx).map(|x| Str(x)),
            _ => Some(Unknown)
        //     LUA_TLIGHTUSERDATA
        //     LUA_TSTRING
        //     LUA_TTABLE
        //     LUA_TFUNCTION
        //     LUA_TUSERDATA
        //     LUA_TTHREAD
        //     LUA_NUMTAGS
        }
    }
}

impl Write for Variant {
    fn write_top(&self, lua: &mut Lua) {
        match *self {
            Variant::Nil => ().write_top(lua),
            Variant::Bool(x) => x.write_top(lua),
            Variant::Int(x) => x.write_top(lua),
            Variant::Num(x) => x.write_top(lua),
            Variant::Str(ref x) => x.write_top(lua),
            _ => unimplemented!()
        }
    }
}
