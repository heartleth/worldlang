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

pub static mut IS_QUIET: bool = false;

fn main()-> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut run_cmds = None;
    let mut path = "";

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
release Chin
");
    }
	else if args.len() == 2 {
        let lang = json::parse(&std::fs::read_to_string("./language.json").expect("Cannot found ./language.json")).expect("Cannot parse language.json");
        filesys::convert(&args[1], lang["ext"].as_str().unwrap_or("txt"), &lang)?;
	}
	else if args.len() >= 3 {
        let lang = json::parse(&std::fs::read_to_string(&args[2]).expect("Cannot found the json")).expect("Cannot parse language json");
        if args[2] == "run" {
            path = &args[1];
            if args.len() >= 4 {
                if args[3] == "quietly" {
                    unsafe { IS_QUIET = true; }
                }
            }
            filesys::convert(&args[1], lang["ext"].as_str().unwrap_or("txt"), &lang)?;
            run_cmds = Some(lang);
        }
        else if args[2] == "quietly" {
            unsafe { IS_QUIET = true; }
            filesys::convert(&args[1], lang["ext"].as_str().unwrap_or("txt"), &lang)?;
        }
        else {
            if args.len() >= 4 {
                if args[3] == "run" {
                    path = &args[1];
                    if args.len() >= 5 {
                        if args[4] == "quietly" {
                            unsafe { IS_QUIET = true; }
                        }
                    }
                }
                else if args[3] == "quietly" {
                    unsafe { IS_QUIET = true; }
                }
            }
            filesys::convert(&args[1], lang["ext"].as_str().unwrap_or("txt"), &lang)?;
            if args.len() >= 4 {
                if args[3] == "run" {
                    run_cmds = Some(lang);
                }
            }
        }
	}

    if let Some(cmds) = run_cmds {
        use std::process::Stdio;
        for c in cmds["run"].members() {
            let cmd = transpile::blocks::render(c.as_str().unwrap(), &json!({"file": path})).unwrap();
            if !unsafe { IS_QUIET } {
                colour::green!("=>Running ");
                println!("{}", cmd);
            }
            if cfg!(windows) {
                std::process::Command::new("cmd")
                    .args(&["/c", &cmd])
                    .stdout(Stdio::inherit())
                    .stdin(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("Error!")
                    .wait()
                    .expect("Error!");
            }
            else {
                std::process::Command::new("sh")
                    .args(&["-c", &cmd])
                    .stdout(Stdio::inherit())
                    .stdin(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("Error!")
                    .wait()
                    .expect("Error!");
            }
        }
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