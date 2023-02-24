#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod planet;
use planet::{Planet, TimeUnit};
use eframe::{egui, Frame};
use egui::{Context, InnerResponse, Ui, Vec2, vec2};
use std::default::Default;
use eframe::epaint::Color32;
use egui::RichText;

fn main() -> Result<(), eframe::Error> {
    let planets = Planet::get_all_planets();

    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Orbital Windows",
        options,
        Box::new(|cc| Box::new(MyApp::new(planets)))
    )
}

struct MyApp {
    planets: [Planet; 7],
    current_planet: String,
    selected_planet: String,
    current_year: String,
    current_day: String
}

impl MyApp {
    pub fn new(planets: [Planet; 7]) -> Self {
        let current_planet = "Kerbin".to_string();
        let selected_planet = "Duna".to_string();
        Self {
            planets,
            current_planet,
            selected_planet,
            current_year: "0".to_string(),
            current_day: "0".to_string()
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: Set the spacing here - ui.set_style() <- this needs some style struct
            ui.heading("Orbital Window Calculator");
            ui.horizontal(|ui| {
                let name_label = ui.label("Current Planet: ");
                egui::ComboBox::from_label("Select current planet")
                    .selected_text(format!("{}", self.current_planet))
                    .show_ui(ui, |ui| {
                        for planet in &self.planets {
                            ui.selectable_value(&mut self.current_planet, planet.name.clone(), &planet.name);
                        }
                    });
            });
            ui.horizontal(|ui| {
                let name_label = ui.label("Selected Planet: ");
                egui::ComboBox::from_label("Select target planet")
                    .selected_text(format!("{}", self.selected_planet))
                    .show_ui(ui, |ui| {
                        for planet in &self.planets {
                            ui.selectable_value(&mut self.selected_planet, planet.name.clone(), &planet.name);
                        }
                    });
            });
            ui.horizontal(|ui| {
                ui.label("Current year: ");
                ui.add_sized(vec2(100f32, 18f32), egui::widgets::TextEdit::singleline(&mut self.current_year));
                ui.label("Current day: ");
                ui.add_sized(vec2(100f32, 18f32), egui::widgets::TextEdit::singleline(&mut self.current_day));
            });
            ui.horizontal(|ui| {
                if self.selected_planet != self.current_planet {
                    ui.label(format!("Transferring from {} to {}", self.current_planet, self.selected_planet));
                }
                else {
                    ui.label("Cannot transfer from a planet to itself.");
                }
            });
            egui::Grid::new("planet-data-grid")
                .striped(true)
                .spacing(vec2(50f32, 0f32))
                .show(ui, |ui| {
                    ui.label("Planet Name");
                    ui.label("Transfer Window Every x Days");
                    ui.label("Days until next transfer window begins");
                    ui.label("Days until optimal launch");
                    ui.end_row();
                    for planet in &self.planets {
                        if planet.name == self.selected_planet {
                            ui.label(RichText::new(format!("{}", planet.name)).color(Color32::RED));
                        }
                        else {
                            ui.label(format!("{}", planet.name));
                        }
                        let transfer_window = Planet::get_orbital_period(&self.planets, &self.current_planet, &planet.name, TimeUnit::Seconds);
                        ui.label(format!("{:.1}", Planet::convert_time_from_seconds(transfer_window, TimeUnit::Days)));
                        let next_transfer_window = Planet::get_next_transfer_window(&self.current_year, &self.current_day, transfer_window, TimeUnit::Days);
                        ui.label(format!("{:.1}", next_transfer_window));
                        ui.label("TODO");
                        ui.end_row();
                    }
                });
        });
    }
}
