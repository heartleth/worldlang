use super::*;

#[inline]
pub fn parse_return(s :&String, lang :&json::JsonValue)->Result<String, &'static str> {
    render(jpath!(lang, returns)?, &json!({ "exp": &value_parse(&String::from(&s[7..]), 1, &lang)? }))
}

#[inline]
pub fn parse_namespace(s :&Mem, pivot :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    render(jpath!(lang, namespaces)?, &json!({
        "name": &verb_parse(&String::from(s[pivot].code[10..].trim()), &lang),
        "block": transpile(s, pivot, &lang)
    }))
}

#[inline]
pub fn parse_access(s :&Mem, pivot :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    Ok(format!("{} {}", match &s[pivot].code.to_ascii_lowercase()[..] {
        "protected" => jpath!(lang, access.protected),
        "private" => jpath!(lang, access.private),
        "public" => jpath!(lang, access.public),
        _=>Err("...")
    }?, transpile(s, pivot, &lang)))
}

#[inline]
pub fn parse_set(s :&String, lang :&json::JsonValue)->Result<String, &'static str> {
    Ok(format!("{}{}", value_parse(&String::from(&s[4..]), 3, &lang)?, jpath!(lang, line_end)?))
}