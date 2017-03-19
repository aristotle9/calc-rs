extern crate rustc_serialize;
extern crate libc;

mod calc_lexer;
mod calc_parser;

fn main() {
    let expr = "1+2*7";
    let ret = calc_lexer::CalcLexer::lex_seq(expr);
    // println!("{:?}", ret);
    // println!("{:?}", std::mem::size_of::<calc_parser::CalcNodeType>());
    // println!("{:?}", std::mem::size_of::<calc_parser::CalcNode>());
    // println!("{:?}", std::mem::size_of::<*mut libc::c_void>());

    let expr = "-23 * 8 / 34";
    let ret = calc_parser::CalcParser::parse_str(expr);
    if ret.is_ok() {
        let json = ret.unwrap();
        if json.is_u64() {
            let ptr = json.as_u64().unwrap();
            unsafe {
                let node: &mut calc_parser::CalcNode = std::mem::transmute(ptr);
                // libc::malloc(1024 * 1024);
                // println!("{:?}", node);
                calc_parser::CalcNode::free_ptr(node as *mut calc_parser::CalcNode);
                // libc::free(ptr as *mut libc::c_void);
            }
        }
    }
    // println!("{:?}", ret);
}