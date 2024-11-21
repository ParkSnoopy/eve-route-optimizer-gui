//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;

#[allow(unused)]
mod trace;

mod config;
mod args;
mod route;
mod system;
mod request;
mod progress;

mod app;



#[tokio::main]
async fn main() -> eyre::Result<eframe::Result> {
    enable_ansi_support::enable_ansi_support()?;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([config::GUI_WIDTH, config::GUI_HEIGHT])
            .with_resizable(false),
        multisampling: 2,
        centered: true,

        ..Default::default()
    };

    Ok(eframe::run_native(
        config::APP_NAME,
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            Ok(Box::new(app::RouteOptimizerApp::new(cc)))
        }),
    ))
}
