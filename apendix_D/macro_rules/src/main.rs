#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let v = my_vec! {1, 2, 3, 4};
    println!("v: {:?}", v);
}
