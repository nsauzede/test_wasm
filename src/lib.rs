// this is a simple interface without leveraging from the Rust language bells and whistles
#[no_mangle]
pub extern fn add(x: u32, y: u32) -> u32 {
    x + y
}
