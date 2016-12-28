extern crate lua53_sys as luac;

pub struct Lua {
    ptr: *mut luac::lua_State,
}

impl Lua {
    pub fn new() -> Lua {
        let ptr = unsafe { luac::luaL_newstate() };
        return Lua {
            ptr: ptr
        };
    }
}

impl Drop for Lua {
    fn drop(&mut self) {
        unsafe { luac::lua_close(self.ptr); }
    }
}

