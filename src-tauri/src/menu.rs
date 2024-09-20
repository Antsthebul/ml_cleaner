use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

pub fn build_menu()-> Menu{

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    Menu::new()
        .add_submenu(submenu)
           
}