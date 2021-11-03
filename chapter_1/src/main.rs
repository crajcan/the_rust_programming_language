#[deny(clippy::absurd_extreme_comparisons)]
fn main() {
    println!("Henlo World!");
}

/*
notes
run clippy: cargo clippy --fix --allow-staged || --allow-dirty
run rustfmt: rustfmt src/main.rs || src.lib.rs
*/
