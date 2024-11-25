//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;

mod config;
mod app;



fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config::GUI_WIDTH, config::GUI_HEIGHT])
            .with_resizable(false),
        multisampling: 2,
        centered: true,

        ..Default::default()
    };

    eframe::run_native(
        config::APP_NAME,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(app::RouteOptimizerApp::new(cc)))
        }),
    )
}
