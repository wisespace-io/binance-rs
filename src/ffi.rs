use std::ffi::CStr;
use std::os::raw::c_char;

fn dummy(_: *const c_char) -> *mut c_char {
    std::ptr::null_mut()
}

static mut FUNC_CPP_FROM_RUST: fn(s: *const c_char) -> *mut c_char = dummy;

#[no_mangle]
pub extern "C" fn initFromCpp(callback: fn(_: *const c_char) -> *mut c_char) -> i32 {
    unsafe {
        FUNC_CPP_FROM_RUST = callback;
    }
    0
}
#[no_mangle]
pub extern "C" fn rustFromCpp(s: *const c_char) -> *mut c_char {
    unsafe {
        let c_str = CStr::from_ptr(s);
        let rust_str = c_str.to_str().unwrap();
        println!("Received String From CPP: {}", rust_str);
        FUNC_CPP_FROM_RUST(s);
        std::ptr::null_mut()
    }
}
