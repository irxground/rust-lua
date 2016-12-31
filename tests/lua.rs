extern crate lua;
use lua::Lua;

#[test]
fn test_bool() {
    let mut lua = Lua::new();
    lua.set_value("a", true);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_value::<bool>("b");
    assert_eq!(None, result);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_value("a");
    assert_eq!(Some(true), result);
    assert_eq!(0, lua.current_stack_size());
}

#[test]
fn test_int() {
    let mut lua = Lua::new();
    lua.set_value("a", 1);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_value::<i64>("b");
    assert_eq!(None, result);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_value("a");
    assert_eq!(Some(1), result);
    assert_eq!(0, lua.current_stack_size());
}

#[test]
fn test_float() {
    let mut lua = Lua::new();
    lua.set_value("a", 1.5);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_value::<f64>("b");
    assert_eq!(None, result);
    assert_eq!(0, lua.current_stack_size());

    let result = lua.get_value("a");
    assert_eq!(Some(1.5), result);
    assert_eq!(0, lua.current_stack_size());
}

#[test]
fn test_str() {
    let mut lua = Lua::new();

    lua.set_value("a", "hello");
    assert_eq!(0, lua.current_stack_size());

    assert_eq!(None, lua.get_value::<&str>("b"));
    assert_eq!(0, lua.current_stack_size());

    assert_eq!(Some("hello"), lua.get_value("a"));
    assert_eq!(0, lua.current_stack_size());

    lua.set_value("a", "");
    assert_eq!(Some(""), lua.get_value("a"));
    assert_eq!(0, lua.current_stack_size());
}

#[test]
fn test_run() {
    let mut lua = Lua::new();

    lua.run("a = 1 + 1").unwrap();
    assert_eq!(Some(2), lua.get_value("a"));
    assert_eq!(0, lua.current_stack_size());
}
