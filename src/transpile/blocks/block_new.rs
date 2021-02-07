use super::*;

pub fn parse_new(s :&String, lang :&json::JsonValue)->Result<String, &'static str> {
    #[derive(Debug)]
    enum InitType {
        Constructor, RefType, CopyType, _None
    }
    #[derive(Debug)]
    enum DeclareType {
        Make,
        Have,
        Let
    }
    use DeclareType::*;
    use InitType::*;

    let keyword = &keyword(&s).to_ascii_lowercase()[..];
    let splited = &split(&s);
    let init_type :InitType;
    let make_type :DeclareType = match &keyword[..] {
        "have" => Have,
        "let" => Let,
        "make" => Make,
        _ => Let
    };

    let mut where_as = 1;
    for elem in &splited[1..] {
        if regi(&elem, "^(as|is|:|->|for|of|to|with|about)$") { break; }
        else { where_as += 1; }
    }

    if where_as == splited.len() {
        init_type = InitType::_None;
    }
    else {
        init_type = match &splited[where_as].to_ascii_lowercase()[..] {
            ":" | "->" |
            "to" | "of" |
            "for" | "about" |
            "with" => InitType::Constructor,
            "is" => InitType::RefType,
            "as" => InitType::CopyType,
            _ => InitType::_None
        };
    }

    let var = &declarition_parse(&splited[1..where_as].to_vec(), &lang)?;

    let ret = match init_type {
        Constructor => {
            let data = &json!({
                "type": &var.typename,
                "name": &var.name,
                "args": &value_parse(&splited[where_as+1..].to_vec().join(" "), 0, &lang)?
            });
            match make_type {
                Have => render(jpath!(lang, assigns.construct.Const)?, data)?,
                Let => render(jpath!(lang, assigns.construct.Let)?, data)?,
                Make => render(jpath!(lang, assigns.construct.make)?, data)?
            }
        },
        CopyType => {
            let data = &json!({
                "type": &var.typename,
                "name": &var.name,
                "copy": &value_parse(&splited[where_as+1..].to_vec().join(" "), 1, &lang)?
            });
            match make_type {
                Have => render(jpath!(lang, assigns.construct.Const)?, data)?,
                Let => render(jpath!(lang, assigns.construct.Let)?, data)?,
                Make => render(jpath!(lang, assigns.construct.make)?, data)?,
            }
        },
        RefType => {
            let data = &json!({
                "type": &var.typename,
                "name": &var.name,
                "ref": &value_parse(&splited[where_as+1..].to_vec().join(" "), 1, &lang)?
            });
            match make_type {
                Have => render(jpath!(lang, assigns.reference.Const)?, data)?,
                Let => render(jpath!(lang, assigns.reference.Let)?, data)?,
                Make => render(jpath!(lang, assigns.reference.make)?, data)?,
            }
        }
        _None => var.to_string(&lang)
    };
    Ok(ret + jpath!(lang, line_end)?)
}