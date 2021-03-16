/***************************************************
 * S A I L O R S A T U R N / T O M O E H O T A R U *
 ***************************************************
 */
//|  |
//|   |                             ____.
//|    | `\`\`|                 ;_=`___  `
// |    |  ``\\             _-``       ````---_
//  \    \     \\         .`                   ````--__
//    \   \      \\      /                             ```-__,
//      \  \      |\    |                                  /
//        \  -_   / |   |    ..  \ .               .   `\ \
//          `-_```  /_  |   /  \\:0:\ _.,           \\ \\ \|
//             `::`` .:  |  |`,o`,   ::"|         \\ |`` ` `
//             ````\  \  |   \ =`  .  ``/o  ,\   |; ``
//                  `\  \.\    \  --`  /   /  \  |
//                   ._| |\`\   \`-_-;`   /  / ||
//                `---\  \  \     \`:o    | / ,:___,
//                  `-=\/``\  \`.,     .  /_`;*:--
//                   -``|   `\ `="=:_,  \ #####/
//                  /_____\   `````` \ .;*=-##\---__,
//                 `      | \   `\``::\   \#######/
//                         \/ \   `\ ##\._       /.
//                           ,  \   `\###-``--_ ( \
//                           |`-.\   |&\###`-._``\_\
//               .           |###\\ /&&&--_#####\. ##-    ,
//                \```--___ /#######\&&&&&&+`--_######\-'|
//                  \######`######._--*===&|    `-#######|  ,
//                   |>###########`--__.`-*``-__   `-######/
//                   |####,--::`########`\\   ||```-..`-.##
//                   |#,*`,-`#############`\  ||#######\ :
//                   |/  /##################\ /#########\ \
//                  `   :.###################i###########`_\
//                        `\==============================

mod operator;
mod sentence;

pub use std::collections::HashMap;
pub use operator::*;
pub use sentence::*;
use super::phrase::*;
use super::types::*;
use super::util::*;
use crate::jpath;
use super::*;

