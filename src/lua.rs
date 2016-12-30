extern crate lua53_sys as luac;

use std::ffi::CString;
use read::Read;

pub type LuaPtr = *mut luac::lua_State;

pub struct Lua {
    pub ptr: LuaPtr,
}

impl Lua {
    pub fn new() -> Lua {
        let ptr = unsafe { luac::luaL_newstate() };
        return Lua {
            ptr: ptr
        };
    }

    pub fn get_value<T: Read>(&self, name: &str) -> Option<T> {
        let cstr = CString::new(name).unwrap();
        let success = unsafe { luac::lua_getglobal(self.ptr, cstr.as_ptr()) != 0 };
        if !success {
            self.pop_stack(1);
            return None;
        }
        let (v, size) = unsafe { T::read_from_stack(&self, -1) };
        self.pop_stack(size as i32);
        return v;
    }

    pub fn get_bool(&self, name: &str) -> Option<bool> {
        self.get_value::<bool>(name)
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
        self.get_value::<i64>(name)
    }

    pub fn set_int(&mut self, name: &str, value: i64) {
        unsafe {
            luac::lua_pushinteger(self.ptr, value);
            let name = CString::new(name).unwrap();
            luac::lua_setglobal(self.ptr, name.as_ptr());
        }
    }

    pub fn get_float(&self, name: &str) -> Option<f64> {
        self.get_value::<f64>(name)
    }

    pub fn set_float(&mut self, name: &str, value: f64) {
        unsafe {
            luac::lua_pushnumber(self.ptr, value);
            let name = CString::new(name).unwrap();
            luac::lua_setglobal(self.ptr, name.as_ptr());
        }
    }

    pub fn get_str(&self, name: &str) -> Option<&str> {
        self.get_value::<&str>(name)
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

