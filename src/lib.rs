#![feature(plugin)]
#![plugin(clippy)]
#![deny(clippy)]

//! MyIdent should pass because it's declared as valid

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
