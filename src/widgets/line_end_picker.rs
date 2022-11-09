use egui::{ComboBox, Response, Widget};

#[derive(PartialEq, Eq, Clone)]
pub enum LineEnd {
    LF,
    CR,
    CrLf,
    None,
}

impl Default for LineEnd {
    fn default() -> Self {
        LineEnd::LF
    }
}

impl ToString for LineEnd {
    fn to_string(&self) -> String {
        match self {
            LineEnd::LF => "LF".to_string(),
            LineEnd::CR => "CR".to_string(),
            LineEnd::CrLf => "CR + LF".to_string(),
            LineEnd::None => "None".to_string(),
        }
    }
}

impl LineEnd {
    pub fn to_value(&self) -> &'static str {
        match self {
            LineEnd::LF => "\n",
            LineEnd::CR => "\r",
            LineEnd::CrLf => "\r\n",
            LineEnd::None => "",
        }
    }
}

pub struct LineEndPicker<'a> {
    width: f32,
    line_ends: [LineEnd; 4],
    line_end: &'a mut LineEnd,
}

impl<'a> LineEndPicker<'a> {
    pub fn new(width: f32, line_end: &'a mut LineEnd) -> Self {
        Self {
            width,
            line_ends: [LineEnd::LF, LineEnd::CR, LineEnd::CrLf, LineEnd::None],
            line_end,
        }
    }
}

impl<'a> Widget for LineEndPicker<'a> {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ComboBox::from_id_source("line_end")
            .selected_text(self.line_end.to_string())
            .width(self.width)
            .show_ui(ui, |ui| {
                for line_end in self.line_ends {
                    ui.selectable_value(self.line_end, line_end.clone(), line_end.to_string());
                }
            })
            .response
    }
}
