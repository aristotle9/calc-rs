extern crate rustc_serialize;

use self::rustc_serialize::json::Json;

use std::error::Error;
use std::fmt;
use std::u64;



/**
 * Created by as3cc on Sat Mar 18 16:26:03 GMT+0800 2017.
 */
type TransTable = Vec<RangeItem>;
type Token = usize;
type CalcLexerResult = Result<Token, CalcLexerError>;
const DEADSTATE: usize = 0xffffffff;
const END_TOKEN: usize = 0;


#[derive(Debug)]
pub struct CalcLexerError {
    token_name: &'static str,
    token_index: usize,
    line: u64,
    col: u64,
    reason: String,
}

impl fmt::Display for CalcLexerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CalcLexerError: {} {}@row:{}col:{}", self.reason, self.token_name, self.line, self.col)
    }
}

impl Error for CalcLexerError {
    fn description(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug)]
struct StateTransItem {
    is_dead: bool,
    final_index: i64,
    to_states: Vec<usize>,
    trans_edge: TransTable,
}

#[derive(Debug)]
struct RangeItem {
    from: u64,
    to: u64,
    value: u64,
}

#[derive(Debug)]
pub struct CalcLexer {
    trans_table: Vec<StateTransItem>,
    input_table: TransTable,
    _start:  usize,
    _old_start:  usize,
    _token_index:  usize,
    _yytext:  Json,
    _yy:  usize,//&mut u8
    _ended:  bool,
    _initial_input:  usize,
    _line:  u64,
    _col:  u64,
    _advanced:  bool,
    _source:  Vec<char>,//
}

impl CalcLexer {
    
