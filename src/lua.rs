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

    fn get(&self, name: &str) -> bool {
        unsafe {
            let cstr = CString::new(name).unwrap();
            return luac::lua_getglobal(self.ptr, cstr.as_ptr()) != 0;
        }
    }

    pub fn get_bool(&self, name: &str) -> Option<bool> {
        if !self.get(name) {
            self.pop_stack(1);
            return None;
        }
        let value = unsafe { luac::lua_toboolean(self.ptr, -1) != 0};
        self.pop_stack(1);
        return Some(value);
    }

    pub fn set_bool(&mut self, name: &str, value: bool) {
        unsafe {
            let b = if value { 1 } else { 0 };
            luac::lua_pushboolean(self.ptr, b);
            let name = CString::new(name).unwrap();
            luac::lua_setglobal(self.ptr, name.as_ptr());
        }
    }

    pub fn get_int(&self, name: &str) -> Option<i64> {
        if !self.get(name) {
            self.pop_stack(1);
            return None;
        }
        let idx = -1;
        let mut isnum = 0;
        let value = unsafe { luac::lua_tointegerx(self.ptr, idx, &mut isnum) };
        self.pop_stack(1);
        if isnum == 0 {
            return None;
        }
        return Some(value);
    }

    pub fn set_int(&mut self, name: &str, value: i64) {
        unsafe {
            luac::lua_pushinteger(self.ptr, value);
            let name = CString::new(name).unwrap();
            luac::lua_setglobal(self.ptr, name.as_ptr());
        }
    }

    pub fn get_float(&self, name: &str) -> Option<f64> {
        if !self.get(name) {
            self.pop_stack(1);
            return None;
        }
        let idx = -1;
        let mut isnum = 0;
        let value = unsafe { luac::lua_tonumberx(self.ptr, idx, &mut isnum) };
        self.pop_stack(1);
        if isnum == 0 {
            return None;
        }
        return Some(value);
    }

    pub fn set_float(&mut self, name: &str, value: f64) {
        unsafe {
            luac::lua_pushnumber(self.ptr, value);
            let name = CString::new(name).unwrap();
            luac::lua_setglobal(self.ptr, name.as_ptr());
        }
    }

    pub fn get_str(&self, name: &str) -> Option<&str> {
        use std::slice::from_raw_parts;
        use std::str::from_utf8;

        if !self.get(name) {
            self.pop_stack(1);
            return None;
        }
        let slice = unsafe {
            let mut len = 0;
            let ptr = luac::luaL_tolstring(self.ptr, -1, &mut len) as *const u8;
            from_raw_parts(ptr, len)
        };
        self.pop_stack(2);
        match from_utf8(slice) {
            Ok(x) => Some(x),
            Err(_) => None,
        }
    }

    pub fn set_str(&mut self, name: &str, value: &str) {
        unsafe {
            luac::lua_pushlstring(self.ptr, value.as_ptr() as *const i8, value.len());
            let name = CString::new(name).unwrap();
            luac::lua_setglobal(self.ptr, name.as_ptr());
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), ()> {
        let nargs = 0;
        let nresults = 0;

        let cstr = CString::new(code).unwrap();
        let code = unsafe { luac::luaL_loadstring(self.ptr, cstr.as_ptr()) };
        if code != 0 {
            debug_assert!(self.current_stack_size() == 0);
            return Err(());
        }
        let code = unsafe { luac::lua_pcallk(self.ptr, nargs, nresults, 0, 0, None) };
        if code != 0 {
            debug_assert!(self.current_stack_size() == 0);
            return Err(());
        }
        debug_assert!(self.current_stack_size() == 0);
        return Ok(());
    }

    pub fn current_stack_size(&self) -> i32{
        unsafe { luac::lua_gettop(self.ptr) }
    }

    fn pop_stack(&self, n: i32) {
        unsafe { luac::lua_settop(self.ptr, -1 - n) }
    }
}

impl Drop for Lua {
    fn drop(&mut self) {
        unsafe { luac::lua_close(self.ptr); }
    }
}

