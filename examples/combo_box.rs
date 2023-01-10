use eframe::egui;
use egui::widgets::{Slider};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    );
}

#[derive(Debug, PartialEq)]
enum Baudrates {
    Baud9600 = 9600,
    Baud115200 = 115200,
}

impl Default for Baudrates {
    fn default() -> Self {
        Baudrates::Baud115200
    }
}

impl std::fmt::Display for Baudrates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Baudrates::Baud9600 => write!(f, "9600"),
            Baudrates::Baud115200 => write!(f, "115200"),
        }
    }
}

#[derive(Default)]
struct MyApp {
    allowed_to_close: bool,
    show_confirmation_dialog: bool,

    age: i32,
    name: String,
    enum_value: i32,

    baudrate: Baudrates,
}

struct State {}

impl eframe::App for MyApp {
    fn on_close_event(&mut self) -> bool {
        // self.show_confirmation_dialog = true;
        // self.allowed_to_close
        true
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Combo box testing");

            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });

            ui.add(Slider::new(&mut self.age, 0..=100));

            ui.horizontal(|ui| {
                if ui.button("click me").clicked() {
                    self.age += 1;
                }

                if ui.button("click me").clicked() {
                    self.age -= 1;
                }
            });

            ui.label(format!("age = {}, name = {}", self.age, self.name));

            ui.separator();

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.enum_value, 0, "0");
                ui.radio_value(&mut self.enum_value, 1, "1");
                ui.radio_value(&mut self.enum_value, 2, "2");
                ui.radio_value(&mut self.enum_value, 3, "3");
            });

            ui.label(format!("radio value = {}", self.enum_value));

            egui::ComboBox::from_label("Baudrates")
                .selected_text(format!("{}", self.baudrate))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.baudrate,
                        Baudrates::Baud9600,
                        format!("{}", Baudrates::Baud9600),
                    );
                    ui.selectable_value(
                        &mut self.baudrate,
                        Baudrates::Baud115200,
                        format!("{}", Baudrates::Baud115200),
                    );
                });

            ui.label(format!("radio value = {}", self.baudrate));
        });

        // if self.show_confirmation_dialog {
        //     // Show confirmation dialog:
        //     egui::Window::new("Do you want to quit?")
        //         .collapsible(false)
        //         .resizable(false)
        //         .show(ctx, |ui| {
        //             ui.horizontal(|ui| {
        //                 if ui.button("Cancel").clicked() {
        //                     self.show_confirmation_dialog = false;
        //                 }

        //                 if ui.button("Yes!").clicked() {
        //                     self.allowed_to_close = true;
        //                     frame.close();
        //                 }
        //             });
        //         });
        // }
    }
}