    pub fn find_token_name(token_index: usize) -> &'static str {
        return match token_index {
0 => "<$>",
1 => "(",
2 => ")",
3 => "*",
4 => "+",
5 => "-",
6 => "/",
7 => "num",
_ => "<undefined_token>",
};
    }

    pub fn find_initial_name(initial_input: u64) -> &'static str {
        return match initial_input {
1 => "INITIAL",
_ => "undefined initial input",
};
    }
    
    #[inline]
    pub fn is_end_token(token: usize) -> bool {
        token == END_TOKEN
    }

    pub fn new() -> CalcLexer {
        ;let _trans_table = 
vec![StateTransItem{is_dead:false, final_index:4294967295, to_states:vec![4294967295, 1], trans_edge:vec![RangeItem{from:0, to:8, value:0}, RangeItem{from:9, to:9, value:1}]}, StateTransItem{is_dead:false, final_index:4294967295, to_states:vec![4294967295, 9, 8, 7, 6, 5, 4, 3, 2], trans_edge:vec![RangeItem{from:0, to:0, value:0}, RangeItem{from:1, to:1, value:1}, RangeItem{from:2, to:2, value:2}, RangeItem{from:3, to:3, value:3}, RangeItem{from:4, to:4, value:4}, RangeItem{from:5, to:5, value:5}, RangeItem{from:6, to:6, value:6}, RangeItem{from:7, to:7, value:7}, RangeItem{from:8, to:8, value:8}, RangeItem{from:9, to:9, value:0}]}, StateTransItem{is_dead:false, final_index:0, to_states:vec![4294967295, 2], trans_edge:vec![RangeItem{from:0, to:7, value:0}, RangeItem{from:8, to:8, value:1}, RangeItem{from:9, to:9, value:0}]}, StateTransItem{is_dead:true, final_index:1, to_states:vec![], trans_edge:vec![]}, StateTransItem{is_dead:true, final_index:3, to_states:vec![], trans_edge:vec![]}, StateTransItem{is_dead:true, final_index:2, to_states:vec![], trans_edge:vec![]}, StateTransItem{is_dead:true, final_index:4, to_states:vec![], trans_edge:vec![]}, StateTransItem{is_dead:true, final_index:5, to_states:vec![], trans_edge:vec![]}, StateTransItem{is_dead:false, final_index:7, to_states:vec![4294967295, 8], trans_edge:vec![RangeItem{from:0, to:1, value:0}, RangeItem{from:2, to:2, value:1}, RangeItem{from:3, to:9, value:0}]}, StateTransItem{is_dead:true, final_index:6, to_states:vec![], trans_edge:vec![]}]
;let _input_table = 
vec![RangeItem{from:0, to:8, value:0}, RangeItem{from:9, to:10, value:2}, RangeItem{from:11, to:11, value:0}, RangeItem{from:12, to:13, value:2}, RangeItem{from:14, to:31, value:0}, RangeItem{from:32, to:32, value:2}, RangeItem{from:33, to:39, value:0}, RangeItem{from:40, to:40, value:3}, RangeItem{from:41, to:41, value:1}, RangeItem{from:42, to:42, value:6}, RangeItem{from:43, to:43, value:7}, RangeItem{from:44, to:44, value:0}, RangeItem{from:45, to:45, value:5}, RangeItem{from:46, to:46, value:0}, RangeItem{from:47, to:47, value:4}, RangeItem{from:48, to:57, value:8}, RangeItem{from:58, to:65535, value:0}]
;
        CalcLexer {
            trans_table: _trans_table,
            input_table: _input_table,
            _start: 0,
            _old_start: 0,
            _token_index: 0,
            _yytext: Json::Null,
            _yy: 0,
            _ended: false,
            _initial_input: 1,
            _line: 0,
            _col: 0,
            _advanced: true,
            _source: vec![],
        }
    }

    fn restart(&mut self, source: &str) {
        self._ended = false;
        self._start = 0;
        self._old_start = 0;
        self._line = 0;
        self._col = 0;
        self._advanced = true;
        self._token_index = 0;
        self._yytext = Json::Null;
        self._yy = 0;
        self._initial_input = 1;
        self._source = source.chars().collect();
        ;

    }

    #[inline]
    pub fn set_source(&mut self, source: &str) {
        self.restart(source);
    }

    #[inline]
    pub fn advance(&mut self) {
        self._advanced = true;
    }

    #[inline]
    pub fn start_index(&self) -> usize {
        self._old_start
    }

    #[inline]
    pub fn end_index(&self) -> usize {
        self._start
    }

    #[inline]
    pub fn get_position(&self) -> [u64; 2] {
        [self._line, self._col]
    }

    #[inline]
    pub fn new_lexer_error(&self, reason: String) -> CalcLexerError {
        // format!("{}@row:{}col:{}", Self::find_token_name(self._token_index), self._line, self._col)
        CalcLexerError {
            token_name: Self::find_token_name(self._token_index),
            token_index: self._token_index,
            line: self._line,
            col: self._col,
            reason: reason,
        }
    }

    #[inline]
    fn set_yytext(&mut self, value: Json) {
        self._yytext = value;
    }

    #[inline]
    pub fn get_yytext(&mut self) -> Json {
        if self._yytext.is_null() && !self._ended {
            self._yytext = Json::String(self._source[self.start_index() .. self.end_index()].iter().cloned().collect())
        }
        self._yytext.clone()
    }

    #[inline]
    fn get_token_index(&self) -> usize {
        self._token_index
    }

    #[inline]
    fn begin(&mut self, initial_input: usize) {
        self._initial_input = initial_input;
    }

    pub fn get_token(&mut self) -> CalcLexerResult {
        if self._advanced {
            match self.next() {
                Ok(value) => {self._token_index = value;},
                Err(err) => {return Err(err);},
            };
            self._advanced = false;
        }
        Ok(self._token_index)
    }

    fn next(&mut self) -> CalcLexerResult {
        loop {
            let mut _next_state: usize = 0;
            let mut _ch: u64 = 0;
            let mut _next: usize = self._start;
            let mut _och: u64 = u64::MAX;
            let mut _cur_state: usize = self.trans_table[0].to_states[self._initial_input];
            let mut _last_final_state = DEADSTATE;
            let mut _last_final_position = self._start;
            loop {
                if _next < self._source.len() {
                    _ch = self._source[_next] as u64;
                    /* 计算行列位置 */
                    if _och != u64::MAX {
                        if _ch == 0x0d {//\r
                            self._col = 0;
                            self._line += 1;
                        } else if _ch == 0x0a {//\n
                            if _och != 0x0d {//\r
                                self._col = 0;
                                self._line += 1;
                            }
                        } else {
                            self._col += 1;
                        }
                    }

                    _och = _ch;
                    _next_state = match self.trans(_cur_state, _ch) {
                        Ok(st) => { st },
                        Err(err) => { return Err(err); }
                    };
                } else {
                    _next_state = DEADSTATE;
                }

                if _next_state == DEADSTATE {
                    if self._start == _last_final_position {
                        if self._start == self._source.len() {
                            if !self._ended {
                                self._ended = true;
                                return Ok(END_TOKEN);
                            } else {
                                return Err(self.new_lexer_error("aleady at end.".to_string()));
                            }
                        } else {
                            return Err(self.new_lexer_error("invalid char".to_string()));
                        }
                    } else {
                        self._yytext = Json::Null;//set yytext
                        self._old_start = self._start;
                        self._start = _last_final_position;
                        let _findex = self.trans_table[_last_final_state].final_index;
                        match _findex {
0x0 => {
    return Ok(7) /* num */ ;
},
0x1 => {
    return Ok(4) /* + */ ;
},
0x2 => {
    return Ok(5) /* - */ ;
},
0x3 => {
    return Ok(3) /* * */ ;
},
0x4 => {
    return Ok(6) /* / */ ;
},
0x5 => {
    return Ok(1) /* ( */ ;
},
0x6 => {
    return Ok(2) /* ) */ ;
},
_ => {},
}
                        break;
                    }
                } else {
                    _next += 1;
                    if self.trans_table[_next_state].final_index != 0xffffffff {
                        _last_final_state = _next_state;
                        _last_final_position = _next;
                    }
                    _cur_state = _next_state;
                }
            }
        }
    }

    fn find(code: u64, table: &TransTable) -> u64 {
        let mut max = table.len();
        let mut min = 0;

        loop {
            let mid = (max + min) >> 1;
            if table[mid].from <= code {
                if table[mid].to >= code {
                    return table[mid].value;
                } else {
                    min = mid + 1;
                }
            } else {
                max = mid - 1;
            }
        }
    }

    fn trans(&self, cur_state: usize, ch: u64) -> CalcLexerResult {
        if ch < self.input_table[0].from || ch > self.input_table[self.input_table.len() - 1].to {
            return Err(self.new_lexer_error("input char out of valid range".to_string()));
        }
        if self.trans_table[cur_state].is_dead {
            return Ok(DEADSTATE);
        }
        let pub_input = Self::find(ch, &self.input_table);
        let inner_input = Self::find(pub_input, &self.trans_table[cur_state].trans_edge);
        return Ok(self.trans_table[cur_state].to_states[inner_input as usize])
    }

    pub fn lex_seq(source: &str) -> Result<Vec<(&'static str, Token, Json, usize, usize)>, CalcLexerError> {//(token_name, token_index, yytext, star, end)
        let mut lexer = CalcLexer::new();
        lexer.set_source(source);
        let mut tokens = Vec::new();
        let mut token: usize = 0;
        match lexer.get_token() {
            Ok(value) => { token = value; },
            Err(err) => {return Err(err);},
        };
        while !Self::is_end_token(token) {
            tokens.push((Self::find_token_name(token), token, lexer.get_yytext(), lexer.start_index(), lexer.end_index()));
            lexer.advance();
            match lexer.get_token() {
                Ok(value) => { token = value; },
                Err(err) => {return Err(err);},
            };
        }
        Ok(tokens)
    }
}
