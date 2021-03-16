extern crate handlebars;
#[macro_use]
extern crate serde_json;
pub extern crate colour;
pub extern crate text_io;
pub mod transpile;
use transpile::*;
mod runner;
use runner::*;
use std::env;
pub use json;

fn main()-> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!(r"Enpp-every
                      /\                 ____
                   /    |               /   |
                 /      |            /      |
                |       /_______   /        |
             ___\    __/    -   `-^-,      /__-___
           -     .---                \ ___/       `--_
         /      //        /V\\         \              \
       /       //    \ \/ \  /     \    |              |
       |       |//  \ /`\_     _/   /  /               |
       |         `--_=\/ \    __-_,, _| \             /
       \           / |\\`/|  `   \ /    |           /
         \         | \\ ``:_.    _-```  /        _/
           -_       -__---___--``     -<_____--``
              `-------=`__-|  |-_____
                 __-=```_ ^\``/     /``\
                /   |  |  \ `//    /     \
              /     |      | v   //\      \
            /      | |  |  | /  /   \      \
         _-      /``===_=__=____=____ \     \
 =-- _ __-     /     |____________\    \     \
  ```=___---/-`     /  /     |   |  \   \   __-
                  /    /     |    \   \  \_-.  `---=
                /     /      |     \    \    \_<---=`
              /       /      |      \     \
            /        |       ^      _\___--\
            \\:::```|=========`````` __---``
                 `````-----------````
 - Compiled to any language
 - enpp-every [filename without ext]
 - enpp-every [filename without ext] [language json]
release Shang
");
    }
	else if args.len() == 2 {
		let lang = json::parse(&std::fs::read_to_string("./language.json").expect("Cannot found ./language.json")).expect("Cannot parse language.json");
        filesys::convert(&args[1], lang["ext"].as_str().unwrap_or("txt"), &lang)?;
	}
	else {
		let lang = json::parse(&std::fs::read_to_string(&args[2]).expect("Cannot found the json")).expect("Cannot parse language json");
        filesys::convert(&args[1], lang["ext"].as_str().unwrap_or("txt"), &lang)?;
	}

	Ok(())
}

#[macro_export]
macro_rules! jpath {
	($ln:expr, $firstname:meta $(.$names:meta)*) => {{
		$ln[stringify!($firstname).to_ascii_lowercase()]$([stringify!($names).to_ascii_lowercase()])*
		.as_str().ok_or(stringify!(Cannot found: $firstname$(.$names)*))
	}};
}