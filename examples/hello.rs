extern crate lua53 as lua;

fn main() {
    let mut lua = lua::Lua::new();
    lua.open_base();
    let result = lua.run(r##"
        print("Hello, world!")
    "##);
    if let Err(msg) = result {
        println!("Err: {:?}", msg);
        return;
    }
}
