// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::process;

use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, Wry};

enum TrayIdentifier {
    Hide,
    Show,
    Quit,
    Unimplemented,
}

impl Into<String> for TrayIdentifier {
    fn into(self) -> String {
        match self {
            TrayIdentifier::Quit => "quit".to_owned(),
            TrayIdentifier::Unimplemented => "unimplemented".to_owned(),
            TrayIdentifier::Hide => "hide".to_owned(),
            TrayIdentifier::Show => "show".to_owned(),
        }
    }
}

impl From<String> for TrayIdentifier {
    fn from(val: String) -> Self {
        match val.as_str() {
            "show" => TrayIdentifier::Show,
            "hide" => TrayIdentifier::Hide,
            "quit" => TrayIdentifier::Quit,
            _ => TrayIdentifier::Unimplemented,
        }
    }
}

pub(crate) fn get_system_tray_menu(is_hidden: bool) -> SystemTrayMenu {
    log::debug!("Generating system tray menu...");
    log::trace!("is hidden: {}", is_hidden);
    let hide_item = CustomMenuItem::new(TrayIdentifier::Hide, "Hide");
    let show_item = CustomMenuItem::new(TrayIdentifier::Show, "Show");
    let quit_item = CustomMenuItem::new(TrayIdentifier::Quit, "Quit");
    SystemTrayMenu::new()
        .add_item(match is_hidden {
            true => show_item,
            false => hide_item,
        })
        .add_item(quit_item)
}

pub fn get_system_tray() -> SystemTray {
    log::debug!("Generating system tray...");
    let tray_menu = get_system_tray_menu(false);
    SystemTray::new().with_menu(tray_menu)
}

pub fn handle_tray_event(app: &AppHandle<Wry>, event: SystemTrayEvent) {
    log::debug!("Handling tray event...");

    let main_window = app
        .get_window("main")
        .expect("Could not get the main window.");

    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            let tray_ident = TrayIdentifier::from(id.clone());

            match tray_ident {
                TrayIdentifier::Quit => {
                    log::info!("Quitting app by system tray request...");
                    process::exit(0);
                }
                TrayIdentifier::Unimplemented => {
                    log::warn!("An unimplemented system tray event was dispatched.");
                    log::warn!("event id: {}", id);
                }
                TrayIdentifier::Hide => {
                    log::info!("Hiding app window by system tray request...");
                    main_window.hide().expect("Could not hide the main window.");
                    log::info!("Replacing system tray...");
                    app.tray_handle()
                        .set_menu(get_system_tray_menu(true))
                        .expect("Could not set system tray menu.");
                }
                TrayIdentifier::Show => {
                    log::info!("Showing app window by system tray request...");
                    main_window.show().expect("Could not show the main window.");
                    log::info!("Replacing system tray...");
                    app.tray_handle()
                        .set_menu(get_system_tray_menu(false))
                        .expect("Could not set system tray menu.");
                }
            }
        }
        SystemTrayEvent::LeftClick { position, size, .. } => {
            log::warn!("Received left click event on system tray but nothing to do here.");
        }
        SystemTrayEvent::RightClick { position, size, .. } => {
            log::warn!("Received right click event on system tray but nothing to do here.");
        }
        SystemTrayEvent::DoubleClick { position, size, .. } => {
            log::warn!("Received double click event on system tray but nothing to do here.");
        }
        _ => log::warn!("Unknown system tray event was dispatched."),
    }
}
