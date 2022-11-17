use eframe::egui;

use egui::{Button, Color32, Frame, Style, TextBuffer, Visuals, Window};

mod serial;
mod widgets;

use native_dialog::{FileDialog, Filter, MessageDialog, MessageType};
use serial::Serial;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use widgets::line_end_picker::{LineEnd, LineEndPicker};
use widgets::port_settings::PortSettings;

const ICON: &[u8; 172598] = include_bytes!("../alex-com.ico");

fn main() {
    let options = eframe::NativeOptions {
        icon_data: Some(eframe::IconData {
            rgba: ICON.to_vec(),
            width: 128,
            height: 128,
        }),
        ..Default::default()
    };

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

    tx_cnt: u32,
    rx_cnt: u32,

    recording_started: bool,
    log_file_name: String,
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

            tx_cnt: 0,
            rx_cnt: 0,

            recording_started: false,
            log_file_name: String::new(),
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
            ui.horizontal(|ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.label(format!(
                    "{} {} | {}, {}{}{} flow control: {}       TX: {}, RX: {}       {}",
                    self.selected_serial_device,
                    if self.device_connected {
                        "OPENED"
                    } else {
                        "CLOSED"
                    },
                    self.baudrate,
                    self.selected_data_bits,
                    self.selected_parity.char_range(0..1),
                    self.selected_stop_bits,
                    self.selected_flow_control,
                    self.tx_cnt,
                    self.rx_cnt,
                    if self.recording_started {
                        format!("Logging to: {}", self.log_file_name)
                    } else {
                        String::new()
                    }
                ));
            })
        });

        egui::CentralPanel::default()
            .frame(Frame::default().fill(Color32::from_rgb(229, 228, 226)))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        if !self.device_connected {
                            if ui
                                .add(Button::new("Connect").fill(Color32::LIGHT_BLUE))
                                .clicked()
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
                                self.tx_cnt = 0;
                                self.rx_cnt = 0;
                            }
                        } else if ui
                            .add(Button::new("Disconnect").fill(Color32::LIGHT_BLUE))
                            .clicked()
                        {
                            self.serial.stop();
                            self.device_connected = false;
                        }

                        if self.recording_started {
                            if ui
                                .add_sized(
                                    [50f32, 20f32],
                                    Button::new("Stop").fill(Color32::LIGHT_BLUE),
                                )
                                .clicked()
                            {
                                self.recording_started = false;
                                self.log_file_name.clear();
                            }
                        } else if ui
                            .add_sized(
                                [50f32, 20f32],
                                Button::new("Record").fill(Color32::LIGHT_BLUE),
                            )
                            .clicked()
                        {
                            let path = FileDialog::new()
                                .set_location(dirs::home_dir().unwrap().to_str().unwrap())
                                .show_save_single_file()
                                .unwrap();

                            if let Some(path) = path {
                                self.recording_started = true;
                                self.log_file_name = path.to_str().unwrap().to_string();
                                _ = File::create(path);
                            }
                        }

                        if ui
                            .add_enabled(
                                !self.device_connected,
                                egui::Button::new("Open port settings").fill(Color32::LIGHT_BLUE),
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
                    });

                    egui::containers::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .stick_to_bottom(true)
                        .max_height(400f32)
                        .show_viewport(ui, |ui, _viewport| {
                            ui.add_sized(
                                [ui.available_width(), 400f32],
                                egui::TextEdit::multiline(&mut self.current_text)
                                    .interactive(false),
                            );
                        });

                    ui.horizontal(|ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::LEFT), |ui| {
                            ui.add(LineEndPicker::new(70f32, &mut self.line_end));

                            if ui
                                .add_sized(
                                    [50f32, 20f32],
                                    Button::new("send").fill(Color32::LIGHT_BLUE),
                                )
                                .clicked()
                            {
                                let mut s = self.send_text.clone();
                                s.push_str(self.line_end.to_value());
                                println!("{:?}", s);

                                self.tx_cnt += s.len() as u32;

                                self.serial.output_channel.0.send(s).unwrap();
                            }

                            if ui
                                .add_sized(
                                    [50f32, 20f32],
                                    Button::new("clear").fill(Color32::LIGHT_BLUE),
                                )
                                .clicked()
                            {
                                self.current_text.clear();
                            }

                            ui.add_sized(
                                ui.available_size(),
                                egui::TextEdit::singleline(&mut self.send_text),
                            );
                        });
                    });
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

                if let Ok(s) = self.serial.get_receiver().try_recv() {
                    self.current_text.push_str(&s);
                    self.rx_cnt += s.len() as u32;

                    if self.recording_started {
                        if let Ok(mut file) = OpenOptions::new()
                            .create(false)
                            .append(true)
                            .write(true)
                            .open(&self.log_file_name)
                        {
                            file.write_all(s.as_bytes()).ok();
                        }
                    }
                }
            });

        ctx.request_repaint();
    }
}
