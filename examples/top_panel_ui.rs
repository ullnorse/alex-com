use eframe::egui;

use egui::{ComboBox, Grid, Window};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

#[derive(Default)]
struct MyApp {
    radio_value: Baudrates,
    data_bits: i32,

    port_settings_open: bool,

    serial_devices: Vec<String>,
    selected_serial_device: String,
}

#[derive(Copy, Clone, PartialEq)]
enum Baudrates {
    Baudrate600 = 600,
    Baudrate1200 = 1200,
    Baudrate2400 = 2400,
    Baudrate4800 = 4800,
    Baudrate9600 = 9600,

    Baudrate14400 = 14400,
    Baudrate19200 = 19200,
    Baudrate28800 = 28800,
    Baudrate38400 = 38400,
    Baudrate56000 = 56000,

    Baudrate57600 = 57600,
    Baudrate115200 = 115200,
    Baudrate128000 = 128000,
    Baudrate256000 = 256000,
    BaudrateCustom = -1,
}

impl Default for Baudrates {
    fn default() -> Self {
        Baudrates::Baudrate115200
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Menus testing");

            // ui.horizontal(|ui| {
            //     ui.vertical(|ui| {
            //         ui.group(|ui| {
            //             Grid::new("baudrate grid").show(ui, |ui| {
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate600, "600");
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate14400, "14000");
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate57600, "57600");
            //                 ui.end_row();

            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate1200, "1200");
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate19200, "19200");
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate115200, "115200");

            //                 ui.end_row();
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate2400, "2400");
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate28800, "28800");
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate128000, "128000");

            //                 ui.end_row();
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate4800, "4800");
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate38400, "38400");
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate256000, "256000");

            //                 ui.end_row();
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate9600, "9600");
            //                 ui.radio_value(&mut self.radio_value, Baudrates::Baudrate56000, "56000");
            //                 ui.radio_value(&mut self.radio_value, Baudrates::BaudrateCustom, "Custom");
            //             });
            //         });
            //     });
            //     ui.group(|ui| {
            //         ui.vertical(|ui| {
            //             ui.radio_value(&mut self.data_bits, 5, "5");
            //             ui.radio_value(&mut self.data_bits, 6, "6");
            //             ui.radio_value(&mut self.data_bits, 7, "7");
            //             ui.radio_value(&mut self.data_bits, 8, "8");
            //         });
            //     });
            // });

            ui.label(format!("baudrate = {}", self.radio_value as i32));

            if ui.button("Open port settings").clicked() {
                self.serial_devices = (0..5).map(|x| format!("/dev/ttyUSB{}", x)).collect();
                self.selected_serial_device = self.serial_devices[0].clone();
                self.port_settings_open = true;
            }

            Window::new("Port Setup")
                .collapsible(false)
                .resizable(false)
                .open(&mut self.port_settings_open)
                .show(ctx, |ui| {
                    Grid::new("grid").show(ui, |ui| {
                        ui.label("Device");
                        ComboBox::from_id_source("device")
                            .selected_text(self.selected_serial_device.clone())
                            .show_ui(ui, |ui| {
                                for device in &self.serial_devices {
                                    ui.selectable_value(
                                        &mut self.selected_serial_device,
                                        device.to_string(),
                                        device,
                                    );
                                }
                            });
                        ui.end_row();

                        ui.label("Baud Rate");
                        ComboBox::from_id_source("baudrate")
                            .selected_text(format!("{}", self.radio_value as i32))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate9600,
                                    "9600",
                                );
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate115200,
                                    "115200",
                                );
                            });
                        ui.end_row();

                        ui.label("Data bits");
                        ComboBox::from_id_source("databits")
                            .selected_text(format!("{}", self.radio_value as i32))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate9600,
                                    "9600",
                                );
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate115200,
                                    "115200",
                                );
                            });
                        ui.end_row();

                        ui.label("Stop Bits");
                        ComboBox::from_id_source("stopbits")
                            .selected_text(format!("{}", self.radio_value as i32))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate9600,
                                    "9600",
                                );
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate115200,
                                    "115200",
                                );
                            });
                        ui.end_row();

                        ui.label("Parity");
                        ComboBox::from_id_source("parity")
                            .selected_text(format!("{}", self.radio_value as i32))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate9600,
                                    "9600",
                                );
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate115200,
                                    "115200",
                                );
                            });
                        ui.end_row();

                        ui.label("Handshake");
                        ComboBox::from_id_source("handshake")
                            .selected_text(format!("{}", self.radio_value as i32))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate9600,
                                    "9600",
                                );
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate115200,
                                    "115200",
                                );
                            });
                        ui.end_row();

                        ui.label("Access Mode");
                        ComboBox::from_id_source("access mode")
                            .selected_text(format!("{}", self.radio_value as i32))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate9600,
                                    "9600",
                                );
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate115200,
                                    "115200",
                                );
                            });
                        ui.end_row();

                        ui.label("Local Echo");
                        ComboBox::from_id_source("localecho")
                            .selected_text(format!("{}", self.radio_value as i32))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate9600,
                                    "9600",
                                );
                                ui.selectable_value(
                                    &mut self.radio_value,
                                    Baudrates::Baudrate115200,
                                    "115200",
                                );
                            });
                    });

                    // ui.vertical(|ui| {
                    //     ui.label("Port Settings");
                    //     ui.horizontal(|ui| {
                    //         ui.label("Device");
                    //         ComboBox::from_id_source("device1")
                    //             .selected_text(format!("{}", self.radio_value as i32))
                    //             .show_ui(ui, |ui| {
                    //                 ui.selectable_value(&mut self.radio_value, Baudrates::Baudrate9600, "9600");
                    //                 ui.selectable_value(&mut self.radio_value, Baudrates::Baudrate115200, "115200");
                    //             })
                    //     });
                    //     ui.horizontal(|ui| {
                    //         ui.label("Baud Rate");
                    //         ComboBox::from_id_source("baudrate")
                    //             .selected_text(format!("{}", self.radio_value as i32))
                    //             .show_ui(ui, |ui| {
                    //                 ui.selectable_value(&mut self.radio_value, Baudrates::Baudrate9600, "9600");
                    //                 ui.selectable_value(&mut self.radio_value, Baudrates::Baudrate115200, "115200");
                    //             })
                    //     });
                    //     ui.horizontal(|ui| {
                    //         ui.label("Data bits");
                    //         ComboBox::from_id_source("data bits")
                    //             .selected_text(format!("{}", self.radio_value as i32))
                    //             .show_ui(ui, |ui| {
                    //                 ui.selectable_value(&mut self.radio_value, Baudrates::Baudrate9600, "9600");
                    //                 ui.selectable_value(&mut self.radio_value, Baudrates::Baudrate115200, "115200");
                    //             })
                    //     });
                    // })
                })
        });
    }
}
