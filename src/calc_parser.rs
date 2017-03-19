extern crate rustc_serialize;

use self::rustc_serialize::json::Json;

use calc_lexer::CalcLexer;
use calc_lexer::CalcLexerError;


extern crate libc;
use std;

#[repr(C)]
#[derive(Debug)]
pub enum CalcNodeType {
    Add = 0,
    Sub = 1,
    Mul = 2,
    Div = 3,
    Minus = 4,
    Num = 5,
}

#[repr(C)]
#[derive(Debug)]
pub struct CalcNode {
    node_type: CalcNodeType,
    left: *mut CalcNode,
    right: *mut CalcNode,
    str_val: *mut u8,
}

impl CalcNode {
    fn new_ptr() -> *mut CalcNode {
        unsafe {
            let p: *mut libc::c_void = libc::calloc(1, std::mem::size_of::<CalcNode>());
            let node: &mut CalcNode = std::mem::transmute(p);
            node
        }
    }

    pub fn free_ptr(p: *mut CalcNode) {
        unsafe {
            let node = &mut *p;
            if !node.left.is_null() {
                Self::free_ptr(node.left);
            }
            if !node.right.is_null() {
                Self::free_ptr(node.right);
            }
            if !node.str_val.is_null() {
                libc::free(node.str_val as *mut _);
            }
            libc::free(node as *mut CalcNode as *mut _);
        }
    }

    fn new_node1(node_type: CalcNodeType, left: Json, right: Json) -> Option<Json>
    {
        let node = Self::new_ptr();
        unsafe {
            (*node).node_type = node_type;
            (*node).left = std::mem::transmute(left.as_u64().unwrap());
            (*node).right = std::mem::transmute(right.as_u64().unwrap());
        }
        return Some(Json::U64(node as u64));
    }
}


type Token = usize;
type State = usize;
type ProdIndex = usize;

#[derive(Debug)]
struct ProductionItem {
    header_id: usize,
    body_length: usize,
}

#[derive(Debug, Copy, Clone)]
enum Action {
    Reduce(ProdIndex),
    Shift(State),
    Accept,
    Goto(State),
    Error,
}

#[derive(Debug)]
pub struct CalcParser {
    prod_list: Vec<ProductionItem>,
    lookup_table: Vec<Action>,
    lookup_index: Vec<usize>,
}

impl CalcParser {
    pub fn new() -> CalcParser {
        CalcParser { prod_list: vec![ProductionItem{header_id: 10, body_length: 2}, ProductionItem{header_id: 9, body_length: 3}, ProductionItem{header_id: 9, body_length: 3}, ProductionItem{header_id: 9, body_length: 3}, ProductionItem{header_id: 9, body_length: 3}, ProductionItem{header_id: 9, body_length: 2}, ProductionItem{header_id: 9, body_length: 3}, ProductionItem{header_id: 9, body_length: 1}]
               , lookup_index: vec![3, 4, 5, 11, 12, 13, 14, 15, 16, 17, 18, 23, 24, 25, 26, 35, 37, 38, 43, 44, 45, 46, 47, 51, 52, 53, 54, 59, 60, 61, 62, 63, 67, 68, 69, 70, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 99, 100, 101, 102, 107, 108, 109, 110, 111, 112, 113, 114, 119, 120, 121, 122, 144, 145, 146, 151, 152, 153, 154]
               , lookup_table: vec![Action::Reduce(7), Action::Accept, Action::Reduce(5), Action::Reduce(6), Action::Reduce(1), Action::Reduce(2), Action::Reduce(3), Action::Reduce(4), Action::Shift(2), Action::Shift(2), Action::Shift(2), Action::Shift(2), Action::Shift(2), Action::Shift(2), Action::Shift(2), Action::Reduce(7), Action::Reduce(5), Action::Shift(11), Action::Reduce(6), Action::Reduce(1), Action::Reduce(2), Action::Reduce(3), Action::Reduce(4), Action::Reduce(7), Action::Shift(9), Action::Shift(9), Action::Shift(9), Action::Reduce(6), Action::Shift(9), Action::Shift(9), Action::Reduce(3), Action::Reduce(4), Action::Reduce(7), Action::Shift(7), Action::Reduce(5), Action::Shift(7), Action::Reduce(6), Action::Reduce(1), Action::Reduce(2), Action::Reduce(3), Action::Reduce(4), Action::Shift(1), Action::Shift(1), Action::Shift(1), Action::Reduce(7), Action::Shift(8), Action::Reduce(5), Action::Shift(8), Action::Shift(1), Action::Shift(1), Action::Shift(1), Action::Shift(1), Action::Reduce(6), Action::Reduce(1), Action::Reduce(2), Action::Reduce(3), Action::Reduce(4), Action::Reduce(7), Action::Shift(10), Action::Shift(10), Action::Shift(10), Action::Reduce(6), Action::Shift(10), Action::Shift(10), Action::Reduce(3), Action::Reduce(4), Action::Shift(3), Action::Shift(3), Action::Shift(3), Action::Shift(3), Action::Shift(3), Action::Shift(3), Action::Shift(3), Action::Goto(4), Action::Goto(5), Action::Goto(6), Action::Goto(12), Action::Goto(13), Action::Goto(14), Action::Goto(15)] }
    }

