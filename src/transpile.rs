/*******************************************************
 * S A I L O R N E P T U N E / K A I O U M I C H I R U *
 *******************************************************
 */
//              ,----_-```-.
//             /            \
//            /   ,-_ ,``.   |
//           |    \````_`|    `--_
//         .`   , /;`0`-``;;.     /
//          ;   \((`o`  `o),`     `-`\
//       ,````:.\(\   ;,  (           `=``
//       |     `) :.-.   _|)         _/`
//       \       \:\. ```(_    ,--.-`
//        \      | \ \     )__/_--`.
//         `-;__,\ |  \   __/ /_-`|/
//          /  / \\ \  \``  /|/   |
//        /   _|  |-:`\ `. /`-^    |
//      /   ,/_/  |`.  \ /##- /|   |
//    /--####|  ,`/```--\####|`:|   |
//  /   /####|-:,/        ##|  `-_  |
// ``--`````    \          /`_      |
//              /          \##``--_/
//          ___/|        ,` ,\##.##|
//    .--#######\\     ,` ######`````-*
//     \#########\:  ,` #############/
//     .\#########\\/.,#############/

pub mod blocks;
pub use blocks::{
    parse_sentence,
    Mem,
    keyword,
    split,
    regi,
    value_parse,
    first_phrase,
    first_clause,
    tree
};
pub use super::runner::filesys::CURRENT_FILE as CURRENT_FILE;
pub use crate::jpath;

pub fn transpile(tree :&Mem, pivot :usize, lang :&json::JsonValue)->String {
    let mut ret = String::new();
    let mut iter = 0;
    let len = tree[pivot].children.len();
    loop {
        if iter == len {
            break;
        }
        let parent = tree[pivot].children[iter];
        let elem = &tree[parent];
        let code = &String::from(elem.code.trim());
        let code_splited = split(&code);
        let keyword = keyword(&code);
        
        let parsed = &(||{
            if code.len() == 0 { Ok(String::new()) }
            else if regi(&keyword, "^(unless|if|else)$") {
                blocks::parse_if(&tree, &mut iter, pivot, &lang)
            }
            else if regi(&keyword, "^(repeat)$") {
                blocks::parse_repeat(&tree, parent, &lang)
            }
            else if regi(&keyword, "^(while)$") {
                blocks::parse_while(&tree, parent, &lang)
            }
            else if regi(&keyword, "^(for)$") {
                blocks::parse_for(&tree, parent, &lang)
            }
            else if regi(&keyword, "^(make|ha(ve|s)|let)$") {
                blocks::parse_new(code, &lang)
            }
            else if regi(&keyword, "^when$") {
                blocks::parse_when(&tree, parent, &lang)
            }
            else if regi(&keyword, "^(include|lib(rary)?|using|import)$") {
                blocks::parse_import(&tree, parent, &lang)
            }
            else if regi(&keyword, "^return$") {
                blocks::parse_return(&code, &lang)
            }
            else if regi(&code, r"^name\s?space\s.+$") {
                blocks::parse_namespace(&tree, parent, &lang)
            }
            else if regi(&keyword, "^(break|continue)$") {
                Ok(format!("{};", code))
            }
            else if regi(&keyword, "^public|private|protected$") {
                blocks::parse_access(&tree, parent, &lang)
            }
            else if regi(&keyword, "^set$") {
                blocks::parse_set(&code, &lang)
            }
            else if regi(&keyword, "^class$") {
                blocks::parse_class(&tree, parent, &lang)
            }
            else if regi(&keyword, "^use$") {
                blocks::parse_use(&tree, parent, &lang)
            }
            else if first_phrase(&code_splited, true, false)? == code_splited.len() - 1 {
                Ok(value_parse(&code, 1, &lang)? + jpath!(lang, line_end)?)
            }
            else if first_clause(&code_splited)? == code_splited.len() - 1 {
                Ok(parse_sentence(&code, &lang)? + jpath!(lang, line_end)?)
            }
            else {
                Err("Invalid sentence")
            }
        })();
        if let Ok(e) = parsed {
            ret = format!("{}{}", ret, e);
        }
        else if let Err(e) = parsed {
            unsafe {
                eprintln!("In file {}:\n{:3} | {}\nError: {}\n", &CURRENT_FILE, elem.line, elem.code, e);
            }
        }
        iter += 1;
    }
    ret
}