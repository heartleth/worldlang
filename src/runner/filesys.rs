use rand::distributions::Alphanumeric;
use super::transpile::*;
use blocks::render;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use rand::Rng;
use colour;
use json;

pub static mut CURRENT_FILE :String = String::new();
pub static mut IS_FIRST :bool = true;

pub fn convert_to_cpp(path :&String, ext :&str, lang :&json::JsonValue)->std::io::Result<()> {
    unsafe { CURRENT_FILE = String::from(&path[..]) }

    let mut target = File::create(format!("{}.{}", path, ext))?;
    let mut source_file = File::open(&format!("{}.epp", path))?;
    let mut source = String::new();
    source_file.read_to_string(&mut source)?;
    let converted :String = transpile(&tree::CodeTree::treeify(&source), 0, &lang);
    unsafe {
        if IS_FIRST {
            colour::green!("  Transpiling ");
        }
        else {
            colour::green!("=>Transpiling ");
        }
    }
    println!("{0}.epp => {0}.{1}", path, ext);
    target.write_all(render(&lang["header_guards"].as_str().unwrap(), &json!({
        "token": rand::thread_rng().sample_iter(&Alphanumeric).take(20).collect::<String>(),
        "contents": converted,
        "filename": format!("{}.{}", path, ext),
        "filenamewithoutext": path
    })).unwrap().as_bytes())?;

    for c in lang["then"].members() {
        let cmd = render(c.as_str().unwrap(), &json!({"file": path})).unwrap();
        colour::green!("=>Executing ");
        println!("{}", cmd);
        if cfg!(windows) {
            std::process::Command::new("cmd")
                .args(&["/c", &cmd]).output()
                .expect("Error!");
        }
        else {
            std::process::Command::new("sh")
                .args(&["-c", &render(jpath!(lang, then).unwrap_or(""), &json!({"file": path})).unwrap_or(String::new())]).output()
                .expect("Error!");
        }
    }
    unsafe { IS_FIRST = false; }
    Ok(())
}