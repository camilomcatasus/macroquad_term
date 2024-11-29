
use macroquad::prelude::*;
use crate::TerminalState;
use crate::models::{Cell, CellPanel};


pub fn write_cell_panels_with_border(
    panels: &[CellPanel], 
    term_width: usize, 
    term_height: usize) -> Vec<Vec<Cell>> {

    let mut cell_buffer: Vec<Vec<Cell>> = vec![vec![Cell::default(); term_width]; term_height];
    let mut border_map: Vec<Vec<bool>> = vec![vec![false;term_width]; term_height];

    for panel in panels {
        assert!(panel.offset_y > 0 
            && panel.offset_y + panel.height < term_height
            && panel.offset_x > 0
            && panel.offset_x + panel.width < term_width, 
        "Panel size: {:?} offset: {:?} does not fit inside terminal with border", (panel.width, panel.height), (panel.offset_x, panel.offset_y));

        for x in (panel.offset_x - 1)..(panel.offset_x + panel.width + 1) {

            border_map[panel.offset_y - 1][x] = true;
            border_map[panel.offset_y + panel.height][x] = true;
            if cell_buffer[panel.offset_y - 1][x].foreground_color == &GREEN {
                cell_buffer[panel.offset_y - 1][x].foreground_color = panel.box_color.unwrap_or(&GREEN);
            }
            if cell_buffer[panel.offset_y + panel.height][x].foreground_color == &GREEN {
                cell_buffer[panel.offset_y + panel.height][x].foreground_color = panel.box_color.unwrap_or(&GREEN);
            }
        }

        for y in (panel.offset_y - 1)..(panel.offset_y + panel.height + 1) {
            border_map[y][panel.offset_x - 1] = true;
            border_map[y][panel.offset_x + panel.width] = true;

            if cell_buffer[y][panel.offset_x - 1].foreground_color == &GREEN {
                cell_buffer[y][panel.offset_x - 1].foreground_color = panel.box_color.unwrap_or(&GREEN);
            }
            if cell_buffer[y][panel.offset_x + panel.width].foreground_color == &GREEN {
                cell_buffer[y][panel.offset_x + panel.width].foreground_color = panel.box_color.unwrap_or(&GREEN);
            }
        }

        panel.write_to_buffer(&mut cell_buffer);
    }

    for y in 0..term_height {
        for x in 0..term_width {
            if border_map[y][x] {
                cell_buffer[y][x].char = parse_cell_from_neighbors(
                    y > 0 && border_map[y-1][x],
                    y < term_height - 1 && border_map[y+1][x], 
                    x > 0 && border_map[y][x-1], 
                    x < term_width - 1 && border_map[y][x+1], 
                );
            }
        }
    }

    cell_buffer
}

pub fn generate_cell_line(string: &str) -> Vec<Cell> {
    string.chars().map(|c| {
        Cell {
            char: c,
            background_color: None,
            foreground_color: &GREEN,
            font_type: crate::models::FontType::Default
            
        }
    }).collect()
}

pub fn highlight_cells(rect: &Rect, terminal_state: &mut TerminalState, background_color: &'static Color) {
    let x_start = rect.x as usize;
    let x_end = x_start + rect.w as usize;
    let y_start = rect.y as usize;
    let y_end = y_start + rect.h as usize;

    for x in x_start..x_end {
        for y in y_start..y_end {
            terminal_state.cell_buffer[y][x].background_color = Some(background_color);
        }
    }
} 

pub fn reset_all_highlights(terminal_state: &mut TerminalState) {
    terminal_state.cell_buffer.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|cell| {
            cell.background_color = None;
        });
    });
}

pub fn parse_cell_from_neighbors(up: bool,
    down:bool, left: bool, right: bool) -> char {
    match (up, down, left, right) {
        (true, true, false, false) => '┃',
        (false, false, true, true) => '━',
        (true, true, false, true) => '┣',
        (true, true, true, false) => '┫',
        (true, false, true, true) => '┻',
        (false, true, true, true) => '┳',
        (true, true, true, true) => '╋',
        (false, true, false, true) => '┏',
        (false, true, true, false) => '┓',
        (true, false, false, true) => '┗',
        (true, false, true, false) => '┛',
        (u, d, l, r) => panic!("up: {}, down: {}, left: {}, right: {}", u, d, l , r),

    }
}

fn fit_strings_to_size<'a>(
    width: usize, 
    height: usize, 
    line_index: usize, 
    strings: &'a Vec<String>) -> Vec<&'a str> {

    let test : Vec<&str> =strings[line_index..]
        .iter()
        .flat_map(|line| line.split("\n"))
        .flat_map(|line| {

            let mut temp : Vec<&str> = Vec::new();
            if line.len() > width {
                let split_line = line.split_at(width);
                temp.push(split_line.0);
                temp.push(split_line.1);
            } else {
                temp.push(line);
            }
            temp

        })
        .take(height)
        .collect();


    test
}

fn write_strs_to_cell_buffer(
    offset_x: usize,
    offset_y: usize,
    parsed: &Vec<&str>,
    char_buffer: &mut Vec<Vec<Cell>>) {
    assert!(parsed.len() > 0 && parsed[0].len() > 0);
    assert!(char_buffer.len() > parsed.len() + offset_y && 
            char_buffer[0].len() > parsed[0].len() + offset_x);


    parsed.iter().enumerate().for_each(|(line_index, line)| {
        line.chars().enumerate().for_each(|(char_index, char_val)| {
            char_buffer[offset_y + line_index][offset_x + char_index].char = char_val;
        });
    })
}

fn write_strs_to_char_buffer(
    offset_x: usize,
    offset_y: usize,
    parsed: &Vec<&str>,
    char_buffer: &mut Vec<Vec<char>>) {
    assert!(parsed.len() > 0 && parsed[0].len() > 0);
    assert!(char_buffer.len() > parsed.len() + offset_y && 
            char_buffer[0].len() > parsed[0].len() + offset_x);


    parsed.iter().enumerate().for_each(|(line_index, line)| {
        line.chars().enumerate().for_each(|(char_index, char_val)| {
            char_buffer[offset_y + line_index][offset_x + char_index] = char_val;
        });
    })
}

pub fn overflow_sub(num:&usize, range: usize) -> usize {
    if *num == 0 {
        range - 1
    }
    else {
        *num - 1
    }
}

pub fn find_substr(cell_line: &Vec<crate::models::Cell>, substr: &str) -> Option<usize> {

    for cell_index in 0..(cell_line.len() - substr.len()) {
        let mut found = true;
        for (char_index, char) in substr.chars().enumerate() {
            found = found && cell_line[cell_index + char_index].char == char;
        }

        if found {
            return Some(cell_index);
        }
    }
    None
}

pub fn print_cells(cell_buffer: &Vec<Vec<Cell>>) {
    cell_buffer.iter().for_each(|line| {
        line.iter().for_each(|cell| {
            print!("{}", cell.char);
        });
        print!("\n");
    })
}
