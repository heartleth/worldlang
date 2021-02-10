use crate::jpath;
use super::*;

pub fn parse_else(tree :&Mem, nth :&mut usize, parent_idx :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    let parent = &tree[parent_idx];
    let splited = &split(&tree[parent.children[*nth]].code);
    let code = &tree[parent.children[*nth]].code;
    let mut is_trailing = false;
    if *nth < parent.children.len() - 1 && splited.len() > 1 {
        if regi(&keyword(&tree[parent.children[*nth + 1]].code), "^else$") {
            is_trailing = true;
        }
    }

    let mut ret = if splited.len() == 1 {
        render(jpath!(lang, blocks.Else)?, &json!({
            "block": &transpile(&tree, parent.children[*nth], &lang).as_str()
        }))?
    }
    else {
        if is_trailing {
            render(jpath!(lang, blocks.else_if_trailing).unwrap_or(jpath!(lang, blocks.else_if)?), &json!({
                "exp" : &value_parse(&String::from(&code[8..]), 1, &lang)?[..],
                "block": &transpile(&tree, parent.children[*nth], &lang).as_str()
            }))?
        }
        else {
            render(jpath!(lang, blocks.else_if)?, &json!({
                "exp" : &value_parse(&String::from(&code[8..]), 1, &lang)?[..],
                "block": &transpile(&tree, parent.children[*nth], &lang).as_str()
            }))?
        }
    };

    if is_trailing {
        *nth += 1;
        ret += parse_else(&tree, nth, parent_idx, &lang)?.as_str();
    }
    Ok(ret)
}

pub fn parse_if(tree :&Mem, nth :&mut usize, parent_idx :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    let parent = &tree[parent_idx];
    let elem = parent.children[*nth];
    let to_parse = &tree[elem];
    let mut is_trailing = false;
    if *nth < parent.children.len() - 1 {
        if regi(&keyword(&tree[parent.children[*nth + 1]].code), "^else$") {
            is_trailing = true;
        }
    }

    let condition = value_parse(&String::from(&to_parse.code[3..]), 1, &lang)?;
    
    let mut ret = if is_trailing {
        render(jpath!(lang, blocks.if_trailing).unwrap_or(jpath!(lang, blocks.If)?), &json!({
            "exp": &condition[..],
            "block": transpile(&tree, elem, &lang).as_str()
        }))?
    }
    else {
        render(jpath!(lang, blocks.If)?, &json!({
            "exp": &condition[..],
            "block": transpile(&tree, elem, &lang).as_str()
        }))?
    };
    
    if is_trailing {
        *nth += 1;
        ret += parse_else(&tree, nth, parent_idx, &lang)?.as_str();
    }
    Ok(ret)
}

pub fn parse_import(s :&Mem, pivot :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    let method = keyword(&s[pivot].code);
    let mut ret = String::new();
    match &method.to_ascii_lowercase()[..] {
        "import" => {
            use std::path::Path;
            let path = &s[pivot].code[7..];
            if Path::new(&format!("{}.{}", path, jpath!(lang, h_ext)?)).exists() {
                crate::runner::filesys::convert_to_cpp(&String::from(path), jpath!(lang, h_ext)?, &lang).expect("Failed to write file");
                ret = render(jpath!(lang, imports.import)?, &json!({ "path": path }))?;
            }
            else {
                ret = render(jpath!(lang, imports.import)?, &json!({ "path": path }))?;
            }
        },
        "lib" | "library" => {
            if regi(&split(&s[pivot].code)[1], "^(std|standard)$") {
                ret = jpath!(lang, imports.stdlib)?.to_string();
                std::fs::copy(jpath!(lang, imports.stdlibpath)?, jpath!(lang, imports.targetstdlibpath)?)
                    .expect("Failed to copy standard library file");
            }
            else {
                ret = render(jpath!(lang, imports.library)?, &json!({ "path": &split(&s[pivot].code)[1] }))?;
            }
        },
        _=>{}
    };
    Ok(ret)
}