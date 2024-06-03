use std::env;
use std::fs; 
use std::fs::File;
use std::io;
use std::io::{BufReader, Write, Read};
use std::path::PathBuf;
use std::fmt::Debug;

use ::serde::{Serialize, Deserialize};

use chrono::*;

use eframe::egui;
use eframe::Storage;

use rfd; // open/save dialogs

use egui::*;

#[derive(Serialize, Deserialize, Default)]
pub struct TodoUi {
    pub loaded_list: Option<TodoList>,
    pub all_lists: Vec<TodoList>,
    pub current_path: Option<PathBuf>,
}

impl TodoUi {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        //customize egui here with cc.egui_ctx.set_fonts, set_visuals
        //to restore app state, use cc.storage with "persistence" feature enabled
        //use cc.gl (a glow::Context) to create graphics shaders and buffers
        //  which can be used for e.g. egui::PaintCallback
        cc.egui_ctx.set_visuals(Visuals::dark());

        let mut fonts = FontDefinitions::default();
        let mut font_path: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Cargo manifest dir not found."));
        font_path.push("fonts");
        font_path.push("DM_Sans");
        font_path.push("DMSans-VariableFont_opsz,wght.ttf");
        let font_file = std::fs::read(font_path).expect("Unable to read from the file containing font data (permission issue maybe?)");
        
        fonts.font_data.insert("DM_Sans".to_owned(),
            FontData::from_owned(font_file)); 

        fonts.families.insert(
            FontFamily::Name("DM_Sans".into()),
            vec![
                "DM_Sans".to_owned(),
            ]
        );

        cc.egui_ctx.set_fonts(fonts);

        //persistence
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
    
    fn top_panel_frame(&self) -> egui::Frame {
        Frame::none()
        .fill(Color32::DARK_GRAY)
    }

}

impl eframe::App for TodoUi {
    // framework calls this to save state before shutdown
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut debug_mode = false;
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            if &args[1] == &String::from("debug") {
                debug_mode = true;
            }
        }

        let mut top_panel = egui::TopBottomPanel::top("top_panel");

        top_panel
            .frame(self.top_panel_frame())
            .show(ctx, |ui| {
                ui.label(RichText::new("Todo List Manager")
                    .color(Color32::YELLOW)
                    .size(36.0)
                    .family(FontFamily::Name("DM_Sans".into()))
                ); 
                ui.label(RichText::new("Catchy subtitle to be determined.")
                    .family(FontFamily::Monospace));
            });

        egui::TopBottomPanel::top("top_panel_2").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Options: ");

                if ui.button("New List").clicked() {
                    //TODO implement check if current list is saved
                    self.loaded_list = Some(TodoList::new(String::from("New Todo List")));
                }

                if ui.button("Load List").clicked() {
                    //TODO implement check if current list is saved
                    let empty_path = PathBuf::new();
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter("json", &["json"])
                        .set_directory(match &self.current_path {
                            Some(p) => p,
                            None => &empty_path,
                        })
                        .pick_file() {
                            self.current_path = Some(path);
                    }

                }

                if self.loaded_list.is_some() {
                    if ui.button("Save List").clicked() {
                        //TODO spin up new thread to handle this to reduce GUI lag
                        let empty_path = PathBuf::new();
                        if let Some(mut path) = rfd::FileDialog::new()
                        .add_filter("json", &["json"])
                        .set_directory(match &self.current_path {
                            Some(p) => p,
                            None => &empty_path,
                        })
                        .save_file() {
                            let mut saved_list = self.loaded_list.as_mut().unwrap();
                            saved_list.save(&path);
                            self.current_path = Some(path);
                        }
                    }
                }
            });
        });

        let rename_list_popup = egui::Window::new("Rename List")
            .collapsible(true)
            .title_bar(true)
            .default_open(false)
            .open(&mut false)
            .show(&ctx, |ui| {
                ui.label("Enter a new title here.");
                ui.text_edit_singleline(&mut String::new());

            });
        

        egui::CentralPanel::default().show(ctx, |ui| {
            if debug_mode {
                egui::ScrollArea::vertical().show(ui, |ui| {    
                    ui.heading("central panel");
                    ui.label("Context memory dump:");
                    //ctx.memory_ui(ui);
                    ui.label("Context inspection ui:");
                    //ctx.inspection_ui(ui);

                    let mut stored = false;
                    ui.label("eframe storage status:");
                    match frame.storage() {
                        Some(store) => {
                            ui.label("eframe storage available. \"test\" returns:");
                            match store.get_string("test") {
                                Some(value) => { 
                                    stored = true;
                                    ui.label(value)
                                }
                                None => ui.label("None."),
                            }
                        }
                        None => ui.label("Storage returned None."),
                        };
                    if !stored {
                        if ui.button("Store test data").clicked() {
                            frame.storage_mut().expect("Store test data button couldn't run storage_mut")
                                .set_string("test", String::from("Successfully-retrieved test string! Hello, eframe storage..."));
                        }
                    }
                })
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    
                    let placeholder_list = TodoList::new(String::from("New Todo List"));
                    ui.label("Under construction! Run with \"debug\" as an arg for egui information. Loaded list path: ");

                    ui.label(self.current_path.as_ref().expect("Problem unwrapping current_path in CentralPanel").display().to_string());

                    let mut loaded_list = match self.loaded_list.as_ref() {
                        Some(list) => list,
                        None => &placeholder_list,
                    };

                    ui.horizontal(|ui| {
                        ui.heading(&loaded_list.title);
                        if ui.button("Change title").clicked() {
                            //??
                        }
                    });
                })
            }
        });
    }
}



