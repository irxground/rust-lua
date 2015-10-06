extern crate libc;
extern crate lua53_sys as luac;

mod data_types;
mod variant;

use std::ffi::CString;

pub use data_types::{ Read, Write };
pub use variant::Variant;

pub struct Lua {
    ptr: *mut luac::lua_State
}

impl Drop for Lua {
    fn drop(&mut self) {
        unsafe { luac::lua_close(self.ptr) }
    }
}

impl Lua {
    pub fn new() -> Lua {
        let ptr = unsafe { luac::luaL_newstate() };
        return Lua { ptr: ptr };
    }

    pub fn open_base(&mut self) -> bool {
        unsafe { luac::luaopen_base(self.ptr) != 0 }
    }

    pub fn current_stack_size(&mut self) -> i32 {
        unsafe { luac::lua_gettop(self.ptr) }
    }

    pub fn pop_stack(&mut self, n: i32) {
        unsafe { luac::lua_settop(self.ptr, - n - 1)}
    }

    pub fn get<T: Read>(&mut self, name: &str) -> Option<T> {
        let cstr = CString::new(name).unwrap();
        let success = unsafe { luac::lua_getglobal(self.ptr, cstr.as_ptr()) != 0 };
        if ! success {
            return None;
        }
        let value = T::read(self, -1);
        self.pop_stack(1);
        return value;
    }

    pub fn set<T: Write>(&mut self, name: &str, value: &T) {
        value.write_top(self);
        unsafe {
            let cstr = CString::new(name).unwrap();
            luac::lua_setglobal(self.ptr, cstr.as_ptr());
        }
    }

    pub fn eval(&mut self, code: &str) -> Result<Variant, String> {
        let cstr = CString::new(code).unwrap();
        let code = unsafe { luac::luaL_loadstring(self.ptr, cstr.as_ptr()) };
        if code != 0 {
            return Err("fail to parse".to_string()); // TODO error
        }
        let code = unsafe { luac::lua_pcallk(self.ptr, 0, 0, 0, 0, None) };
        if code != 0 {
            let str = String::read(self, -1).expect("cannot read error message");
            return Err(str);
        }
        let value = Variant::read(self, -1);
        self.pop_stack(1);
        if let Some(value) = value {
            Ok(value)
        } else {
            Ok(Variant::Nil)
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), String> {
        try!(self.eval(code));
        Ok(())
    }
}

// #[test]
// fn it_works() {
// }
