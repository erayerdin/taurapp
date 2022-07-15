// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use tauri::{GlobalWindowEvent, Manager};

use crate::tray;

pub fn handle_window_event(event: GlobalWindowEvent) {
    log::debug!("Handling window event");

    let window_event = event.event();

    match window_event {
        tauri::WindowEvent::Resized(size) => {
            log::warn!("Received resize event on window but nothing to do here.")
        }
        tauri::WindowEvent::Moved(position) => {
            log::warn!("Received move event on window but nothing to do here.")
        }
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
        tauri::WindowEvent::Destroyed => {
            log::warn!("Received destroy event on window but nothing to do here.")
        }
        tauri::WindowEvent::Focused(is_focused) => {
            log::warn!("Received focused event on window but nothing to do here.")
        }
        tauri::WindowEvent::ScaleFactorChanged {
            scale_factor,
            new_inner_size,
            ..
        } => log::warn!("Received scale factor changed event on window but nothing to do here."),
        tauri::WindowEvent::FileDrop(file_drop_event) => {
            log::warn!("Received file drop event on window but nothing to do here.")
        }
        tauri::WindowEvent::ThemeChanged(theme) => {
            log::warn!("Received theme changed event on window but nothing to do here.")
        }
        _ => todo!(),
    }
}
