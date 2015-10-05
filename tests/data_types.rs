extern crate lua53 as lua;

use lua::Lua;

#[test]
fn for_f64() {
    let mut lua = Lua::new();
    lua.set("a", 1.0);
    assert_eq!(Some(1.0), lua.get("a"));
}
