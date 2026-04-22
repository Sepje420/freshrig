// Copyright (c) 2026 Seppe Willemsens (ZIPREX420). MIT License.
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    freshrig_lib::run();
}
