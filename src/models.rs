use macroquad::{color::Color, math::Rect, text::Font};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub ascii_art: Vec<String>,
    pub about: Vec<String>,
    pub url: String,
}

#[derive(Debug)]
pub struct Panel {
    pub text: Vec<String>,
    pub index: usize,
    pub width: usize,
    pub height: usize,
    pub offset_x: usize,
    pub offset_y: usize
}

#[derive(Default)]
pub struct TerminalState {
    pub cell_buffer: Vec<Vec<Cell>>,
    pub line_buffer: Vec<String>,
    pub highlighted_boxes: Vec<Rect>,
    pub projects: Vec<ProjectInfo>,
    pub line_index: usize,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub font_size: f32,
    pub terminal_width_px: f32,
    pub terminal_height_px: f32,
    pub sub_state: TermSubState,
    pub font: Option<Font>,
}

pub enum TermSubState {
    Load {
        step: usize,
        timer: f32,
    },
    Main {
        index: usize
    },
    Projects {
        selected_project_index: usize,
        project_about_scroll: usize,
        main_focus: bool,
        panels: Vec<Panel>
    },
    Resume {

    },
    Contact {
        
    }
}

impl Default for TermSubState {
    fn default() -> Self {
        return TermSubState::Load { step: 0, timer: 0f32 }
    }
}

pub struct Cell {
    pub char: char,
    pub foreground_color: &'static Color,
    pub background_color: Option<&'static Color>,
}