#[derive(Serialize, Deserialize, Default)]
pub struct TodoList {
    pub title: String,
    pub items: Vec<TodoListItem>,
    date_created: DateTime<Local>,
    //pub path: String,
}

#[derive(Serialize, Deserialize, Default)]
pub struct TodoListItem {
    id: usize,
    pub name: String,
    pub description: String,
    date_created: DateTime<Local>,
    completed: bool,
}

impl TodoList {
    pub fn new(title: String) -> TodoList {
        let date_created = Local::now();
        let items: Vec<TodoListItem> = Vec::new();
        TodoList {title, date_created, items}
    }

    pub fn add(&mut self, name: String, description: String) {

        let ids: Vec<usize> = self.items.iter().map(|x| x.id).collect();
        let mut id: usize = 0;

        for i in 0..ids.len() {
            if i >= id {
                id = i + 1;
            }
        }

        self.items.push(TodoListItem::new(id, name, description));
    }

    fn clear_list(&mut self) {
        self.items = Vec::new();
    }

    pub fn remove_item(&mut self, id: usize) {
        //TODO gracefully handle situations where multiple items may have the same id
        //this could happen due to bugs or due to manually editing the saved list files
        //for now, just removes the first such instance found

        for i in 0..self.items.len() {
            if self.items[i].id == id {
                self.items.remove(i);
                break;
            }
        }
    }

    pub fn from_file(path: &PathBuf) -> Result<TodoList, io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let contents = serde_json::from_reader(reader)?;

