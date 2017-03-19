extern crate libc;

mod calc_lexer;
mod calc_parser;

use std::io::Write;

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

fn getCStr(s: &str) -> *mut u8 {
    unsafe {
        let ret = libc::calloc(s.len() + 1, 1);
        libc::memcpy(ret, s.as_ptr() as *const _, s.len());
        return ret as *mut _;
    }
}

#[no_mangle]
pub extern fn parseCalcNode(p_str: *const u8, str_len: i32, p_code: *mut i32, p_node: &mut *mut calc_parser::CalcNode, p_err: &mut *mut u8) {
    unsafe {
        let s: &[u8] = std::slice::from_raw_parts(p_str, str_len as usize);
        let s = std::str::from_utf8(s);
        if s.is_err() {
            *p_code = 1;
            // *p_err = ;
            let mut err_str: Vec<u8> = Vec::new();
            write!(err_str, "{}", s.unwrap_err());
            *p_err = getCStr(std::str::from_utf8(&err_str).unwrap());
            return;
        }
        let ret = calc_parser::CalcParser::parse_str(s.unwrap());
        match ret {
            Ok(json) => {
                *p_code = 0;
                *p_node = std::mem::transmute(json.as_u64().unwrap());
            },
            Err(parse_err) => {
                *p_code = 1;
                let mut err_str: Vec<u8> = Vec::new();
                write!(err_str, "{}", parse_err);
                *p_err = getCStr(std::str::from_utf8(&err_str).unwrap());
            },
        }
    }
}

#[no_mangle]
pub extern fn freeCalcNode(pnode: &mut *mut calc_parser::CalcNode) {
    unsafe {
        if !(*pnode).is_null() {
            calc_parser::CalcNode::free_ptr(*pnode);
            *pnode = std::ptr::null_mut();
        }
    }
}