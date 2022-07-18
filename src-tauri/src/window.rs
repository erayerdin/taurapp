// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tauri::{GlobalWindowEvent, Manager};

use crate::tray;

pub fn handle_window_event(event: GlobalWindowEvent) {
    log::debug!("Handling window event...");

    let window_event = event.event();

    match window_event {
        tauri::WindowEvent::CloseRequested { api, .. } => {
            api.prevent_close();
            event.window().app_handle().tray_handle().set_menu(match event.window().is_visible() {
                Ok(is_visible) => tray::get_system_tray_menu(is_visible),
                Err(err) => {
                    log::error!("An error occured while getting the window's visibility property. Setting it to visible...");
                    log::error!("Window visibility property access error: {}", err);
                    tray::get_system_tray_menu(false)
                },
            }).expect("Could not set system tray menu.");

            event
                .window()
                .hide()
                .expect("Could not hide the main window.");
        }
        _ => {}
    }
}
