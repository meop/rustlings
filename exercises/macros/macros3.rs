// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a hint.

// // I AM NOT DONE

// i think the "modern" answer relies on proper arrangement of crates
// which rustlings does not do
// https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files

#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    //pub(crate) use my_macro;
}

fn main() {
    //macros::my_macro!();

    my_macro!();
}
