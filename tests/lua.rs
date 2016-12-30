extern crate lua;
use lua::Lua;

#[test]
fn test_bool() {
    let mut lua = Lua::new();
    lua.set_bool("a", true);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_bool("b");
    assert_eq!(None, result);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_bool("a");
    assert_eq!(Some(true), result);
    assert_eq!(0, lua.current_stack_size());
}

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

#[test]
fn test_float() {
    let mut lua = Lua::new();
    lua.set_float("a", 1.5);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_float("b");
    assert_eq!(None, result);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_float("a");
    assert_eq!(Some(1.5), result);
    assert_eq!(0, lua.current_stack_size());
}

