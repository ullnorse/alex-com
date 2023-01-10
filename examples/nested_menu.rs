use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    );
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Menus testing");

            ui.menu_button("Edit", |ui| {
                let _ = ui.button("Copy");
                let _ = ui.button("Cut");
                let _ = ui.button("Paste");
                ui.separator();
                if ui.button("Close").clicked() {
                    ui.close_menu();
                }
            });
        });
    }
}
