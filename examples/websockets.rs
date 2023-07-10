
use std::ffi::CString;
use std::os::raw::c_char;

extern {
    fn callApiEndpointFromRust(name: *const c_char);
}

#[link(name = "callcpp", kind = "dylib")]
extern "C" {}
fn main() {
    let name = CString::new("Hello CPP!").unwrap();
    let name_ptr = name.as_ptr();
    unsafe { callApiEndpointFromRust(name_ptr) };
}