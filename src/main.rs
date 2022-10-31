use clap::{Command, arg, ArgMatches };
use serde::{Serialize,Deserialize};
use std::fs::File;
use std::fs::read_dir;
use std::io::Error;
use std::path::PathBuf;
use std::process;


#[derive(Default, Debug, Serialize, Deserialize)]
struct Conf {
    path: PathBuf,
    editor: String,
}

fn read_conf() -> Result<String, confy::ConfyError> {
    let conf: Conf = confy::load("fastnote", "fastnote-conf")?;
    let base = conf.path.into_os_string().into_string().unwrap();
    let clean_base = format!("{}", base.replace("fastnote-conf.toml", ""));

    Ok(clean_base)
} 

fn create_conf() -> Result<Conf, confy::ConfyError> {

    let test = confy::get_configuration_file_path("fastnote", "fastnote-conf")?;
    println!("{:?}", test);

    let mut p = String::new();
    println!("Enter path for where to store your notes:");
    std::io::stdin().read_line(&mut p).unwrap(); 
    // let clean_path = format!("{}", p.trim());

    let mut e = String::new();
    println!("Which text editor do you want to use:");
    std::io::stdin().read_line(&mut e).unwrap();
    let clean_editor = format!("{}", e.trim());

    let test_conf = Conf {
        path: test,
        editor: clean_editor,
    };
    Ok(test_conf)
} 

fn init() -> Result<(), confy::ConfyError> {

    let test_conf = create_conf()?;
    confy::store("fastnote", "fastnote-conf", test_conf)?;
    println!("=> FastNote ready to use!");
    println!("=> To create your first note, type: fastnote new [NOTE_NAME]");
    Ok(())
}

fn new(name :&ArgMatches) -> Result<(), Error> {

    let base = read_conf().unwrap();
    let note_name = name.get_one::<String>("NOTE_NAME").unwrap();
    let full_path = format!("{}{}", base, note_name);
    
    File::create(&full_path)?;
    println!("{}", full_path);

    process::Command::new("/bin/bash")
        .arg("-c")
        .arg("vim $0")
        .arg(full_path)
        .spawn()
        .expect("vim failed to start")
        .wait()
        .expect("Error: Editor returned a non-zero status");

    Ok(())
}

fn open(name: &ArgMatches) {

    let base = read_conf().unwrap();
    let note_name = name.get_one::<String>("NOTE_NAME").unwrap();
    let full_path = format!("{}{}", base, note_name);

    println!("{}", full_path);
    
    process::Command::new("/bin/bash")
        .arg("-c")
        .arg("vim $0")
        .arg(full_path)
        .spawn()
        .expect("vim failed to start")
        .wait()
        .expect("Error: Editor returned a non-zero status");
}

fn ls() {
    let base = read_conf().unwrap();

    println!("{}", base);

    let files = read_dir(base).unwrap();
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
        ).subcommand(
            Command::new("open")
                .about("open note")
                .arg(arg!([NOTE_NAME])),
        )
        .get_matches();

        match matches.subcommand() {
            Some(("init", _sub_m)) => init().unwrap(),
            Some(("ls", _sub_m)) => ls(),
            Some(("del", sub_m)) => del(&sub_m),
            Some(("open", sub_m)) => open(&sub_m),
            Some(("new", sub_m)) => {
                let res = new(&sub_m);
                match res {
                    Ok(_) => {},
                    Err(e) => panic!("{}", e)
                }
            },
            _ => println!("Not a recognized command, type --help to get help")
        }    
}
