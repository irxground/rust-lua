extern crate lua53_sys as luac;

use std::ffi::CString;
use read::Read;
use write::Write;

pub type LuaPtr = *mut luac::lua_State;

pub struct Lua {
    pub ptr: LuaPtr,
}

impl Lua {
    pub fn new() -> Lua {
        let ptr = unsafe { luac::luaL_newstate() };
        Lua { ptr: ptr }
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

    pub fn set_value<T: Write>(&mut self, name: &str, value: T) {
        unsafe {
            value.write_to_stack(&self);
            let name = CString::new(name).unwrap();
            luac::lua_setglobal(self.ptr, name.as_ptr());
        }
    }

    pub fn remove_value(&mut self, name: &str) {
        unsafe {
            luac::lua_pushnil(self.ptr);
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

    pub fn set_fn<F: FnMut() -> ()>(&mut self, name: &str, f: F) {
        use std::mem::size_of;

        unsafe extern "C" fn callback<F: FnMut() -> ()>(ptr: LuaPtr) -> i32 {
            let f = luac::lua_touserdata(ptr, luac::lua_upvalueindex(1)) as *mut F;
            (*f)();
            return 0;
        }

        unsafe {
            // create user data
            let size = size_of::<F>();
            let loc = luac::lua_newuserdata(self.ptr, size) as *mut F;
            debug_assert!(self.current_stack_size() == 1);
            *loc = f;
            // set metatable
            self.push_metatable_for_gc::<F>();
            luac::lua_setmetatable(self.ptr, -2);
            debug_assert!(self.current_stack_size() == 1);
            // set global
            luac::lua_pushcclosure(self.ptr, Some(callback::<F>), 1);
            let name = CString::new(name).unwrap();
            luac::lua_setglobal(self.ptr, name.as_ptr());
        }
    }

    pub fn current_stack_size(&self) -> i32 {
        unsafe { luac::lua_gettop(self.ptr) }
    }

    fn pop_stack(&self, n: i32) {
        unsafe { luac::lua_settop(self.ptr, -1 - n) }
    }

    fn push_metatable_for_gc<T>(&mut self) {
        use std::ptr::drop_in_place;

        unsafe extern "C" fn call_drop<T>(ptr: LuaPtr) -> i32 {
            let item = luac::lua_touserdata(ptr, luac::lua_upvalueindex(1)) as *mut T;
            drop_in_place(item);
            return 0;
        }

        unsafe {
            let meta_table_name = format!("meta-{:p}", &call_drop::<T>);
            let meta_table_name = CString::new(meta_table_name).unwrap();
            luac::luaL_newmetatable(self.ptr, meta_table_name.as_ptr());
            debug_assert!(self.current_stack_size() == 2);
            "__gc".write_to_stack(self);
            luac::lua_pushcclosure(self.ptr, Some(call_drop::<T>), 0);
            debug_assert!(self.current_stack_size() == 4);
            luac::lua_rawset(self.ptr, -3);
            debug_assert!(self.current_stack_size() == 2);
        }
    }
}

impl Drop for Lua {
    fn drop(&mut self) {
        unsafe {
            luac::lua_close(self.ptr);
        }
    }
}
