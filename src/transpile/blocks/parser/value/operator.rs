use super::*;

pub fn left_operator<T>(
    do_pass :&mut bool,
    (units, list, reg) :(&Vec<String>, &Vec<String>, &str),
    functor :&mut T)->Result<(), &'static str>
    where T : FnMut(usize)->Result<(), &'static str> {

    let mut stack :Vec<char> = Vec::new();
    let mut in_string = false;
    let mut escaped = false;
    let mut cnt = 0;

    for _ in 0..units.len() {
        let elem = &units[cnt];
        for e in elem.chars() {
            match e {
                '\\' => { escaped = in_string && !escaped },
                '"' => { if !escaped { in_string = !in_string; } escaped=false; },
                '(' => if !in_string { stack.push('(') },
                ')' => if !in_string {
                    if stack.is_empty() { return Err("Unmatching bracket pairs"); }
                    else if *stack.last().unwrap() == '(' { stack.pop(); }
                    else { return Err("Unmatching bracket pairs"); }
                },
                '{' => if !in_string { stack.push('{') },
                '}' => if !in_string {
                    if stack.is_empty() { return Err("Unmatching bracket pairs"); }
                    else if *stack.last().unwrap() == '{' { stack.pop(); }
                    else { return Err("Unmatching bracket pairs"); }
                },
                _ => { escaped=false; }
            };
        }

        if regi(&elem, reg) && !in_string {
            *do_pass = false;
            let lport = first_phrase(&list[..cnt].to_vec(), true, true)? + 1;
            if lport != cnt {
                return Err("SyntaxError: phrase left of the operator is too short.");
            }
            functor(cnt)?;
        }
        cnt += 1;
    }
    Ok(())
}