use super::*;

#[inline]
pub fn parse_use(s :&Mem, pivot :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    render(jpath!(lang, calls.call)?, &json!({
        "method": &verb_parse(&split(&s[pivot].code)[1], &lang),
        "args": render(jpath!(lang, lambda_blocks)?, &json!({
            "args": "",
            "block": transpile(s, pivot, &lang)
        }))
    }))
}