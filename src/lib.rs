extern crate libc;

#[no_mangle]
pub extern fn hello_world() {
    let mut vec: Vec<Option<i32>> = Vec::new();
    vec.push(None);

    println!("vec {:?}", vec.len());
}

#[no_mangle]
pub extern fn createI32Vec() -> *mut Vec<i32> {
    unsafe {
        let mut v = Box::new(Vec::new());
        Box::into_raw(v)
    }
}

#[no_mangle]
pub extern fn pushI32Vec(p: *mut Vec<i32>, i: i32) -> i32 {
    if p.is_null() {
        println!("null ptr");
        return 0;
    }
    unsafe {
        let mut v: Box<Vec<i32>> = Box::from_raw(p);
        v.push(i);
        let l = v.len() as i32;
        println!("{:?}", v);
        Box::into_raw(v);
        return l;
    }
}

#[no_mangle]
pub extern fn dropI32Vec(p: &mut *mut Vec<i32>) {
    if p.is_null() {
        return;
    }
    unsafe {
        let mut v: Box<Vec<i32>> = Box::from_raw(*p);
    }
    *p = std::ptr::null_mut();
}