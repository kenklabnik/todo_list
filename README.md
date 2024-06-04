# todo_list
Very much WIP. Will create and manage to-do lists, powered by Rust.

## todo_list's TODO list
The project needs a better name.

The internal logic appears to be working locally based on the tests written, including the file I/O. 

On my Windows machine, ``cargo test`` makes temporary test files in /AppData/Local/Temp, and they seem to not be being properly deleted, but it is unclear to me whether this is an OS issue or something I can fix from inside this project.

I'm still figuring out how to operate egui/eframe, so the UI is not functioning yet. As such, I can't really say this is in a usable state yet--I just uploaded this repo to show what I've been learning recently.

Once the UI is working to my satisfaction, some refinements I'm thinking about include:

- I would like to be able to access my lists from other devices (web/mobile, security features)
- Aesthetic improvements
- Notifications
- Under-the-hood improvements (modules, removing unncessary comments and debug code, performance)
