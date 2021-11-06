fn main() {
    let mut a = Some("Hello Friend");
 
    swap_if_exists(&mut a);

    println!("mut a is now: {:?}", a);
}

// change 'Friend' to 'World' if the string exists
fn swap_if_exists(b: &mut Option<&str>) {
    if let Some(s) = b {
        *b = Some(&s.replace("Friend", "World").clone());
    }
}