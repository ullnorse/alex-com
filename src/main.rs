use eframe::egui;

use egui::{Grid, Window, ComboBox, TextBuffer};

mod serial;

use serial::Serial;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Alex-Com",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}

struct MyApp {
    baudrates: Vec<u32>,
    baudrate: u32,

    data_bits: [u8; 4],
    selected_data_bits: u8,

    stop_bits: [u8; 2],
    selected_stop_bits: u8,

    parity: [String; 3],
    selected_parity: String,

    flow_control: [String; 3],
    selected_flow_control: String,

    local_echo: bool,

    port_settings_open: bool,

    serial_devices: Vec<String>,
    selected_serial_device: String,

    current_text: String,

    device_connected: bool,

    serial: Serial,
}

impl MyApp {
    fn new() -> Self {
        let mut app = Self {
            baudrates: vec![9600, 115200, 1000000],
            baudrate: 115200,
            data_bits: [5, 6, 7, 8],
            selected_data_bits: 8,

            stop_bits: [1, 2],
            selected_stop_bits: 1,

            parity: ["None".to_string(), "Odd".to_string(), "Even".to_string()],
            selected_parity: "None".to_string(),

            flow_control: ["None".to_string(), "Software".to_string(), "Hardware".to_string()],
            selected_flow_control: "None".to_string(),

            local_echo: false,

            port_settings_open: false,
            serial_devices: Serial::available_ports(),
            selected_serial_device: Default::default(),

            current_text: "".to_string(),
            device_connected: false,

            serial: Serial::new(),
        };

        app.selected_serial_device = if app.serial_devices.is_empty() {
            "".to_string()
        } else {
            app.serial_devices[0].clone()
        };

        app
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("my_panel").show(ctx, |ui| {
            ui.label(format!("{} | {}, {}{}{} flow control: {}",
            self.selected_serial_device,
            self.baudrate,
            self.selected_data_bits,
            self.selected_parity.char_range(0..1),
            self.selected_stop_bits,
            self.selected_flow_control,
            ));

            egui::widgets::global_dark_light_mode_switch(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                if !self.device_connected {
                    if ui.button("Connect").clicked() && self.serial.start( serial::SerialConfig { 
                            port: self.selected_serial_device.clone(),
                            baudrate: self.baudrate,
                            ..Default::default() } ).is_ok() {
                        self.device_connected = true;
                    }
                } else if ui.button("Disconnect").clicked() {
                    self.serial.stop();
                    self.device_connected = false;
                }

                if ui.button("Open port settings").clicked() {
                    self.serial_devices = Serial::available_ports();
                    self.selected_serial_device = if self.serial_devices.is_empty() {
                        "".to_string()
                    } else {
                        self.serial_devices[0].clone()
                    };

                    self.port_settings_open = true;
                }
            });

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
                                    ui.selectable_value(&mut self.selected_serial_device, device.to_string(), device);
                                }
                            });
                        ui.end_row();

                        ui.label("Baud Rate");
                        ComboBox::from_id_source("baudrate")
                            .selected_text(format!("{}", self.baudrate as i32))
                            .show_ui(ui, |ui| {
                                for baudrate in &self.baudrates {
                                    ui.selectable_value(&mut self.baudrate, *baudrate, baudrate.to_string());
                                }
                            });
                        ui.end_row();

                        ui.label("Data bits");
                        ComboBox::from_id_source("databits")
                            .selected_text(format!("{}", self.selected_data_bits))
                            .show_ui(ui, |ui| {
                                for data_bits in self.data_bits {
                                    ui.selectable_value(&mut self.selected_data_bits, data_bits, data_bits.to_string());
                                }
                            });
                        ui.end_row();

                        ui.label("Stop Bits");
                        ComboBox::from_id_source("stopbits")
                            .selected_text(format!("{}", self.selected_stop_bits))
                            .show_ui(ui, |ui| {
                                for stop_bits in self.stop_bits {
                                    ui.selectable_value(&mut self.selected_stop_bits, stop_bits, stop_bits.to_string());
                                }
                            });
                        ui.end_row();

                        ui.label("Parity");
                        ComboBox::from_id_source("parity")
                            .selected_text(&self.selected_parity)
                            .show_ui(ui, |ui| {
                                for parity in &self.parity {
                                    ui.selectable_value(&mut self.selected_parity, parity.clone(), parity);
                                }
                            });
                        ui.end_row();

                        ui.label("Flow control");
                        ComboBox::from_id_source("flowcontrol")
                            .selected_text(&self.selected_flow_control)
                            .show_ui(ui, |ui| {
                                for flow_control in &self.flow_control {
                                    ui.selectable_value(&mut self.selected_flow_control, flow_control.clone(), flow_control);
                                }
                            });
                        ui.end_row();

                        ui.label("Local Echo");
                        ui.checkbox(&mut self.local_echo, "");
                        ui.end_row();

                    });
                });

            egui::containers::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show_viewport(ui, |ui, _viewport| {
                    ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.current_text).interactive(false));
                });

            if ui.input().key_pressed(egui::Key::Backspace) {
                self.current_text.clear();
            }

            if let Ok(s) = self.serial.get_receiver().try_recv() {
                self.current_text.push_str(&s);
                print!("{s}");
            }

            ctx.request_repaint();
        });
    }
}
