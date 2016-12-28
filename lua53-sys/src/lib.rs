#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
mod lua {
    include!(concat!(env!("OUT_DIR"), "/lua.rs"));
}

pub use lua::{
    /* struct */
    lua_State,

    /* function */
    lua_close,
    luaL_newstate,
};

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
