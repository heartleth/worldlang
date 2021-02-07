use crate::jpath;
use super::*;

pub static mut IS_DYNAMIC :bool = false;

pub fn parse_when(tree :&Mem, pivot :usize, lang :&json::JsonValue)->Result<String, &'static str> {
    let code = &tree[pivot].code;
    let splited = split(&code);

    if splited[1].to_ascii_lowercase() == "it" && splited[2].to_ascii_lowercase() == "starts" {
        render(jpath!(lang, funcs.entry)?, &json!({
            "block": transpile(&tree, pivot, &lang)
        }))
    }
    else {
        match &splited[2].to_ascii_lowercase()[..] {
            "created" => render(jpath!(lang, funcs.constructor)?, &json!({
                "class": &splited[1],
                "block": transpile(&tree, pivot, &lang),
                "args": args_to_string(&arguments_parse(&splited[3..].to_vec(), &lang)?, &lang)
            })),
            _ => {
                let func_name = &splited[2];
                let return_type :String;
                let mut func_is_dynamic = false;
                let mut func_is_const = false;
                let mut where_return = 0;
                let mut where_is = 0;

                for elem in &splited {
                    if regi(&elem, "returns?") {
                        break;
                    }
                    else {
                        where_return += 1;
                    }
                }
                for elem in &splited {
                    if regi(&elem, "is") {
                        break;
                    }
                    else {
                        where_is += 1;
                    }
                }
                if where_return == splited.len() {
                    return_type = String::from(jpath!(lang, types.void_type)?);
                }
                else {
                    return_type = type_parse(&splited[where_return+1..where_is].to_vec(), &lang);
                }
                
                let where_args_end = if where_return > where_is { where_is } else { where_return };
                let args = &arguments_parse(&splited[3..where_args_end].to_vec(), &lang)?;

                unsafe { IS_DYNAMIC = func_is_dynamic }

                for elem in &splited[where_is..] {
                    if regi(&elem, "dynamic") {
                        func_is_dynamic = true
                    }
                    if regi(&elem, "const") {
                        func_is_const = true
                    }
                }

                if func_is_dynamic && jpath!(lang, name)? == "C++" {
                    
                    Ok(format!("{retType} {funcName}({args}) {cst} {{
static std::map<std::tuple<{argTypes}>, {retType}> __MEMOI;
auto __MEMOI_ELEMENT=std::tuple<{argTypes}>({argNames});
if(__MEMOI.count(__MEMOI_ELEMENT)){{
return __MEMOI[__MEMOI_ELEMENT];}}
{scope}
                    }}",
                        argTypes = args_types_to_string(args, &lang),
                        argNames = args_names_to_string(args, &lang),
                        args = args_to_string(args, &lang),
                        retType = return_type,
                        scope = transpile(&tree, pivot, &lang),
                        funcName = func_name,
                        cst = (if func_is_const {"const"} else {""})
                    ))
                }
                else {
                    render(jpath!(lang, funcs.functions)?, &json!({
                        "type": return_type,
                        "name": &splited[2],
                        "args": args_to_string(&arguments_parse(&splited[3..where_args_end].to_vec(), &lang)?, &lang),
                        "block": transpile(&tree, pivot, &lang)
                    }))
                }
            }
        }
    }
}