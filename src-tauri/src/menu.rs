use tauri::{menu::{Menu, MenuItem, Submenu}, App, EventLoopMessage, Wry};

// pub fn build_menu(app:App) -> Menu<Wry<EventLoopMessage>>{
//     let handle = app.handle();
//     let quit = MenuItem::new(handle, "Quit", true, None::<&str>).unwrap();
//     let close = MenuItem::new(handle,"Close", true, None::<&str>).unwrap();
//     let sub = Submenu::with_items(handle, "File", true, 
//     &[&quit, &close]).unwrap();
//     let menu = Menu::with_items(handle, &[&sub]).unwrap();


// }
