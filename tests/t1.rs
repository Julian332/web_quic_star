extern "C" {
    fn abs(input: i32) -> i32;
}

#[test]
fn c() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

unsafe trait Foo {
    // 方法列表
}

unsafe impl Foo for i32 {
    // 实现相应的方法
}

fn main() {}

#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}
