use eframe::egui;

use egui::{Style, TextBuffer, Visuals, Window, Button};

mod serial;
mod widgets;

use serial::Serial;
use widgets::port_settings::PortSettings;
use widgets::line_end_picker::{LineEnd, LineEndPicker};

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Alex-Com",
        options,
        Box::new(|_cc| {
            let style = Style {
                visuals: Visuals::light(),
                ..Style::default()
            };
            _cc.egui_ctx.set_style(style);
            Box::new(MyApp::new())
        }),
    );
}

struct MyApp {
    baudrate: u32,
    selected_data_bits: u8,
    selected_stop_bits: u8,
    selected_parity: String,
    selected_flow_control: String,
    local_echo: bool,

    port_settings_open: bool,

    serial_devices: Vec<String>,
    selected_serial_device: String,

    current_text: String,

    device_connected: bool,

    serial: Serial,

    send_text: String,
    line_end: LineEnd,
}

impl MyApp {
    fn new() -> Self {
        let mut app = Self {
            baudrate: 115200,
            selected_data_bits: 8,
            selected_stop_bits: 1,
            selected_parity: "None".to_string(),
            selected_flow_control: "None".to_string(),
            local_echo: false,

            port_settings_open: false,
            serial_devices: Serial::available_ports(),
            selected_serial_device: Default::default(),

            current_text: String::new(),
            device_connected: false,

            serial: Serial::new(),

            send_text: String::new(),

            line_end: LineEnd::default(),
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
            ui.add_space(30f32);
            ui.horizontal(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.label(format!(
                    "{} | {}, {}{}{} flow control: {}",
                    self.selected_serial_device,
                    self.baudrate,
                    self.selected_data_bits,
                    self.selected_parity.char_range(0..1),
                    self.selected_stop_bits,
                    self.selected_flow_control,
                ));
            })
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                if !self.device_connected {
                    if ui.button("Connect").clicked()
                        && self
                            .serial
                            .start(serial::SerialConfig {
                                port: self.selected_serial_device.clone(),
                                baudrate: self.baudrate,
                                ..Default::default()
                            })
                            .is_ok()
                    {
                        self.device_connected = true;
                    }
                } else if ui.button("Disconnect").clicked() {
                    self.serial.stop();
                    self.device_connected = false;
                }

                if ui
                    .add_enabled(
                        !self.device_connected,
                        egui::Button::new("Open port settings"),
                    )
                    .clicked()
                {
                    self.serial_devices = Serial::available_ports();
                    self.selected_serial_device = if self.serial_devices.is_empty() {
                        "".to_string()
                    } else {
                        self.serial_devices[0].clone()
                    };

                    self.port_settings_open = true;
                }

                if ui.button("Clear").clicked() {
                    self.current_text.clear();
                }
            });

            Window::new("Port Setup")
                .collapsible(false)
                .resizable(false)
                .open(&mut self.port_settings_open)
                .show(ctx, |ui| {
                    ui.add(PortSettings::new(
                        &mut self.selected_serial_device,
                        &self.serial_devices,
                        &mut self.baudrate,
                        &mut self.selected_data_bits,
                        &mut self.selected_stop_bits,
                        &mut self.selected_parity,
                        &mut self.selected_flow_control,
                        &mut self.local_echo,
                    ));
                });

            egui::containers::ScrollArea::vertical()
                .auto_shrink([false, false])
                .stick_to_bottom(true)
                .show_viewport(ui, |ui, _viewport| {
                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::multiline(&mut self.current_text).interactive(false),
                    );
                });

            if let Ok(s) = self.serial.get_receiver().try_recv() {
                self.current_text.push_str(&s);
            }

            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                    ui.add_sized((30f32, ui.available_height()), LineEndPicker::new(&mut self.line_end));
                    
                    if ui.add_sized((80f32, ui.available_height()), Button::new("send")).clicked() {
                        let mut s = self.send_text.clone();
                        s.push('\n');
                        self.serial.output_channel.0.send(s).unwrap();
                    }

                    ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut self.send_text),
                    );
                });
            });
        });

        ctx.request_repaint();
    }
}
