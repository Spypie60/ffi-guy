use ffi_guy::ffi_guy;

fn main() {
    println!("Hello, world!");
    unsafe {
        test();
    }
}
#[ffi_guy]
pub fn test() {
    println!("Hello, world!");
}
pub unsafe extern "C" fn test2() {
    println!("Hello, world!");
}