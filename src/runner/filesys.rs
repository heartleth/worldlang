use rand::distributions::Alphanumeric;
use super::transpile::*;
use blocks::render;
use std::io::Write;
use std::fs::File;
use std::io::Read;
use rand::Rng;
use json;

pub static mut CURRENT_FILE :String = String::new();

pub fn convert_to_cpp(path :&String, ext :&str, lang :&json::JsonValue)->std::io::Result<()> {
    unsafe { CURRENT_FILE = String::from(&path[..]) }

    let mut target = File::create(format!("{}.{}", path, ext))?;
    let mut source_file = File::open(&format!("{}.epp", path))?;
    let mut source = String::new();
    source_file.read_to_string(&mut source)?;
    let converted :String = transpile(&tree::CodeTree::treeify(&source), 0, &lang);
    if ext == lang["h_ext"].as_str().unwrap() {
        target.write_all(render(&lang["header_guards"].as_str().unwrap(), &json!({
            "token": rand::thread_rng().sample_iter(&Alphanumeric).take(20).collect::<String>(),
            "contents": converted
        })).unwrap().as_bytes())?;
    }
    else {
        target.write_all(converted.as_bytes())?;
    }
    Ok(())
}