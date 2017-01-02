extern crate lua;
use lua::Lua;

#[test]
fn test_bool() {
    let mut lua = Lua::new();
    lua.set_value("a", true);
    assert_eq!(0, lua.current_stack_size());

    assert_eq!(None, lua.get_value::<bool>("b"));
    assert_eq!(0, lua.current_stack_size());

    assert_eq!(Some(true), lua.get_value("a"));
    assert_eq!(0, lua.current_stack_size());

    lua.remove_value("a");

    assert_eq!(None, lua.get_value::<bool>("a"));
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

    {
        let result = lua.run("true + false");
        assert!(result.is_err());
    }
    assert_eq!(0, lua.current_stack_size());

    lua.run("a = 1 + 1").unwrap();
    assert_eq!(Some(2), lua.get_value("a"));
    assert_eq!(0, lua.current_stack_size());
}

#[test]
fn test_fn() {
    let mut n = 1;
    {
        let mut lua = Lua::new();
        lua.set_fn("inc", || n = n + 1);
        assert_eq!(0, lua.current_stack_size());
        lua.run("inc();inc();inc();").unwrap();
    }
    assert_eq!(4, n);
}

#[test]
fn test_fn_with_gc() {
    use std::rc::Rc;
    use std::cell::Cell;

    let mut n = Rc::new(Cell::new(1));
    assert!(Rc::get_mut(&mut n).is_some());
    {
        let m = n.clone();
        assert!(Rc::get_mut(&mut n).is_none());

        let mut lua = Lua::new();
        lua.set_fn("inc", || m.set(m.get() + 1));
        assert_eq!(0, lua.current_stack_size());
        lua.run("inc();inc();inc();").unwrap();
    }
    assert!(Rc::get_mut(&mut n).is_some());
    assert_eq!(4, n.get());
}
