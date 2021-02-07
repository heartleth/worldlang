use regex;

pub fn split(s:&String)->Vec<String> {
    let re = regex::Regex::new(r"[\s\t\r\n]+").unwrap();
    let mut ret :Vec<String> = Vec::new();
    for part in re.split(&s) {
        ret.push(String::from(part));
    }
    ret
}

pub fn split_token(s:&String, delim:&str)->Vec<String> {
    let re = regex::Regex::new(delim).unwrap();
    let mut ret :Vec<String> = Vec::new();
    for part in re.split(&s) {
        ret.push(String::from(part));
    }
    ret
}

#[inline]
pub fn regi(s:&String, reg:&str)->bool {
    regex::Regex::new(reg).unwrap().is_match(&s.to_lowercase())
}

pub fn keyword(s:&String)->String {
    let k = split(s);
    String::from(&k[0])
}

pub fn is_bracket(s :&String, (start, end) :(char, char))->Result<bool, &'static str> {
    let mut in_string = false;
    let mut escaped = false;
    let mut stack :Vec<char> = Vec::new();

    for elem in s.chars() {
        match elem {
            '\\' => { escaped = in_string && !escaped },
            '"' => { if !escaped { in_string = !in_string; } escaped=false; },
            '(' => if !in_string { stack.push('(') },
            ')' => if !in_string {
                if stack.is_empty() { return Err("Unmatching bracket pair"); }
                else if *stack.last().unwrap() == '(' { stack.pop(); }
                else { return Err("Unmatching bracket pair"); }
            },
            '{' => if !in_string { stack.push('{') },
            '}' => if !in_string {
                if stack.is_empty() { return Err("Unmatching bracket pair"); }
                else if *stack.last().unwrap() == '{' { stack.pop(); }
                else { return Err("Unmatching bracket pair"); }
            },
            _ => { escaped=false; if stack.is_empty() {return Ok(false)} }
        };
    }
    Ok((stack.is_empty())
     && (s.trim().chars().next().unwrap() == start)
     && (s.trim().chars().last().unwrap() == end))
}

pub fn is_string(s :&String)->bool {
    let mut in_string = false;
    let mut escaped = false;

    for elem in s.chars() {
        match elem {
            '\\' => { escaped = in_string && !escaped },
            '"' => { if !escaped { in_string = !in_string; } escaped=false; },
            ' ' | '\n' | '\r' | '\t' => escaped = false,
            _ => { escaped = false; if !in_string { return false; } }
        }
    }
    true
}

pub fn existing_keys(s: &Vec<String>)->Result<Vec<String>, &'static str> {
    let mut ret :Vec<String> = Vec::new();
    let mut stack :Vec<()> = Vec::new();
    let mut in_lambda = false;
    let mut in_string = false;
    let mut escaped = false;

    for elem in s {
        let mut has_bracket = false;
        for i in elem.chars() {
            match i {
                '(' | '{' => if !in_string {stack.push(has_bracket = true)},
                ')' | '}' => if !in_string {
                    if stack.is_empty() {
                        return Err("Unmatching bracket pair");
                    }
                    stack.pop();
                    has_bracket = true;
                },
                '|' => in_lambda = !in_lambda,
                '\\' => { escaped = in_string && !escaped },
                '"' => { if !escaped { in_string = !in_string; } escaped=false; },
                _ => escaped=false
            }
        }
        if !has_bracket && !in_lambda && stack.is_empty() {
            ret.push(String::from(elem));
        }
        else {
            ret.push(String::from("%IGNORED%"));
        }
    }
    Ok(ret)
}

#[inline]
pub fn trim_tail(s :&String)->String {
    String::from(&split_token(&s, "`")[0])
}

#[inline]
pub fn render<T :serde::ser::Serialize>(s :&str, data :&T)->Result<String, &'static str>{
    let mut reg = handlebars::Handlebars::new();
    reg.register_escape_fn(|e|e.to_string());
    let res = reg.render_template(s, data);
    if let Err(_) = res {
        Err("TemplateRenderError")
    }
    else {
        Ok(res.unwrap())
    }
}