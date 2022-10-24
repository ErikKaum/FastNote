use clap::{Command, arg, ArgMatches };
use serde::{Serialize,Deserialize};
use std::fs::File;
use std::fs::read_dir;
use std::io::Error;

#[derive(Debug, Serialize, Deserialize)]
enum Editor {
    Vim,
    Nano
}

#[derive(Debug, Serialize, Deserialize)]
struct Conf {
    path: String,
    editor: Editor,
}


fn init() -> Result<(), confy::ConfyError> {

    let test_conf = Conf {
        path: String::from("./data/"),
        editor: Editor::Vim,
    };
    confy::store("fastnote", "fastnote-conf", test_conf)?;
    println!("=> FastNote ready to use!");
    println!("=> To create your first note, type: fastnote new [NOTE_NAME]");
    Ok(())
}

fn add(name :&ArgMatches) -> Result<(), Error> {
    
    let base = String::from("./data/");
    let note_name = name.get_one::<String>("NOTE_NAME").unwrap();
    
    let full_path = format!("{}{}", base, note_name);
    
    File::create(full_path)?;
    Ok(())
}

fn ls() {
    let files = read_dir("./data/").unwrap();
    for file in files {
        let path = file.unwrap().path();
        let name = path.to_str();
        match name {
            None => {},
            // Some(f) => println!("{}", f.replace("./", ""))
            Some(f) => println!("{}", f)
        }
    }
}

fn del(name: &ArgMatches) {
    let note_name = name.get_one::<String>("NOTE_NAME").unwrap();
    println!("This function will delete the note: {}", note_name);
}

fn main() {
    let matches = Command::new("FastNote")
        .version("0.1.0")
        .author("Erik Kaunismäki>")
        .about("Take notes Fast & Easy in the terminal. Extremely unopinionated!")
        .subcommand(
            Command::new("init")
            .about("init FastNote")
        )
        .subcommand(
            Command::new("new")
                .about("create new note")
                .arg(arg!([NOTE_NAME])),
        )
        .subcommand(
            Command::new("ls")
                .about("list all notes")
        ).subcommand(
            Command::new("del")
                .about("delete note")
                .arg(arg!([NOTE_NAME])),
        )
        .get_matches();

        match matches.subcommand() {
            Some(("init", _sub_m)) => init().unwrap(),
            Some(("ls", _sub_m)) => ls(),
            Some(("del", sub_m)) => del(&sub_m),
            Some(("new", sub_m)) => {
                let res = add(&sub_m);
                match res {
                    Ok(_) => {},
                    Err(e) => panic!("{}", e)
                }
            },
            _ => println!("Not a recognized command, type --help to get help")
        }    
}
