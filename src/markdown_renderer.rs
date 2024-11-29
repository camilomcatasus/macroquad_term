use std::default;

use macroquad::prelude::*;
use crate::models::{
    CellPanel,
    Cell
};
use crate::FontType;

#[derive(PartialEq, Eq, Debug)]
enum Token {
    Word(String),
    DoubleAsterisk,
    Asterisk,
    NewLine,
    Space,
    PoundSign,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match &self {
            Token::Space => String::from(" "),
            Token::NewLine => String::from("\n"),
            Token::Asterisk => String::from("*"),
            Token::DoubleAsterisk => String::from("**"),
            Token::PoundSign => String::from("#"),
            Token::Word(str) => str.clone()
        }
    }
}

fn lexer(mark_down_text: &String) -> Vec<Token> {
    let mut tokens : Vec<Token> = Vec::new();

    let mut pointer = 0;

    //debug!("Started lexing text: \n{}", mark_down_text);
    while pointer < mark_down_text.len() - 1 {
        let mut at1 = &mark_down_text[pointer..(pointer+1)];
        let mut at2 = &mark_down_text[(pointer + 1)..(pointer + 2)];

        let new_token = match (at1, at2) {
            ("#", _) => Token::PoundSign,
            ("*", "*") => {pointer += 1; Token::DoubleAsterisk},
            ("*", _) => Token::Asterisk,
            (" ", _) => Token::Space,
            ("\r", "\n") => {pointer += 1; Token::NewLine},
            ("\n", _) =>  Token::NewLine,
            _ => {
                //parse word
                let start = pointer;
                while pointer < mark_down_text.len() - 1 && !(at1 == "#" || at1 == "*" || at1 == " " || at1 == "\n" || at1 =="\r") {
                    at1 = &mark_down_text[pointer..(pointer+1)];
                    at2 = &mark_down_text[(pointer + 1)..(pointer + 2)];

                    if at1 == "\\" && (at2 == "#" || at2 == "*") {
                        pointer += 1;
                    }
                    pointer += 1;
                }
                pointer -= 1;
                let word_text = mark_down_text[start..pointer].to_string();
                pointer -= 1;
                Token::Word(word_text)
                
            }
        };
        pointer += 1;
        tokens.push(new_token);
    }
    //debug!("Finished lexing text");

    tokens
}
pub fn render_markdown(mark_down_text: &String, width: usize, height: usize) -> CellPanel {
    let mark_down_tokens = lexer(mark_down_text);

    //debug!("Tokens: {:?}", mark_down_tokens);

    let mut token_index = 0;
    let mut generator = CellGenerator {
        line_width: width,
        ..Default::default()
    };

    while token_index < mark_down_tokens.len() {
        match &mark_down_tokens[token_index] {
            Token::Asterisk => {
                if generator.italic {
                    generator.italic = false;
                }
                else if has_token(token_index + 1, &mark_down_tokens, Token::Asterisk) {
                    generator.italic = true;
                } else {
                    generator.gen_cell('*')
                }
            }
            Token::DoubleAsterisk => {
                if generator.bold {
                    generator.bold = false;
                }
                else if has_token(token_index + 1, &mark_down_tokens, Token::DoubleAsterisk) {
                        generator.bold = true;
                } else {
                    generator.gen_cell('*');
                    generator.gen_cell('*');
                }

            }
            Token::PoundSign => {
                if (token_index == 0 || mark_down_tokens[token_index - 1] == Token::NewLine) && mark_down_tokens[token_index + 1] == Token::Space {
                    token_index += 1;
                    generator.header = true;

                } else {
                    generator.gen_cell('#');
                }
            }
            Token::Space => {
                generator.gen_cell(' ');
            }
            Token::Word(word) => {
                if word.len() + generator.cell_line.len() >= generator.line_width {
                    generator.cell_buffer.push(generator.cell_line);
                    generator.cell_line = Vec::new();
                }
                word.chars().for_each(|c| {
                    generator.gen_cell(c);
                })
            }
            Token::NewLine => {
                generator.cell_buffer.push(generator.cell_line);
                generator.cell_line = Vec::new();
                generator.header = false;
            }
        }

        token_index += 1;
    }

    //print_cells(&generator.cell_buffer);

    CellPanel {
        cells: generator.cell_buffer,
        fitted_buffer: Vec::new(),
        index: 0,
        width,
        height,
        offset_x: 0,
        offset_y: 0,
        ..Default::default()
    }
}

#[derive(Default)]
struct CellGenerator {
    cell_buffer: Vec<Vec<Cell>>,
    cell_line: Vec<Cell>,
    bold: bool,
    italic: bool,
    header: bool,
    line_width: usize,
}

impl CellGenerator {
    fn gen_cell(&mut self, c: char) {
        let font_type = match (self.bold, self.italic) {
            (true, true) => FontType::ResumeItalicBold,
            (false, false) => FontType::ResumeDefault,
            (true, false) => FontType::ResumeBold,
            (false, true) => FontType::ResumeItalic,
        };

        let foreground_color = match self.header {
            true => &WHITE,
            false => &GREEN
        };

        let background_color = match self.header {
            true => Some(&DARKGREEN),
            false => None,
        };

        self.cell_line.push(Cell {
            char: c,
            font_type,
            foreground_color,
            background_color,
        });

        if self.cell_line.len() >= self.line_width {
            self.cell_buffer.push(self.cell_line.clone());
            self.cell_line = Vec::new();
        }
    }
}

fn has_token(token_index: usize, tokens: &Vec<Token>, target: Token) -> bool { 
    let mut new_index = token_index;
    while new_index < tokens.len() && tokens[new_index] != Token::NewLine {
        if tokens[new_index] == target {
            return true;
        }
        new_index += 1;

    }
    return false;
}
