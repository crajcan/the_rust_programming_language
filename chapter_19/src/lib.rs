#[no_mangle]
// build command:
// cbindgen --config cbindgen.toml --crate chapter_19 --output rust_fn.h --lang c
pub extern "C" fn call_from_c() {
    println!("Just called a rust fn from C");
}
