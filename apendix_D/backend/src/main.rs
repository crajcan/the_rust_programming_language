extern crate function_like;
use function_like::sql;

fn main() {
    let sql = sql!("SELECT * FROM posts WHERE id=1");
    println!("{}", sql);
}
