use super::*;

pub fn parse_sentence(s :&String, lang :&json::JsonValue)->Result<String, &'static str> {
    let ret;
    let spl = split(&s);
    let mut has_subject = true;
    
    if s.len() <= 1 { return Err("wrong clause"); }
    let subject_idx = first_phrase(&spl, true, false)?;
    let mut subject = value_parse(&spl[..=subject_idx].to_vec().join(" "), 1, &lang)?;

    if subject.to_ascii_lowercase() == "it" {
        subject = String::new();
        has_subject = false;
    }
    
    if spl.len() > subject_idx + 2 && regi(&spl[subject_idx + 2], "^(->|(to|of|with|about|for|:|->)|about|for|:)$") {
        if has_subject {
            ret = render(jpath!(lang, calls.method_call)?, &json!({
                "object": &subject,
                "call": render(jpath!(lang, calls.call)?, &json!({
                    "method": &spl[subject_idx + 1],
                    "args": &value_parse(&spl[subject_idx+3..].to_vec().join(" "), 0, &lang)?
                }))?,
                "method": &spl[subject_idx + 1],
                "args": &value_parse(&spl[subject_idx+3..].to_vec().join(" "), 0, &lang)?
            }))?;
        }
        else {
            ret = render(jpath!(lang, calls.call)?, &json!({
                "method": &spl[subject_idx + 1],
                "args": &value_parse(&spl[subject_idx+3..].to_vec().join(" "), 0, &lang)?
            }))?;
        }
    }
    else {
        if has_subject {
            ret = render(jpath!(lang, calls.method_call)?, &json!({
                "object": &subject,
                "call": render(jpath!(lang, calls.call)?, &json!({
                    "method": &spl[subject_idx + 1],
                    "args": ""
                }))?,
                "method": &spl[subject_idx + 1],
                "args": ""
            }))?;
        }
        else {
            ret = render(jpath!(lang, calls.call)?, &json!({
                "method": &spl[subject_idx + 1],
                "args": ""
            }))?;
        }
    }
    Ok(ret)
}