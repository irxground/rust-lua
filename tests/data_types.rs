extern crate lua53 as lua;

use lua::Lua;

#[test]
fn for_f64() {
    let v: f64 = 1.0;
    let mut lua = Lua::new();
    lua.set("a", v);
    assert_eq!(Some(v), lua.get("a"));
}

#[test]
fn for_i64() {
    let v: i64 = 123;
    let mut lua = Lua::new();
    lua.set("a", v);
    assert_eq!(Some(v), lua.get("a"));
}
