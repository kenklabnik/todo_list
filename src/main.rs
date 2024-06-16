//todo_list basics:
//display a GUI
//save a list of items to a text file
//read items from text file and display
//future ideas:
//possible web integration? mobile support?
//notifications
//multiple lists with different properties

use todo_list::TodoUi;

fn main() -> eframe::Result<()> {
    env_logger::init(); //logs to stderr if run with `RUST_LOG=debug`
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Todo List Manager",
        native_options,
        Box::new(|cc| Box::new(TodoUi::new(cc)))
    )

}


//previous code, might delete later
/*fn logic_init() {
    println!("Checking for existing list...");

    //if filesystem changes break this project, this may help for debugging
    //println!("CARGO_MANIFEST_DIR: {}", env::var("CARGO_MANIFEST_DIR").unwrap()); //gets project root as seen by Cargo
    //println!("Current dir: {}", env::current_dir().expect("\nError: couldn't get current directory.").display());

    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest dir not found."));
    
    path.push("lists");
    path.push("daily.txt");

    if path.exists() {
        println!("List found at path: {}", path.display());
    } else {
        println!("No list found at path: {}\nMaking ./lists/daily.txt ...", path.display());
        path.pop();
        if !path.exists() {
            fs::create_dir(path.clone()).expect("Problem creating directory 'lists'");
        }
        path.push("daily.txt");
        
        let _ = match fs::write(&path, "Sample Title\nSample Daily Item") {
            Ok(()) => println!("File created."),
            Err(error) => panic!("Problem creating daily.txt: {:?}", error),
        };
    }

    path.pop();
    println!("Shutting down...");
}*/