use std::fs;
use std::fs::File;
use tauri::{CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent};

pub fn generate_menu() -> Menu {
    let open_file = CustomMenuItem::new("open_file".to_string(), "打开文件");
    let setting = CustomMenuItem::new("setting".to_string(), "设置");
    let submenu = Submenu::new("打开", Menu::new().add_item(open_file).add_item(setting));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_native_item(MenuItem::Cut)
        .add_item(CustomMenuItem::new("hide", "最小化"))
        .add_submenu(submenu);

    menu
}

//菜单事件
// pub fn menu_envent(e: WindowMenuEvent) {
//     let win = Some(e.window());
//     match e.menu_item_id() {
//         "open_file" => {
//             e.
//         }
//         _=>{}
//     }
// }
