extern crate libc;
extern crate lua53_sys as luac;

use std::ffi::CString;
use std::ffi::CStr;
use libc::{c_char};

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

    pub fn get_top(&mut self) -> i32 {
        unsafe { luac::lua_gettop(self.ptr) }
    }

    /*
    fn assert_stack_size<T, F: Fn() -> T>(f: F) -> T {
        f()
    }
    */

    pub fn set_number<T: AsRef<str>>(&mut self, name: T, value: f64) {
        unsafe {
            luac::lua_pushnumber(self.ptr, value);
            with_cstr(name.as_ref(), |str| {
                luac::lua_setglobal(self.ptr, str);
            });
        }
    }

    pub fn get_number<T: AsRef<str>>(&mut self, name: T) -> Option<f64> {
        unsafe {
            let success = with_cstr(name.as_ref(), |str| {
                luac::lua_getglobal(self.ptr, str)
            });
            if success == 0{
                return None;
            }
            let mut isnum = 0;
            let lua_number = luac::lua_tonumberx(self.ptr, -1, &mut isnum);
            if isnum == 0 {
                return None;
            }
            return Some(lua_number);
        }
    }

    fn read_cstr(&mut self) -> &CStr {
        let mut len = 0;
        unsafe {
            CStr::from_ptr(luac::lua_tolstring(self.ptr, -1, &mut len))
        }
    }

    pub fn run<T: AsRef<str>>(&mut self, code: T) -> Result<(), String> {
        unsafe {
            let code = with_cstr(code.as_ref(), |str| {
                luac::luaL_loadstring(self.ptr, str)
            });
            if code != 0 {
                return Err("fail to parse".to_string()); // TODO error
            }
            let code = luac::lua_pcallk(self.ptr, 0, 0, 0, 0, 0);
            if code != 0 {
                let cstr = self.read_cstr();
                // return Err(cstr.to_str().unwrap().to_string());
                return Err(String::from_utf8_lossy(cstr.to_bytes()).to_string())
            }
        }
        return Ok(());
    }
}

#[test]
fn it_works() {
    let mut lua = Lua::new();
    lua.set_number("a", 100.0);
    assert_eq!(Some(100.0), lua.get_number("a"));
    assert_eq!(None, lua.get_number("b"));

    let ok = lua.run("c = a * 2");
    assert!(ok);
    assert_eq!(Some(200.0), lua.get_number("c"));
}
