#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};

fn main() {
    // here `"quit".to_string()` defines the menu item id, and the second parameter is the menu item label.
    let vpnconnect = CustomMenuItem::new("vpnconnect".to_string(), "Connect");
    let vpndisconnect = CustomMenuItem::new("vpndisconnect".to_string(), "Disconnect");
    let vpnconnectfrankfurt = CustomMenuItem::new("vpnconnectfrankfurt".to_string(), "Connect Frankfurt");
    let restore = CustomMenuItem::new("restore".to_string(), "Restore resolve.conf");
    let tray_menu = SystemTrayMenu::new()
        .add_item(vpnconnect)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(vpnconnectfrankfurt)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(restore)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(vpndisconnect);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "vpnconnect" => {
                    println!("Connecting");
                    connectnord();
                }
                "vpnconnectfrankfurt" => {
                    println!("Connecting Frankfurt");
                    connectnordfrankfurt();
                }
                "vpndisconnect" => {
                    println!("Disconnecting");
                    disconnectnord();
                }
                "restore" => {
                    println!("Resoring resolve.conf");
                    restore_resolve();
                }
                _ => {}
            },
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use std::process::Command;

fn connectnord() {
    let output = Command::new("nordvpn")
        .arg("c")
        .arg("p2p")
        .spawn()
        .expect("failed to execute process");
    // let hello = output.stdout;
    println!("{:?}", output);
}

fn connectnordfrankfurt() {
    let output = Command::new("nordvpn")
        .arg("c")
        .arg("frankfurt")
        .spawn()
        .expect("failed to execute process");
    // let hello = output.stdout;
    println!("{:?}", output);
}

fn disconnectnord() {
  let output = Command::new("nordvpn")
      .arg("d")
      .spawn()
      .expect("failed to execute process");
  // let hello = output.stdout;
  println!("{:?}", output);
}

fn restore_resolve() {
  let output = Command::new("terminator")
    //   .arg("-e")
    //   .arg("res")
      .spawn()
      .expect("failed to execute process");
  // let hello = output.stdout;
  println!("{:?}", output);
}
