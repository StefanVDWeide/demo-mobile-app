#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub fn main() {
    // TODO: MAKE SURE THIS IS CLEAR IN THE ARTICLES
    demo_mobile_app::AppBuilder::new().run();
}
