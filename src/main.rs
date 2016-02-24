#[macro_use]
extern crate lazy_static;
extern crate regex;

mod one;
mod two;
mod three;
// mod four;
mod five;
mod six;

fn main() {
    one::solve();
    two::solve();
    three::solve();

    five::solve();
    six::solve();
}
