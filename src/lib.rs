extern crate libc;
extern crate lua53_sys as luac;

pub mod data_types;

use std::ffi::CString;
use std::ffi::CStr;
use libc::{ c_char };

use data_types::{ Read, Write };

fn with_cstr<T, F>(str: &str, f: F) -> T where F: Fn(*const c_char) -> T {
    let cstr = CString::new(str).unwrap();
    return f(cstr.as_ptr());
}

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

    pub fn get_top_index(&mut self) -> i32 {
        unsafe { luac::lua_gettop(self.ptr) }
    }

    /*
    fn assert_stack_size<T, F: Fn() -> T>(f: F) -> T {
        f()
    }
    */

    pub fn get<T: Read>(&mut self, name: &str) -> Option<T> {
        let success = with_cstr(name, |str| {
            unsafe { luac::lua_getglobal(self.ptr, str) != 0 }
        });
        if ! success {
            return None;
        }
        return T::read(self, -1);
    }

    pub fn set<T: Write>(&mut self, name: &str, value: &T) {
        value.write_top(self);
        unsafe {
            with_cstr(name.as_ref(), |str| {
                luac::lua_setglobal(self.ptr, str);
            });
        }
    }

    fn read_cstr(&mut self) -> &CStr {
        let mut len = 0;
        unsafe {
            CStr::from_ptr(luac::lua_tolstring(self.ptr, -1, &mut len))
        }
    }

    pub fn run(&mut self, code: &str) -> Result<(), String> {
        unsafe {
            let code = with_cstr(code.as_ref(), |str| {
                luac::luaL_loadstring(self.ptr, str)
            });
            if code != 0 {
                return Err("fail to parse".to_string()); // TODO error
            }
            let code = luac::lua_pcallk(self.ptr, 0, 0, 0, 0, None);
            if code != 0 {
                let cstr = self.read_cstr();
                return Err(String::from_utf8_lossy(cstr.to_bytes()).to_string())
            }
        }
        return Ok(());
    }
}

// #[test]
// fn it_works() {
// }
