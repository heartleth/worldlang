use crate::jpath;

#[derive(Debug)]
pub struct Variable {
    pub typename :String,
    pub name :String
}

impl Variable {
    pub fn to_string(&self, lang :&json::JsonValue)->String {
        crate::blocks::render(jpath!(lang, arg_vars).unwrap_or("{} {}"), &json!({
            "type": self.typename,
            "name": self.name
        })).unwrap_or(String::new())
    }
}

pub fn type_parse(s :&Vec<String>, lang :&json::JsonValue)->String {
    let mut ret = String::new();
    for elem in s {
        ret += &format!("{} ", match &elem.to_ascii_lowercase()[..] {
            "integer" => &jpath!(lang, types.integer).unwrap_or(""),
            "constant" => &jpath!(lang, types.constant).unwrap_or(""),
            _ => &elem[..]
        })[..];
    }
    ret
}

pub fn declarition_parse(s :&Vec<String>, lang :&json::JsonValue)->Result<Variable, &'static str> {
    let last = s.len()-1;
    if s.len() == 0 { return Err("No variable name."); }
    let mut ret = Variable{
        typename: type_parse(&s[0..s.len()-1].to_vec(), &lang),
        name: String::from(&s[last])
    };
    if last == 0 { ret.typename=String::from(jpath!(lang, types.deduce_type)?); }
    Ok(ret)
}

pub fn arguments_parse(s :&Vec<String>, lang :&json::JsonValue)->Result<Vec<Variable>, &'static str> {
    let mut ret :Vec<Variable> = Vec::new();
    let mut begin = 0;
    let mut end = 0;

    for elem in s {
        if elem == "," {
            ret.push(declarition_parse(&s[begin..end].to_vec(), &lang)?);
            begin = end + 1;
        }
        end += 1;
    }

    if s.len() > 0 {
        ret.push(declarition_parse(&s[begin..].to_vec(), &lang)?);
    }

    Ok(ret)
}

pub fn args_to_string(args :&Vec<Variable>, lang :&json::JsonValue)->String {
    let mut ret = String::new();
    let comma = jpath!(lang, operator.commas).unwrap_or(",");
    for elem in args {
        ret += &elem.to_string(&lang);
        ret += comma;
    }
    if args.len() > 0 {
        for _ in comma.chars() {
            ret.pop();
        }
    }
    ret
}

pub fn args_types_to_string(args :&Vec<Variable>, lang :&json::JsonValue)->String {
    let mut ret = String::new();
    let comma = jpath!(lang, operator.commas).unwrap_or(",");
    for elem in args {
        ret += &elem.typename;
        ret += comma;
    }
    if args.len() > 0 {
        for _ in comma.chars() {
            ret.pop();
        }
    }
    ret
}

pub fn args_names_to_string(args :&Vec<Variable>, lang :&json::JsonValue)->String {
    let mut ret = String::new();
    let comma = jpath!(lang, operator.commas).unwrap_or(",");
    for elem in args {
        ret += &elem.name;
        ret += comma;
    }
    if args.len() > 0 {
        for _ in comma.chars() {
            ret.pop();
        }
    }
    ret
}