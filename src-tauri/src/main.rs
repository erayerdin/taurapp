// Copyright (c) 2022 Eray Erdin
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use app::{logging, tray, window::handle_window_event};

fn main() {
    logging::setup_logger().expect("Could not set up loggers.");
    log::info!("Launching app...");
    tauri::Builder::default()
        .system_tray(tray::get_system_tray())
        .on_system_tray_event(tray::handle_tray_event)
        .on_window_event(handle_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
