use super::util::*;
pub use clause::*;
use super::*;

pub fn first_phrase(s :&Vec<String>, _is_first :bool, allow_multi :bool)->Result<usize, &'static str> {
    // if cfg!(debug_assertions) {
    //     println!("{}", s.join(" "));
    // }
    let operators = if allow_multi {
        r"^(,|[#:]|and|(or)?or|plus|minus|=|is(not)?|as|[+\-*/%]|<<|>>|&|[><]|[a-zA-Z_][a-zA-Z0-9\-_]*[=!]|having|was|were|do|in)$"
    } else {
        r"^([#:]|and|(or)?or|plus|minus|=|is(not)?|as|[+\-*/%]|<<|>>|&|[><]|[a-zA-Z_][a-zA-Z0-9\-_]*[=!]|having|was|were|do|in)$"
    };
    
    let mut ret = 0;
    let len = s.len();
    if len == 1 {
        return Ok(0);
    }
    else if len == 0 {
        return Err("-1");
    }
    else {
        let first_low = s[0].to_ascii_lowercase();
        if is_bracket(&s.join(" "), ('(', ')'))? || is_bracket(&s.join(" "), ('{', '}'))? {
            let inner_phrase = first_phrase(&s[1..len - 1].to_vec(), true, true);
            if let Err(e) = inner_phrase {
                if e == "-1" {
                    return Ok(1);
                }
            }

            if inner_phrase? == len - 3 {
                return Ok(len - 1);
            }
            else {
                return Err("wrong phrase in bracket");
            }
        }
        else if is_string(&s.join(" ")) {
            return Ok(len - 1);
        }
        else if first_low == "|" {
            for elem in 1 .. s.len() - 1 {
                if s[elem] == "|" {
                    ret = elem + 1;
                    return Ok(ret + first_phrase(&s[ret..].to_vec(), true, allow_multi)?);
                }
            }
        }
        else if regi(&first_low, "^if$") {
            let mut expression = first_phrase(&s[1..].to_vec(), true, allow_multi)?;
            expression += 1;
            expression += first_phrase(&s[expression+2..].to_vec(), true, allow_multi)?;
            expression += 2;
            ret = expression;
            ret += first_phrase(&s[expression+2..].to_vec(), true, allow_multi)?;
            ret += 2;
        }
        else if regi(&first_low, "^(result)$") {
            ret = 2;
            ret += first_clause(&s[2..].to_vec())?;
            if ret < s.len()-1 {
                ret += first_phrase(&s[ret+1..].to_vec(), false, false)? + 1;
            }
        }
        else if regi(&first_low, r"^(\$|[tw]hat)$") {
            ret = 1;
            ret += first_clause(&s[1..].to_vec())?;
            if ret < s.len()-1 {
                ret += first_phrase(&s[ret+1..].to_vec(), false, false)? + 1;
            }
        }
        else if regi(&first_low, "^(make|to)$") {
            let to_give = [vec![String::from("a ")], s[1..].to_vec()].concat();
            ret = first_clause(&to_give)?;
            if ret < s.len()-1 {
                ret += first_phrase(&s[ret+1..].to_vec(), false, false)? + 1;
            }
        }
        else if regi(&first_low, "'s$") {
            ret = 1;
            if ret < s.len()-1 {
                ret += first_phrase(&s[1..].to_vec(), false, false)?;
            }
        }
        else if regi(&first_low, "^(value|addr(ess)?|ptr|pointer)$") {
            ret = 2;
            if ret < s.len()-1 {
                ret += first_phrase(&s[2..].to_vec(), false, false)?;
            }
        }
        else if regi(&s[1], "^(to|of|with|about|for|:|->)$") {
            ret = 2;
            ret += first_phrase(&s[2..].to_vec(), false, true)?;
        }
        else if regi(&first_low, r"^([a-zA-Z_][a-zA-Z_0-9\-]*:|await|async)$") {
            ret = 1;
            ret += first_phrase(&s[1..].to_vec(), false, true)?;
        }
        else if regi(&s[1], operators) {
            ret = 1 + 1;
            if regi(&s[1], "^(do|was|were)$") {
                if ret + 1 < s.len() {
                    if regi(&s[ret+1], "^(to|of|with|about|for|:|->)$") {
                        ret += first_clause(&[&[String::from("it")], &s[1+1..].to_vec()[..]].concat())?-1;
                    }
                }
            }
            ret += first_phrase(&s[ret..].to_vec(), false, allow_multi)?;
        }
        else {
            let ignores = existing_keys(&s)?;
            let mut breaker = 0;
            let mut is_breaked = false;
            for elem in &ignores {
                if regi(&elem, operators) {
                    is_breaked = true;
                    break;
                }
                else { breaker += 1; }
            }
            if !is_breaked || breaker == s.len()-1 {
                let mut stack :Vec<()> = Vec::new();
                let mut i = 0;
                for elem in s {
                    match &elem[..] {
                        "(" => stack.push(()),
                        ")" => {stack.pop();},
                        _ => {}
                    }
                    if stack.is_empty() {
                        return Ok(i);
                    }
                    i+=1;
                }
            }
            else {
                let lport = first_phrase(&s[..breaker].to_vec(), true, allow_multi)?;
                if lport != breaker - 1 {
                    return Ok(lport);
                }
                ret = breaker;
                if regi(&s[breaker], "^(do|was|were)$") {
                    if ret + 1 < s.len() {
                        if regi(&s[breaker+1], "^(to|of|with|about|for|:|->)$") {
                            ret += 1 + first_clause(&[&[String::from("it")], &s[breaker+1..].to_vec()[..]].concat())?;
                        }
                    }
                }
                ret += first_phrase(&s[ret+1..].to_vec(), true, allow_multi)?;
                ret += 1;
            }
        }
    }

    Ok(ret)
}