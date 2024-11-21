use eframe::egui;
use std::sync::{ Arc, RwLock };

use crate::{
    config,
    args::Args,
    route::RouteOption,
    system::{
        CurrentShortest,
        SystemHolder,
    },
    progress::ProgressHolder,
    request::make_requests,
};



#[derive(Default)]
pub struct RouteOptimizerApp {
    route: String,
    start: String,
    end: String,
    route_option: RouteOption,
    concurrent: usize,

    result: String,
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
            egui::global_theme_preference_switch(ui);

            ui.vertical_centered(|ui| ui.label(
                egui::RichText::new(config::APP_NAME).size(24f32)
            ));
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

                    ui.label("route option");
                    egui::ComboBox::new("arguments_route-option", std::iter::repeat(" ").take(100).collect::<Vec<&str>>().join(""))
                        .selected_text(format!("{}", self.route_option))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.route_option, RouteOption::Fastest, "Fastest");
                            ui.selectable_value(&mut self.route_option, RouteOption::Highsec, "Highsec");
                            ui.selectable_value(&mut self.route_option, RouteOption::LowNull, "Lowsec and Nullsec");
                        }
                    );
                    ui.end_row();

                    ui.label("concurrent");
                    ui.add_sized(ui.available_size(), egui::Slider::new(&mut self.concurrent, 1..=100).text(""));
                    ui.end_row();
                });

            ui.separator();

            if ui.button("calculate").clicked() {
                let args = Args::builder()
                    .set_start(&self.start)
                    .set_route(&self.route)
                    .set_end(&self.end)
                    .set_route_option(&self.route_option)
                    .set_concurrent(self.concurrent)
                    .build();

                if let Ok(args) = args {
                    let (current_shortest, progress_holder) = tokio::task::block_in_place(move || {
                        tokio::runtime::Handle::current().block_on(async move {
                            calculate(args).await
                        })
                    });
                    self.result = format!("{}\n\n{:#?}", progress_holder.state, current_shortest);
                } else {
                    self.result = "Format Error".to_string();
                }
            }

            ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.result));
        });
    }
}

async fn calculate(args: Args) -> (CurrentShortest, ProgressHolder) {
    // Alloc NEW
    let mut system_holder = SystemHolder::new();
    let progress_holder = RwLock::new(ProgressHolder::new());

    // Initialize by given `args`
    system_holder.register_route(&args.route);
    system_holder.register_system(&args.start);
    match &args.end {
        Some(system) => {
            system_holder.register_system(&system);
        },
        _ => (),
    }

    // make inter-system-distance requests
    make_requests(&args, &system_holder).await;

    let calculation_count: u128 = system_holder.permutation_size_hint().unwrap_or(u128::MAX);
    progress_holder.write().unwrap().set_total(calculation_count);

    let feedback_step: usize = std::cmp::min(1_000_000, std::cmp::max(1, calculation_count/200) as usize);
    let current_shortest = system_holder.build_shortest_path( &args, &progress_holder, feedback_step );

    let current_shortest = Arc::into_inner(current_shortest).unwrap();
    let current_shortest = current_shortest.into_inner().unwrap();

    let progress_holder = progress_holder.into_inner().unwrap();

    (current_shortest, progress_holder)
}
