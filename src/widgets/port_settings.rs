use egui::{ComboBox, Grid, Response, Widget};

pub struct PortSettingsWindow<'a> {
    serial_devices: &'a Vec<String>,
    serial_device: &'a mut String,

    baudrates: Vec<u32>,
    baudrate: &'a mut u32,

    data_bits: [u8; 4],
    selected_data_bits: &'a mut u8,

    stop_bits: [u8; 2],
    selected_stop_bits: &'a mut u8,

    parity: [String; 3],
    selected_parity: &'a mut String,

    flow_control: [String; 3],
    selected_flow_control: &'a mut String,

    local_echo: &'a mut bool,
}

impl<'a> PortSettingsWindow<'a> {
    pub fn new(
        serial_device: &'a mut String,
        serial_devices: &'a Vec<String>,
        baudrate: &'a mut u32,
        selected_data_bits: &'a mut u8,
        selected_stop_bits: &'a mut u8,
        selected_parity: &'a mut String,
        selected_flow_control: &'a mut String,
        local_echo: &'a mut bool,
    ) -> Self {
        Self {
            serial_device,
            serial_devices,
            baudrate,
            baudrates: vec![9600, 115200, 1000000],
            data_bits: [5, 6, 7, 8],
            selected_data_bits,
            stop_bits: [1, 2],
            selected_stop_bits,
            parity: ["None".to_string(), "Odd".to_string(), "Even".to_string()],
            selected_parity,
            flow_control: [
                "None".to_string(),
                "Software".to_string(),
                "Hardware".to_string(),
            ],
            selected_flow_control,
            local_echo,
        }
    }
}

impl<'a> Widget for PortSettingsWindow<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        Grid::new("grid")
            .show(ui, |ui| {
                ui.label("Device");
                ComboBox::from_id_source("device")
                    .selected_text(self.serial_device.to_string())
                    .show_ui(ui, |ui| {
                        for device in self.serial_devices {
                            ui.selectable_value(self.serial_device, device.clone(), device.clone());
                        }
                    });
                ui.end_row();

                ui.label("Baud Rate");
                ComboBox::from_id_source("baudrate")
                    .selected_text(format!("{}", *self.baudrate as i32))
                    .show_ui(ui, |ui| {
                        for baudrate in &self.baudrates {
                            ui.selectable_value(self.baudrate, *baudrate, baudrate.to_string());
                        }
                    });
                ui.end_row();

                ui.label("Data bits");
                ComboBox::from_id_source("databits")
                    .selected_text(format!("{}", self.selected_data_bits))
                    .show_ui(ui, |ui| {
                        for data_bits in &self.data_bits {
                            ui.selectable_value(
                                self.selected_data_bits,
                                *data_bits,
                                data_bits.to_string(),
                            );
                        }
                    });
                ui.end_row();

                ui.label("Stop Bits");
                ComboBox::from_id_source("stopbits")
                    .selected_text(format!("{}", self.selected_stop_bits))
                    .show_ui(ui, |ui| {
                        for stop_bits in &self.stop_bits {
                            ui.selectable_value(
                                self.selected_stop_bits,
                                *stop_bits,
                                stop_bits.to_string(),
                            );
                        }
                    });
                ui.end_row();

                ui.label("Parity");
                ComboBox::from_id_source("parity")
                    .selected_text(self.selected_parity.to_string())
                    .show_ui(ui, |ui| {
                        for parity in self.parity {
                            ui.selectable_value(
                                self.selected_parity,
                                parity.clone(),
                                parity.clone(),
                            );
                        }
                    });
                ui.end_row();

                ui.label("Flow control");
                ComboBox::from_id_source("flowcontrol")
                    .selected_text(self.selected_flow_control.to_string())
                    .show_ui(ui, |ui| {
                        for flow_control in &self.flow_control {
                            ui.selectable_value(
                                self.selected_flow_control,
                                flow_control.clone(),
                                flow_control.clone(),
                            );
                        }
                    });
                ui.end_row();

                ui.label("Local Echo");
                ui.checkbox(self.local_echo, "");
                ui.end_row();
            })
            .response
    }
}