        Ok(contents)
    }

    pub fn save(&mut self, path: &PathBuf) -> Result<(), io::Error> {
        let json = serde_json::to_string(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

impl TodoListItem {
    pub fn new(id: usize, name: String, description: String) -> TodoListItem {
        let date_created = Local::now();
        TodoListItem { id, name, description, date_created, completed: false, }
    }

    pub fn date_created(&self) -> DateTime<Local> {
        self.date_created
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile;

    fn make_empty_list() -> TodoList {
        let mut list = TodoList::new(String::from("test list"));
        list
    }

    fn make_one_item_list() -> TodoList {
        let mut list = TodoList::new(String::from("test list"));
        list.add(String::from("Test Item"), String::from("Test Description"));
        list
    }

    fn make_item_no_list() -> TodoListItem {
        let item = TodoListItem::new(999, String::from("Test Item"), String::from("Test Description"));
        item
    }

    #[test]
    fn create_todo_list() {
        let list = make_empty_list();
        assert_eq!(list.title, String::from("test list"));
    }

    #[test]
    fn create_list_item() {
        let item = make_item_no_list();
        assert_eq!(item.name, String::from("Test Item"));
        assert_eq!(item.description, String::from("Test Description"));
        assert_eq!(item.completed, false);
        //TODO assert something about item.date_created();
    }

    #[test]
    fn add_item_to_todo_list() {
        let list = make_one_item_list();

        assert_eq!(list.items[0].name, String::from("Test Item"));
        assert_eq!(list.items[0].description, String::from("Test Description"));
    }

    #[test]
    fn clear_list() {
        let mut list = make_one_item_list();
        list.clear_list();
        assert!(list.items.len() == 0);
    }

    #[test]
    fn remove_item_from_list() {
        let mut list = make_one_item_list();
        list.add(String::from("New Item"), String::from("Other Description"));
        list.remove_item(0);

        assert!(list.items.len() == 1);
        assert_eq!(list.items[0].name, String::from("New Item"));
    }

    #[test]
    fn only_removes_one_item() {
        let mut list = make_one_item_list();
        list.add(String::from("New Item"), String::from("Other Description"));
        list.add(String::from("New Item"), String::from("Other Description"));

        list.remove_item(1);
        assert!(list.items.len() == 2);
    }

    #[test]
    fn clears_whole_list() {
        let mut list = make_one_item_list();
        list.add(String::from("New Item"), String::from("Other Description"));
        list.add(String::from("New Item"), String::from("Other Description"));

        list.clear_list();
        assert!(list.items.len() == 0);
    }

    #[test]
    fn saves_json_to_file() {
        let mut list = make_one_item_list();

        let mut temp = tempfile::NamedTempFile::new().expect("saves_json_to_file() test panicked; temporary file creation failed.");
        let temp_path = temp.path().to_path_buf();

        list.save(&temp_path).expect("saves_json_to_file() test panicked; temporary file created but could not be saved to.");
        
        let mut temp_contents = String::new();

        temp.read_to_string(&mut temp_contents).expect("saves_json_to_file() test panicked; temporary file created but could not be read from.");

        //eprintln!("{}", temp_contents); // run with `-- --nocapture` argument to `cargo test` 

        assert!(temp_contents.len() > 0); //for now, we assert anything is in the file; TODO more specific assertions?
        temp.close().expect("saves_json_to_file() test panicked; temporary file was not deleted afterwards.");
    }

    #[test]
    fn loads_json_from_file() {
        let mut temp = tempfile::NamedTempFile::new().expect("loads_json_from_file() test panicked; temporary file creation failed.");

        let mut contents = String::from("{\"title\":\"test list\",\"items\":\
        [{\"id\":0,\"name\":\"Test Item\",\"description\":\"Test Description\",\"date_created\":\"2024-05-22T15:40:04.970459400-04:00\",\"completed\":false}],\
        \"date_created\":\"2024-05-22T15:40:04.969858100-04:00\"}");

        temp.write(contents.as_bytes()).expect("loads_json_from_file() test panicked; temporary file created, but could not be written to.");
        let temp_path = temp.path().to_path_buf();

        let list = TodoList::from_file(&temp_path).expect("loads_json_from_file() test panicked; a temp file was created, but TodoList::from_file could not construct a TodoList.");

        assert!(list.items.len() > 0);

        //eprintln!("{}", temp_path.display()); // run with `-- --nocapture` argument to `cargo test` 
        temp.close().expect("saves_json_to_file() test panicked; temporary file was not deleted afterwards.");

    }
}