pub fn value_parse(s :&String, level :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    if cfg!(debug_assertions) {
        println!("{}", s);
    }
    let s = &String::from(s.trim());
    if s.len() == 0 { return Ok(String::new()); }
    let mut ret = String::new();
    let mut do_pass = true;
    let list = &split(&s);
    let units = &existing_keys(&split(&s))?;

    if units.len() == 1 {
        if is_string(&s) {
            return Ok(format!("{}", s));
        }
        if units[0].chars().next().unwrap() == '[' {
            let valid = String::from(&s[1..units[0].len() - 1]);
            let range :Vec<String> = split_token(&valid, "\\.");
            let step = range.len() - 1;
            let begin :&String = &range[0];
            let end :&String = &range[step];
            if begin.chars().next().unwrap() == '\'' {
                let begin = begin.chars().nth(1).unwrap() as u32;
                let end = end.chars().nth(1).unwrap() as u32;
                let comma = jpath!(lang, operator.commas)?;
                
                ret += jpath!(lang, bracket.dotarray_open)?;
                for elem in (begin..=end).step_by(step-1) {
                    ret += elem.to_string().as_str();
                    ret += comma;
                }
                ret += jpath!(lang, bracket.dotarray_close)?;
            }
            else {
                let begin = begin.parse::<i32>().unwrap();
                let end = end.parse::<i32>().unwrap();
                let comma = jpath!(lang, operator.commas)?;

                ret += jpath!(lang, bracket.dotarray_open)?;
                for elem in (begin..=end).step_by(step-1) {
                    ret += elem.to_string().as_str();
                    ret += comma;
                }
                ret += jpath!(lang, bracket.dotarray_close)?;
            }
            return Ok(ret);
        }
        if units[0] == "true" {
            return Ok(String::from(jpath!(lang, True)?));
        }
        if units[0] == "false" {
            return Ok(String::from(jpath!(lang, False)?));
        }
        let first :char = units[0].chars().next().unwrap();
        if first.is_numeric() {
            return Ok(s.to_string());
        }
        return render(jpath!(lang, ident)?, &json!({ "name":s }));
    }
    
    if is_bracket(&s, ('(', ')'))? {
        return Ok(format!("{}{}{}", 
            jpath!(lang, bracket.value_open)?,
            &value_parse(&String::from(&s[1..s.len()-1]), 0, &lang)?,
            jpath!(lang, bracket.value_close)?,
        ));
    }
    else if is_bracket(&s, ('{', '}'))? {
        return Ok(format!("{}{}{}", 
            jpath!(lang, bracket.list_open)?,    
            &value_parse(&String::from(&s[1..s.len()-1]), 0, &lang)?,
            jpath!(lang, bracket.list_clode)?
        ));
    }
    if level == 0 {
        left_operator(&mut do_pass, (units, list, "^,$"), &mut |cnt :usize| {
            let (left_s, right_s) = (String::from(&list[..cnt].to_vec().join(" ")), String::from(&list[cnt+1..].to_vec().join(" ")));
            let left = value_parse(&left_s, 0, &lang);
            let right = &value_parse(&right_s, 0, &lang)?;
            ret = format!("{}{comma}{}", left?, right, comma = jpath!(lang, operator.commas)?);
            Ok(())
        })?;
    }
    else if level == 1 {
        if list[0] == "|" {
            do_pass = false;
            let mut cnt = 0;
            for elem in &list[1..].to_vec() {
                cnt += 1;
                if elem == "|" {
                    break;
                }
            }
            let mut vars = String::new();
            vars += &args_to_string(&arguments_parse(&list[1..cnt].to_vec(), &lang)?, &lang);

            ret = render(jpath!(lang, lambdas)?, &json!({
                "args": vars,
                "exp": &value_parse(&list[cnt+1..].to_vec().join(" "), level, &lang)?
            }))?;
        }
    }
    else if level == 2 {
        if regi(&units[0], r"^(make)$") {
            do_pass = false;
            ret = parse_sentence(&format!("it {}", &s[5..]), &lang)?;
        }
        else if regi(&units[0], r"^(result)$") {
            do_pass = false;
            ret = parse_sentence(&String::from(&s[10..]), &lang)?;
        }
        else if regi(&units[0], r"^([wt]hat|\$)$") {
            do_pass = false;
            ret = parse_sentence(&String::from(&s[5..]), &lang)?;
        }
        else if regi(&units[0], r"^(to)$") {
            do_pass = false;
            let func_call = parse_sentence(&format!("it {}", &s[3..]), &lang)?;
            
            ret = render(jpath!(lang, to_lambdas)?, &json!({
                "call": &func_call[..func_call.len()-1],
                "comma": if func_call.chars().nth(func_call.len()-2).unwrap() == '(' {""} else {
                    jpath!(lang, operator.commas)?
                }
            }))?;
        }
    }
    else if level == 3 {
        if regi(&units[0].to_ascii_lowercase(), "^if$") {
            do_pass = false;
            let first = 1 + first_phrase(&list[1..].to_vec(), true, false)?;
            let second = first + 2 + first_phrase(&list[first+2..].to_vec(), true, false)?;
            ret = render(jpath!(lang, tenary)?, &json!({
                "condition": &value_parse(&list[1..first+1].to_vec().join(" "), 1, &lang)?,
                "l": &value_parse(&list[first+2..second+1].to_vec().join(" "), 1, &lang)?,
                "r": &value_parse(&list[second+2..].to_vec().join(" "), 1, &lang)?
            }))?;
        }
    }
    else if level == 4 {
        let mut cnt = units.len();
        for _ in 0..units.len() {
            cnt -= 1;
            let elem = &units[cnt];

            if regi(&elem, r"^(=|as|[a-zA-Z_][a-zA-Z_0-9\-]*=)$") {
                let lport = first_phrase(&list[..cnt].to_vec(), true, false)? + 1;
                if lport != cnt {
                    return Err("SyntaxError: phrase left of the operator 'as' is too short.");
                }

                do_pass = false;
                if regi(&elem, r"^(=|as)$") {
                    ret = render(jpath!(lang, operator.assign)?, &json!({
                        "l": &value_parse(&list[..cnt].to_vec().join(" "), 1, &lang)?,
                        "r": &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?
                    }))?;
                }
                else {
                    let (to_assign_s, argument_s) = (String::from(&list[..cnt].to_vec().join(" ")), &list[cnt+1..].to_vec().join(" "));
                    let to_assign = value_parse(&to_assign_s, 1, &lang)?;
                    let argument = &value_parse(&argument_s, 1, &lang)?;
                    let template_call = jpath!(lang, calls.call)?;
                    let template_assign = jpath!(lang, operator.assign)?;
                    let comma = jpath!(lang, operator.commas)?;

                    ret = render(template_assign, &json!({
                        "l": &to_assign[..],
                        "r": render(template_call, &json!({
                            "method": &elem[..elem.len()-1],
                            "args": &format!("{}{}{}", &to_assign[..], comma, argument)[..]
                        }))?
                    }))?;
                }
            }
        }
    }
    else if level == 5 {
        left_operator(&mut do_pass, (units, list, "^(or)?or$"), &mut |cnt :usize| {
            if regi(&list[cnt], "^oror$") {
                ret = render(jpath!(lang, operator.or)?, &json!({
                    "l": &value_parse(&list[..cnt].to_vec().join(" "), 1, &lang)?,
                    "r": &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?
                }))?;
            }
            else {
                ret = render(jpath!(lang, operator.bitor)?, &json!({
                    "l": &value_parse(&list[..cnt].to_vec().join(" "), 1, &lang)?,
                    "r": &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?
                }))?;
            }
            Ok(())
        })?;
    }
    else if level == 6 {
        left_operator(&mut do_pass, (units, list, "^(and)?and$"), &mut |cnt :usize| {
            if regi(&list[cnt], "^andand$") {
                ret = render(jpath!(lang, operator.and)?, &json!({
                    "l": &value_parse(&list[..cnt].to_vec().join(" "), 1, &lang)?,
                    "r": &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?
                }))?;
            }
            else {
                ret = render(jpath!(lang, operator.bitand)?, &json!({
                    "l": &value_parse(&list[..cnt].to_vec().join(" "), 1, &lang)?,
                    "r": &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?
                }))?;
            }
            Ok(())
        })?;
    }
    else if level == 7 {
        left_operator(&mut do_pass, (units, list, "^(is(not)?|[<>]=?)$"), &mut |cnt :usize| {
            if regi(&list[cnt], r"^is$") {
                ret = render(jpath!(lang, operator.eq)?, &json!({
                    "l": &value_parse(&list[..cnt].to_vec().join(" "), 1, &lang)?,
                    "r": &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?
                }))?;
            }
            else if regi(&list[cnt], r"^isnot$") {
                ret = render(jpath!(lang, operator.neq)?, &json!({
                    "l": &value_parse(&list[..cnt].to_vec().join(" "), 1, &lang)?,
                    "r": &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?
                }))?;
            }
            else {
                ret = render(jpath!(lang, operator.Else)?, &json!({
                    "l": &value_parse(&list[..cnt].to_vec().join(" "), 1, &lang)?,
                    "r": &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?,
                    "operator": &units[cnt]
                }))?;
            }
            Ok(())
        })?;
    }
    else if level == 8 {
        left_operator(&mut do_pass, (units, list, "^([+-]|plus|minus)$"), &mut |cnt :usize| {
            let left_s = String::from(&list[..cnt].to_vec().join(" "));
            let left = value_parse(&left_s, 1, &lang);
            if regi(&units[cnt], "^(plus|\\+)$") {
                ret = render(jpath!(lang, operator.plus)?, &json!({
                    "l": left?,
                    "r": &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?,
                }))?;
            }
            else if regi(&units[cnt], "^(minus|-)$") {
                ret = render(jpath!(lang, operator.minus)?, &json!({
                    "l": left?,
                    "r": &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?,
                }))?;
            }
            Ok(())
        })?;
    }
    else if level == 9 {
        left_operator(&mut do_pass, (units, list, "^([/*%])$"), &mut |cnt :usize| {
            ret = render(jpath!(lang, operator.Else)?, &json!({
                "l": &value_parse(&list[..cnt].to_vec().join(" "), 1, &lang)?,
                "r": &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?,
                "operator": &units[cnt]
            }))?;
            Ok(())
        })?;
    }
    else if level == 10 {
        if regi(&units[0], "^(await)$") {
            do_pass = false;
            ret = render(jpath!(lang, operator.Await)?, &json!({
                "exp": &value_parse(&list[1..].to_vec().join(" "), 1, &lang)?
            }))?;
        }
        else if regi(&units[0], "^(async)$") {
            do_pass = false;
            ret = render(jpath!(lang, operator.Async)?, &json!({
                "exp": &value_parse(&list[1..].to_vec().join(" "), 1, &lang)?
            }))?;
        }
        else {
            left_operator(&mut do_pass, (units, list, r"^[a-zA-Z_][a-zA-Z_0-9\-]*!$"), &mut |cnt: usize| {
                let left_s = list[..cnt].to_vec().join(" ");
                let left = value_parse(&left_s, 1, &lang);

                ret = render(jpath!(lang, calls.call)?, &json!({
                    "method": &verb_parse(&String::from(&units[cnt][..&units[cnt].len() - 1]), &lang),
                    "args": format!("{}{}{}",
                        left?,
                        jpath!(lang, operator.commas)?,
                        &value_parse(&list[cnt+1..].to_vec().join(" "), 1, &lang)?
                    )
                }))?;
                Ok(())
            })?;
        }
    }
    else if level == 11 {
        left_operator(&mut do_pass, (units, list, "^:$"), &mut |cnt :usize| {
            ret = render(jpath!(lang, operator.pair)?, &json!({
                "l": &value_parse(&list[..cnt].to_vec().join(" "), level, &lang)?,
                "r": &value_parse(&list[cnt+1..].to_vec().join(" "), level, &lang)?,
            }))?;
            Ok(())
        })?;
    }
    else if level == 12 {
        if regi(&units[1], "^(to|of|with|about|for|:|->)$") {
            do_pass = false;
            let func_name = &verb_parse(&units[0], &lang);
            if regi(&units[0], r"^(value)$") {
                ret = render(jpath!(lang, operator.de_pointer)?, &json!({
                    "exp": &value_parse(&list[2..].to_vec().join(" "), 1, &lang)?
                }))?;
            }
            else if regi(&units[0], "^(addr(ess)?|pointer|ptr)$") {
                ret = render(jpath!(lang, operator.pointer)?, &json!({
                    "exp": &value_parse(&list[2..].to_vec().join(" "), 1, &lang)?
                }))?;
            }
            else {
                let to_be_evaluated = &value_parse(&list[2..].to_vec().join(" "), 1, &lang)?;
                ret = render(jpath!(lang, calls.call)?, &json!({
                    "method": func_name,
                    "args": to_be_evaluated
                }))?;
            }
        }
    }
    else if level == 13 {
        if regi(&units[0], r"^[a-zA-Z_][a-zA-Z_0-9\-]*:$") {
            do_pass = false;
            let func_name = &verb_parse(&String::from(&units[0][..&units[0].len()-1]), &lang);
            let to_be_evaluated = &value_parse(&list[1..].to_vec().join(" "), 0, &lang)?;
            if is_bracket(&list[1..].to_vec().join(" "), ('(', ')'))? {
                ret = render(jpath!(lang, calls.call)?, &json!({
                    "method": func_name,
                    "args": to_be_evaluated[jpath!(lang, bracket.value_open)?.len()..to_be_evaluated.len()-jpath!(lang, bracket.value_close)?.len()]
                }))?;
            }
            else {
                ret = render(jpath!(lang, calls.call)?, &json!({
                    "method": func_name,
                    "args": to_be_evaluated
                }))?;
            }
        }
    }
    else if level == 14 {
        left_operator(&mut do_pass, (units, list, r"^(was|were|do)$"), &mut |cnt :usize| {
            if regi(&units[cnt], "^(do)$") {
                let call_args = parse_sentence_struct(&format!("it {}", &units[cnt + 1..].to_vec().join(" ")), &lang)?;
                ret = render(jpath!(lang, calls.method_call)?, &json!({
                    "object": &value_parse(&list[0..cnt].to_vec().join(" "), 1, &lang)?,
                    "call": &parse_sentence(&format!("it {}", &units[cnt + 1..].to_vec().join(" ")), &lang),
                    "method": &(call_args.1),
                    "args": &(call_args.2),
                }))?;
            }
            else {
                let prep = units.get(cnt + 2);
                let has_prep =
                    if let Some(txt) = prep { regi(&txt, "^(to|of|with|about|for|:|->)$")  }
                    else { false };
                if has_prep {
                    ret = render(jpath!(lang, calls.call)?, &json!({
                        "method": &verb_parse(&units[cnt+1], &lang),
                        "args": format!("{}{}{}",
                            &value_parse(&list[..cnt].to_vec().join(" "), 0, &lang)?,
                            jpath!(lang, operator.commas)?,
                            &value_parse(&list[cnt+3..].to_vec().join(" "), 1, &lang)?
                        )
                    }))?;
                }
                else {
                    ret = render(jpath!(lang, calls.call)?, &json!({
                        "method": &verb_parse(&units[cnt+1], &lang),
                        "args": &value_parse(&list[..cnt].to_vec().join(" "), 1, &lang)?
                    }))?;
                }
            }
            Ok(())
        })?;
    }
    else if level == 15 {
        if regi(&units[units.len()-2], r"^(in)$") {
            do_pass = false;
            ret = render(jpath!(lang, operator.context)?, &json!({
                "context": &units[units.len()-1],
                "member": &value_parse(&list[0..units.len()-2].to_vec().join(" "), 1, &lang)?
            }))?;
        }
    }
    else if level == 16 {
        if regi(&units[units.len()-2], r"^(having)$") {
            do_pass = false;
            ret = render(jpath!(lang, operator.property)?, &json!({
                "object": &units[units.len()-1],
                "name": &value_parse(&list[0..units.len()-2].to_vec().join(" "), 1, &lang)?
            }))?;
        }
        else if regi(&units[units.len()-2], r"'s$") {
            do_pass = false;
            ret = render(jpath!(lang, operator.property)?, &json!({
                "object": &units[units.len()-2][0..&units[units.len()-2].len()-2],
                "name": &value_parse(&list[1..units.len()].to_vec().join(" "), 1, &lang)?
            }))?;
        }
    }
    else if level == 17 {
        left_operator(&mut do_pass, (units, list, "^#$"), &mut |cnt :usize| {
            ret = render(jpath!(lang, operator.property)?, &json!({
                "exp": &value_parse(&list[..cnt].to_vec().join(" "), level, &lang)?,
                "at": &value_parse(&list[cnt+1..].to_vec().join(" "), level, &lang)?
            }))?;
            Ok(())
        })?;
    }
    else if level > 17 {
        if is_string(&s) {
            return Ok(format!("{}", s));
        }
        else {
            return Err("SyntaxError: no matching operator");
        }
    }

    if do_pass { return value_parse(&s, level + 1, &lang); }

    Ok(ret)
}