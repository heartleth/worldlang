use rand::distributions::Alphanumeric;
extern crate rand;
use rand::Rng;
use super::*;
use crate::jpath;


pub fn parse_repeat(tree :&Mem, pivot :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    let s = format!("q{}", rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(20)
        .collect::<String>());
        
    render(jpath!(lang, blocks.repeat)?, &json!({
        "var": s,
        "time": &value_parse(&String::from(&tree[pivot].code[7..]), 1, &lang)?,
        "block": &transpile(&tree, pivot, &lang)
    }))
}

#[inline]
pub fn parse_while(tree :&Mem, pivot :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    render(jpath!(lang, blocks.While)?, &json!({
        "exp": &value_parse(&String::from(&tree[pivot].code[6..]), 1, &lang)?,
        "block": &transpile(&tree, pivot, &lang)
    }))
}

pub fn parse_for(tree :&Mem, pivot :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    let splited = &split(&tree[pivot].code);
    let mut where_of = 0;
    
    for elem in splited {
        if elem.to_ascii_lowercase() == "of" { break; }
        else { where_of += 1; }
    }
    let mut to_assign = render(jpath!(lang, types.iter_vars)?, &json!({
        "type": jpath!(lang, types.deduce_type)?,
        "name": "_"
    }))?;
    let dec = arguments_parse(&splited[1..where_of].to_vec(), &lang)?;

    if dec.len() == 1 {
        to_assign = render(jpath!(lang, types.iter_vars)?, &json!({
            "type": dec[0].typename,
            "name": dec[0].name
        }))?;
    }
    else if jpath!(lang, name) == Ok("C++") {
        to_assign = String::from("auto [");
        for elem in &dec {
            to_assign += &elem.name[..];
            to_assign += ", ";
        }
        to_assign.pop();
        to_assign.pop();
        to_assign += "]";
    }
    
    render(jpath!(lang, blocks.foreach)?, &json!({
        "var": to_assign,
        "collection": &value_parse(&splited[where_of+1..].to_vec().join(" "), 1, &lang)?,
        "block": &transpile(&tree, pivot, &lang)
    }))
}