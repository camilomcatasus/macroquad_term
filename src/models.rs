use std::collections::HashMap;

use serde::Deserialize;
use macroquad::prelude::*;

#[derive(Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub ascii_art: Vec<String>,
    pub markdown: String,
    pub url: String,
}

#[derive(Debug, Default)]
pub struct CellPanel {
    pub cells: Vec<Vec<Cell>>,
    pub fitted_buffer: Vec<Vec<Cell>>,
    pub box_color: Option<&'static Color>,
    pub index: usize,
    pub width: usize,
    pub height: usize,
    pub offset_x: usize,
    pub offset_y: usize,
}

impl CellPanel {
    pub fn from_strings(value: &Vec<String>, width: usize, height: usize, offset_x: usize, offset_y: usize) -> Self {
        let cells: Vec<Vec<Cell>> = value.iter().map(|line| {
            let cell_line: Vec<Cell> = line.chars().map(|c| {
                Cell {
                    char: c,
                    background_color: None,
                    foreground_color: &GREEN,
                    font_type: FontType::Default
                }
            }).collect();

            cell_line
        }).collect();

        let mut cell_panel = CellPanel {
            cells,
            width,
            height,
            offset_y,
            offset_x,
            ..Default::default()
        };

        cell_panel.fit_buffer();
        return cell_panel;
    }

    pub fn update_from_strings(&mut self, strings: &Vec<String>) {
        let cells: Vec<Vec<Cell>> = strings.iter().map(|line| {
            let cell_line: Vec<Cell> = line.chars().map(|c| {
                Cell {
                    char: c,
                    background_color: None,
                    foreground_color: &GREEN,
                    font_type: FontType::Default
                }
            }).collect();

            cell_line
        }).collect();

        self.cells = cells;
        self.fit_buffer();
    }
}

impl CellPanel {
    pub fn fit_buffer(&mut self) {
        self.fitted_buffer = Vec::new();

        let mut col = 0;
        let mut current_cell_line : Vec<Cell> = vec![Cell::default(); self.width];
        for cell_line in self.cells.iter() {

            for cell in cell_line.iter() {
                if col >= self.width {
                    self.fitted_buffer.push(current_cell_line);
                    current_cell_line = vec![Cell::default(); self.width];
                    col = 0;
                }
                current_cell_line[col] = cell.clone();
                col += 1;
            }

            self.fitted_buffer.push(current_cell_line);
            current_cell_line = vec![Cell::default(); self.width];
            col = 0;
        }

        let current_height = self.fitted_buffer.len();
        for _ in current_height..self.height  {
            self.fitted_buffer.push(vec![Cell::default(); self.width]);
        }
    }
    pub fn write_to_buffer(&self, char_buffer: &mut Vec<Vec<Cell>>) {
        let mut row = self.offset_y;

        for cell_line in self.fitted_buffer[usize::min(self.index, self.fitted_buffer.len() - 1)..(usize::min(self.index + self.height, self.fitted_buffer.len()))].iter() {
            let mut col = self.offset_x;
            for cell in cell_line.iter() {
                char_buffer[row][col] = cell.clone();
                col += 1;
            }
            row += 1;
        }
    }
}
#[derive(Default)]
pub struct TerminalState {
    pub cell_buffer: Vec<Vec<Cell>>,
    pub line_buffer: Vec<String>,
    pub highlighted_boxes: Vec<Rect>,
    pub projects: Vec<ProjectInfo>,
    pub loaded_projects: HashMap<String, String>,
    pub line_index: usize,
    pub cursor_x: f32,
    pub cursor_y: f32,
    pub font_size: f32,
    pub terminal_width_px: f32,
    pub terminal_height_px: f32,
    pub sub_state: TermSubState,
    pub default_font: Option<Font>,
    pub resume_normal_font: Option<Font>,
    pub resume_bold_font: Option<Font>,
    pub resume_italic_font: Option<Font>,
    pub resume_italic_bold_font: Option<Font>,
}

pub enum TermSubState {
    Load,
    Main {
        index: usize
    },
    Projects {
        selected_project_index: usize,
        project_about_scroll: usize,
        main_focus: bool,
        cell_panels: Vec<CellPanel>,
    },
    Resume (CellPanel),
    Contact {
        
    }
}

impl Default for TermSubState {
    fn default() -> Self {
        return TermSubState::Load;
    }
}


#[derive(Clone, Debug)]
pub struct Cell {
    pub char: char,
    pub foreground_color: &'static Color,
    pub background_color: Option<&'static Color>,
    pub font_type: FontType
}


impl Default for Cell {
    fn default() -> Self {
        Cell {
            char: ' ',
            foreground_color: &GREEN,
            background_color: None,
            font_type: FontType::Default
        }
    }
}

impl Cell {
    pub fn new(c: char) -> Self {
        Self {
            char: c,
            foreground_color: &GREEN,
            background_color: None,
            font_type: FontType::Default
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum FontType {
    Default,
    ResumeDefault,
    ResumeBold,
    ResumeItalic,
    ResumeItalicBold,
}
