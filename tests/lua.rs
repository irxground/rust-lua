extern crate lua;
use lua::Lua;

#[test]
fn test_int() {
    let mut lua = Lua::new();
    lua.set_int("a", 1);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_int("b");
    assert_eq!(None, result);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_int("a");
    assert_eq!(Some(1), result);
    assert_eq!(0, lua.current_stack_size());
}

