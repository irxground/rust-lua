
#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
mod generated {
    include!(concat!(env!("OUT_DIR"), "/lua.rs"));
}

#[cfg_attr(rustfmt, rustfmt_skip)]
pub use generated::{
    /* struct */
    lua_State,
    lua_CFunction,

    /* function */
    lua_close,
    lua_getglobal,
    lua_gettop,
    lua_newuserdata,
    lua_pcallk,
    lua_pushboolean,
    lua_touserdata,
    lua_pushcclosure,
    lua_pushinteger,
    lua_pushlstring,
    lua_pushnil,
    lua_pushnumber,
    lua_setglobal,
    lua_settop,
    lua_toboolean,
    lua_tointegerx,
    lua_tonumberx,
    luaL_loadstring,
    luaL_newstate,
    luaL_tolstring,
};

mod manual {
    use super::generated::*;
    pub fn lua_upvalueindex(i: i32) -> i32 {
        LUA_REGISTRYINDEX - i
    }
}

pub use manual::lua_upvalueindex;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            let lua = luaL_newstate();
            lua_close(lua);
        }
    }
}
