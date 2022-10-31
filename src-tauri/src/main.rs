#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello {}", name)
}
use libloading::{Library, Symbol};
use std::env;
use tauri::{
  App, AppHandle, GlobalShortcutManager, GlobalWindowEvent, Manager, RunEvent, SystemTrayEvent,
  WindowMenuEvent, Wry,
};
extern crate libloading;
type MyHcInit = fn(usize, usize);

mod menu;
mod tray;

fn main() {
  // let library_path = "hc_icrf32.dll";

  // unsafe {
  //   let lib = Library::new(library_path).unwrap();
  //   let func: Symbol<MyHcInit> = lib.get(b"hc_init").unwrap();

  //   func(100, 9600);
  // };
  let app = tauri::Builder::default()
    .plugin(tauri_plugin_single_instance::init(|app, _argv, _cwd| {
      //设置插件
      let window = app.get_window("main").unwrap(); //二次打开软件时，显示已打开窗口，单例运行app
      window.set_focus().unwrap();
      window.show().unwrap();
    }))
    .system_tray(tray::system_tray())
    .menu(menu::generate_menu())
    // .on_menu_event(menu::menu_envent)
    .on_system_tray_event(menu_handle)
    .invoke_handler(tauri::generate_handler![greet])
    .build(tauri::generate_context!())
    .expect("error while running tauri application");
  register_shortcut(&app);
  app.run(handle_run_events);
}
// 托盘事件
fn menu_handle(app_handle: &tauri::AppHandle, event: SystemTrayEvent) {
  match event {
    SystemTrayEvent::LeftClick {
      position: _,
      size: _,
      ..
    } => {
      let window = app_handle.get_window("main").unwrap();
      println!("鼠标-左击");
    }
    SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
      "quit" => {
        std::process::exit(0);
      }
      "hide" => {
        let item_handle = app_handle.tray_handle().get_item(&id);
        let window = app_handle.get_window("main").unwrap();
        if window.is_visible().unwrap() {
          window.hide().unwrap();
          item_handle.set_title("显示窗口").unwrap();
        } else {
          window.show().unwrap();
          item_handle.set_title("隐藏窗口").unwrap();
        }
      }
      _ => {}
    },
    _ => {}
  }
}
// 快捷键
fn register_shortcut(app: &App<Wry>) {
  let mut short_cut = app.global_shortcut_manager();
  let app_handler = app.handle();
  let result = short_cut.register("alt+space", move || {
    let window = app_handler.get_window("main").unwrap();
    if window.is_visible().unwrap() {
      window.hide().unwrap();
    } else {
      window.show().unwrap();
    }
  });
  if let Err(e) = result {
    println!("{}", e);
  }
}
// 快捷键事件
fn handle_run_events(_app_handle: &AppHandle<Wry>, e: RunEvent) {
  match e {
    RunEvent::Exit => {}
    RunEvent::ExitRequested { .. } => {}
    RunEvent::WindowEvent {
      label: _, event: _, ..
    } => {}
    RunEvent::Ready => {}
    RunEvent::Resumed => {}
    RunEvent::MainEventsCleared => {}
    _ => {}
  }
}
