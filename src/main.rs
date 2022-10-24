use clap::{Command, arg, ArgMatches };
use std::fs::File;
use std::fs::read_dir;
use std::io::Error;

fn init() {
    println!("called init function");
}

fn add(name :&ArgMatches) -> Result<&String, Error> {
    let note_name = name.get_one::<String>("NOTE_NAME").unwrap();
    File::create(note_name)?;
    Ok(note_name)
}

fn ls() {
    let files = read_dir("./").unwrap();
    for file in files {
        let path = file.unwrap().path();
        let name = path.to_str();
        match name {
            None => {},
            Some(f) => println!("{}", f.replace("./", ""))
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
            Some(("init", _sub_m)) => init(),
            Some(("ls", _sub_m)) => ls(),
            Some(("del", sub_m)) => del(&sub_m),
            Some(("new", sub_m)) => {
                let res = add(&sub_m);
                match res {
                    Ok(res) => println!("{res}"),
                    Err(e) => panic!("{}", e)
                }
            },
            _ => println!("Not a recognized command, type --help to get help")
        }    
}
