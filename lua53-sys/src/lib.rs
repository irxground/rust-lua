#[allow(non_snake_case)]
#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(dead_code)]
mod lua {
    include!(concat!(env!("OUT_DIR"), "/lua.rs"));
}

#[cfg(test)]
mod tests {
    use super::lua::*;

    #[test]
    fn it_works() {
        unsafe {
            let lua = luaL_newstate();
    
            lua_close(lua);
        }
    }
}
