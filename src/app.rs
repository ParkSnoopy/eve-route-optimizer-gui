use eframe::egui;

use route_optimizer_core::{
    prelude::*,
    trace,
};

use std::sync::{ Arc, RwLock };

use crate::config;



#[derive(Default)]
pub struct RouteOptimizerApp {
    route: String,
    start: String,
    end: String,
    route_option: RouteOption,
    concurrent: usize,

    result: String,
    buffer: Arc<RwLock<String>>,
}

impl RouteOptimizerApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self::default();

        app.concurrent = config::DEFAULT_PARAREL_REQUEST;

        app
    }
}

impl eframe::App for RouteOptimizerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                egui::global_theme_preference_buttons(ui);
                ui.label(
                    egui::RichText::new(config::APP_NAME).size(24f32)
                );
            });
            ui.separator();

            ui.label(
                egui::RichText::new("Arguments").size(18f32)
            );
            egui::Grid::new("arguments")
                .show(ui, |ui| {
                    ui.label("start");
                    ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.start));
                    ui.end_row();

                    ui.label("route");
                    ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.route));
                    ui.end_row();

                    ui.label("end");
                    ui.add_sized(ui.available_size(), egui::TextEdit::singleline(&mut self.end));
                    ui.end_row();

                    ui.label("concurrent");
                    ui.add_sized(ui.available_size(), egui::Slider::new(&mut self.concurrent, 1..=100));
                    ui.end_row();

                    ui.label("route option");
                    egui::ComboBox::new("arguments_route-option", std::iter::repeat(" ").take(100).collect::<Vec<&str>>().join(""))
                        .selected_text(format!("{}", self.route_option))
                        .width(ui.available_size().x * 0.8f32)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.route_option, RouteOption::Fastest, "Fastest");
                            ui.selectable_value(&mut self.route_option, RouteOption::Highsec, "Highsec");
                            ui.selectable_value(&mut self.route_option, RouteOption::LowNull, "Lowsec and Nullsec");
                        }
                    );
                    ui.end_row();
                });

            ui.separator();

            if ui.button("calculate").clicked() {
                let args_result = Args::builder()
                    .set_start(&self.start)
                    .set_route(&self.route)
                    .set_end(&self.end)
                    .set_route_option(&self.route_option)
                    .set_concurrent(self.concurrent)
                    .build();

                match args_result {
                    Ok(args) => {
                        let _runner_joinhandle = crossbeam::thread::scope(|s| {
                            let buffer = self.buffer.clone();
                            s.spawn(move |_| {
                                run(args, buffer);
                            });
                        });
                    },
                    Err(e) => {
                        self.result = format!("{e}");
                    },
                }
            }

            self.result = self.buffer.read().unwrap().to_string();
            ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.result));
        });
    }
}

fn run(args: Args, buffer: Arc<RwLock<String>>) {
    let (recv, manager) = RouteOptimizeManager::with_args(args);
    let joinhandle = std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .worker_threads(2)
            .thread_name("RouteOptimizeManager Parallel Runner")
            .enable_all()
            .build()
            .unwrap();

        let _ = runtime.block_on( async {
            manager.run().await
        });
    });
    joinhandle.join().unwrap();

    let mut current_shortest: CurrentShortest = CurrentShortest::new();
    for response in recv.iter() {
        if let ManagerResponse::Err(_) = response {
            let mut wlock = buffer.write().unwrap();
            wlock.clear();
            wlock.push_str(&trace::string::error(response.to_string()));
        } else {
            let mut wlock = buffer.write().unwrap();
            wlock.clear();
            wlock.push_str(&trace::string::info(response.to_string()));
        };

        if let ManagerResponse::Ok(returned_current_shortest) = response {
            current_shortest = returned_current_shortest;
            break;
        }
    };

    let mut wlock = buffer.write().unwrap();
    wlock.clear();
    wlock.push_str(&format!("{:?}", current_shortest.to_named()));
}
