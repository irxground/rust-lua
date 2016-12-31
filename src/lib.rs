
pub use lua::Lua;
mod lua;
mod read;
mod write;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _ = Lua::new();
    }
}
