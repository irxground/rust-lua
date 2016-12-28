extern crate lua53_sys as luac;

use std::ffi::CString;

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

    pub fn current_stack_size(&self) -> i32{
        unsafe { luac::lua_gettop(self.ptr) }
    }

    pub fn set_int(&mut self, name: &str, value: i64) {
        unsafe {
            luac::lua_pushinteger(self.ptr, value);
            let name = CString::new(name).unwrap();
            luac::lua_setglobal(self.ptr, name.as_ptr());
        }
    }

    pub fn get_int(&self, name: &str) -> Option<i64> {
        unsafe {
            let cstr = CString::new(name).unwrap();
            let success = luac::lua_getglobal(self.ptr, cstr.as_ptr()) != 0;
            if ! success {
                self.pop_stack();
                return None;
            }

            let idx = -1;
            let mut isnum = 0;
            let lua_integer = luac::lua_tointegerx(self.ptr, idx, &mut isnum);
            self.pop_stack();
            if isnum == 0 {
                return None;
            }
            return Some(lua_integer);
        }
    }

    fn pop_stack(&self) {
        let n = 1;
        unsafe { luac::lua_settop(self.ptr, -1 - n) }
    }
}

impl Drop for Lua {
    fn drop(&mut self) {
        unsafe { luac::lua_close(self.ptr); }
    }
}

