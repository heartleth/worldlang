use super::*;

pub fn parse_class(s :&Mem, pivot :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    let code_splited :Vec<String> = split(&s[pivot].code);
    let mut ret = String::new();
    let mut is_classname = false;
    let mut inheriting = true;
    let mut inherits = String::new();

    for elem in &code_splited[1..] {
        if elem == "type" {
            is_classname = true;
            if inheriting {
                inherits.pop();
                inherits.pop();
            }
        }
        else if !is_classname {
            if regi(&elem, "object") {
                inherits.clear();
                inheriting = false;
            }
            else {
                inherits += &elem;
                inherits += jpath!(lang, operator.commas)?;
            }
        }
        else {
            if inheriting {
                ret = render(jpath!(lang, classes.inherit)?, &json!({
                    "name":  &elem,
                    "block": &transpile(&s, pivot, &lang)
                }))?;
            }
            else {
                ret = render(jpath!(lang, classes.Class)?, &json!({
                    "name":  &elem,
                    "super": &inherits,
                    "block": &transpile(&s, pivot, &lang),
                }))?;
            }
        }
    }
    if ret.len() == 0 { Err("no class name") }
    else { Ok(ret) }
}