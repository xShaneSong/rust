use std::ffi::{CStr, CString};
use std::os::raw::c_char;


extern "C" {
    fn getenv(s: *const c_char) -> *mut c_char;
}

#[repr(C)]
union MyUnion {
    i: i32,
    f: f32,
}

fn main() {
    let c1 = CString::new("PATH").unwrap();
    unsafe {
        let path = getenv(c1.as_ptr());
        let c_str = CStr::from_ptr(path);
        let r_str = c_str.to_str().unwrap();
        println!("PATH: {}", r_str);
    }

    let float = MyUnion { f: 3.14 };
    let f = unsafe { float.f };
    println!("f: {:.3}", f);
}

