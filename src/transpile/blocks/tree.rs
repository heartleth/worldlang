#[derive(Debug)]
pub struct CodeTree {
    pub code:String,
    pub line:usize,
    pub children:Vec<usize>
}

impl CodeTree {
    pub fn new(s:&str, l:usize)->CodeTree {
        CodeTree {
            code: String::from(s),
            line: l,
            children: Vec::new()
        }
    }
}

pub mod filter {
    fn is_empty(s:&str)->bool {
        String::from(s).trim().len() == 0
    }
    
    static mut INDEDT_TYPE:char = '\t';
    static mut INDENT_SIZE:usize = 0;
    static mut IS_FIRST:bool = true;

    pub fn get_indents(s:&str)->i32 {
        if is_empty(s) {
            return -1;
        }
        
        if &s[0..1] != " " && &s[0..1] != "\t" {
            return 0;
        }
        let mut ret:i32 = 0;
        unsafe {
            if IS_FIRST && (&s[0..1] == " " || &s[0..1] == "\t") {
                for elem in s.as_bytes() {
                    let c = *elem as char;
                    if c != ' ' && c != '\t' {
                        break;
                    }
                    INDENT_SIZE += 1;
                }
                INDEDT_TYPE = s.as_bytes()[0] as char;
                IS_FIRST = false;
                
                return 1;
            }
            else {
                for elem in s.chars() {
                    let c = elem;
                    if c != INDEDT_TYPE {
                        break;
                    }
                    ret += 1;
                }
                ret /= INDENT_SIZE as i32;
            }
        }
        return ret;
    }

    pub fn filter(s :&str)->String {
        let mut ret = String::new();
        let mut in_string = false;
        let mut escaped = false;

        for elem in s.chars() {
            match elem {
                '(' | ')' | '+' | '*' | '/' | ',' | '|' | '{' | '}' => {
                    if !in_string {
                        ret.push(' ');
                        ret.push(elem);
                        ret.push(' ');
                    }
                    else {
                        ret.push(elem);
                    }
                },
                ':' => {
                    if !in_string {
                        ret.push(elem);
                        ret.push(' ');
                    }
                    else {
                        ret.push(elem);
                    }
                },
                '"' => {
                    if !escaped {
                        in_string = !in_string;
                    }
                    escaped=false;
                    ret.push('"');
                },
                '\\' => { escaped = in_string && !escaped; ret.push('\\'); },
                ';' => {if in_string{ret.push(';')}else {return ret}},
                _ => {
                    ret.push(elem);
                    escaped = false;
                }
            };
        }
        ret
    }
}

fn count_brackets(s :&str, brackets :&mut Vec<char>) {
    let mut in_string = false;
    let mut escaped = false;
    for elem in s.chars() {
        match elem {
            '(' => if !in_string { brackets.push('('); },
            ')' => if !in_string { if brackets.last().unwrap() == &'(' { brackets.pop(); } },
            '{' => if !in_string { brackets.push('{'); },
            '}' => if !in_string { if brackets.last().unwrap() == &'{' { brackets.pop(); } },
            '"' => {
                if !escaped { in_string = !in_string }
                escaped=false;
            },
            '\\' => { escaped = in_string && !escaped; },
            _ => escaped=false
        };
    }
}

impl CodeTree {
    pub fn treeify(s :&String)->Vec<CodeTree> {
        let mut mem :Vec<CodeTree> = vec!(CodeTree::new("root", 0));
        let mut stack :Vec<usize> = vec!(0);
        let mut brackets :Vec<char> = Vec::new();

        let mut before_indents:usize = 0;
        let mut line:usize = 0;

        for elem in s.split('\n') {
            line += 1;
            let indents = filter::get_indents(&elem);
            if indents >= 0 {
                if brackets.is_empty() {
                    let indents = indents as usize;
                    let top = stack.len() - 1;
                    
                    let diff = indents as i32 - before_indents as i32;
                    if diff > 0 {
                        let last = mem[stack[top]].children.len() - 1;
                        stack.push(mem[stack[top]].children[last]);
                    }
                    else if diff < 0 {
                        for _ in indents .. before_indents {
                            stack.pop();
                        }
                    }
                
                    let top = stack.len() - 1;
                    let code = &filter::filter(elem.trim())[..];
                    mem.push(CodeTree::new(code, line));
                    let back_memory = mem.len() - 1;

                    count_brackets(&code, &mut brackets);

                    mem[stack[top]].children.push(back_memory);
                    before_indents = indents;
                }
                else {
                    mem.last_mut().unwrap().code += &filter::filter(elem.trim())[..];
                    count_brackets(&elem.trim(), &mut brackets);
                }
            }
        }
        mem
    }
}