    fn lookup(&self, token: Token, state: State) -> Action {
        let input_length: usize = 10;
		let state_length: usize = 16;
		if token >= input_length || state >= state_length {
			return Action::Error;
		}
		self.find(token * state_length + state)
    }

    fn find(&self, value: usize) -> Action {
        let mut left: usize = 0;
        let mut right: usize = self.lookup_index.len();
        let mut old_mid: usize = 0;
        loop {
            let mut mid = (left + right) >> 1;
            let index_value = self.lookup_index[mid];
            if index_value == value {
                return self.lookup_table[mid].clone();
            } else if index_value > value {
                right = mid;
            } else {
                left = mid;
            }

            if old_mid == mid {
                return Action::Error;
            } else {
                old_mid = mid;
            }
        }
        Action::Error
    }

    pub fn parse(&self, lexer: &mut CalcLexer) -> Result<Json, CalcLexerError> {
		


        let mut state_stack: Vec<State> = vec![0];
        let mut output_stack: Vec<Json> = Vec::new();

        loop {
            let token = match lexer.get_token() {
                Ok(token) => token,
                Err(err) => return Err(err),
            };
            let state = state_stack[state_stack.len() - 1];
            let action = self.lookup(token, state);
            //println!("{:?}", ("cur_state", state, "token", CalcLexer::find_token_name(token), token, lexer.get_yytext(), "action", action));
            match action {
                Action::Accept => {
                    return Ok(output_stack.pop().unwrap());
                }
                Action::Shift(next_state) => {
                    output_stack.push(lexer.get_yytext());
                    state_stack.push(next_state);
                    lexer.advance();
                }
                Action::Reduce(prod_index) => {
                    let length = self.prod_list[prod_index].body_length;
                    let mut _result: Option<Json> = None;
                    match prod_index {
                        0x1 => {
 _result = CalcNode::new_node1(CalcNodeType::Add, (output_stack[output_stack.len() - 3].clone()), (output_stack[output_stack.len() - 1].clone())); 
},
0x2 => {
 _result = CalcNode::new_node1(CalcNodeType::Sub, (output_stack[output_stack.len() - 3].clone()), (output_stack[output_stack.len() - 1].clone())); 
},
0x3 => {
 _result = CalcNode::new_node1(CalcNodeType::Mul, (output_stack[output_stack.len() - 3].clone()), (output_stack[output_stack.len() - 1].clone())); 
},
0x4 => {
 _result = CalcNode::new_node1(CalcNodeType::Div, (output_stack[output_stack.len() - 3].clone()), (output_stack[output_stack.len() - 1].clone())); 
},
0x5 => {

        let node = CalcNode::new_ptr();
        unsafe {
            (*node).node_type = CalcNodeType::Minus;
            (*node).left = std::mem::transmute((output_stack[output_stack.len() - 1].clone()).as_u64().unwrap());
        };
        _result = Some(Json::U64(node as u64));
    
},
0x6 => {
 _result = Some((output_stack[output_stack.len() - 2].clone())); 
},
0x7 => {

        let node = CalcNode::new_ptr();
        unsafe {
            let bytes: Vec<u8> = Vec::from((output_stack[output_stack.len() - 1].clone()).as_string().unwrap());
            let s_ptr = libc::calloc(bytes.len() + 1, 1);
            libc::memcpy(s_ptr, bytes.as_ptr() as *const _, bytes.len());
            (*node).node_type = CalcNodeType::Num;
            (*node).str_val = s_ptr as *mut _;
        }
        _result = Some(Json::U64(node as u64));
    
},
_ => {},
                    };
                    if length > 0 && _result.is_none() {
                        _result = Some(output_stack[output_stack.len() - length].clone());
                    }
                    if _result.is_none() {
                        _result = Some(Json::Null);
                    }
                    {
                        let mut i = 0;
                        while i < length {
                            state_stack.pop();
                            output_stack.pop();
                            i += 1;
                        }
                    }
                    let next_state = state_stack[state_stack.len() - 1];
                    let next_action = self.lookup(self.prod_list[prod_index].header_id, next_state);
                    match next_action {
                        Action::Goto(next_state) => {
                            state_stack.push(next_state);
                            output_stack.push(_result.unwrap());
                        }
                        _ => return Err(lexer.new_lexer_error("Goto Error".to_string())),
                    };
                }
                _ => {
                    return Err(lexer.new_lexer_error("Parser Error".to_string()));
                }
            }
        }
        Ok(Json::Null)
    }

    pub fn parse_str(source: &str) -> Result<Json, CalcLexerError> {
        let mut lexer = CalcLexer::new();
        lexer.set_source(source);
        let mut p = Self::new();
        p.parse(&mut lexer)
    }
}
