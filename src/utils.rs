use macroquad::color::Color;
use macroquad::math::Rect;

use crate::TerminalState;
use crate::models::{Cell, Panel};

pub fn generate_panels_buffer(
    panels: &Vec<Panel>, 
    term_width: usize, 
    term_height: usize) -> Vec<String> 
{
    let mut border_map: Vec<Vec<bool>> = vec![vec![false;term_width]; term_height];
    let mut temp_buffer: Vec<Vec<char>> = vec![vec![' '; term_width]; term_height];
    for panel in panels {
        println!("{:#?}", panel);
        for x in panel.offset_x..(panel.offset_x + panel.width) {
            border_map[panel.offset_y][x] = true;
            border_map[panel.offset_y + panel.height - 1][x] = true;
        }

        for y in panel.offset_y..(panel.offset_y + panel.height) {
            border_map[y][panel.offset_x] = true;
            border_map[y][panel.offset_x + panel.width - 1] = true;
        }

        let fitted_strings = fit_strings_to_size(panel.width - 2, panel.height - 2, panel.index, &panel.text);
        write_strs_to_char_buffer(panel.offset_x + 1, panel.offset_y + 1, &fitted_strings, &mut temp_buffer);
    }


    for y in 0..term_height {
        for x in 0..term_width {
            if border_map[y][x] {
                temp_buffer[y][x] = parse_cell_from_neighbors(
                    y > 0 && border_map[y-1][x],
                    y < term_height - 1 && border_map[y+1][x], 
                    x > 0 && border_map[y][x-1], 
                    x < term_width - 1 && border_map[y][x+1], 
                );
            }
        }
    }

    return temp_buffer.iter().map(|line| line.into_iter().collect()).collect();

}

pub fn generate_cells_from_panels(panels: &Vec<Panel>, term_width: usize, term_height: usize) -> Vec<Vec<Cell>> {
    let mut cell_buffer: Vec<Vec<Cell>> = vec![vec![Cell::default(); term_width]; term_height];
    let mut border_map: Vec<Vec<bool>> = vec![vec![false;term_width]; term_height];

    for panel in panels {
        println!("{:#?}", panel);
        for x in panel.offset_x..(panel.offset_x + panel.width) {
            border_map[panel.offset_y][x] = true;
            border_map[panel.offset_y + panel.height - 1][x] = true;
        }

        for y in panel.offset_y..(panel.offset_y + panel.height) {
            border_map[y][panel.offset_x] = true;
            border_map[y][panel.offset_x + panel.width - 1] = true;
        }

        let fitted_strings = fit_strings_to_size(panel.width - 2, panel.height - 2, panel.index, &panel.text);
        write_strs_to_cell_buffer(panel.offset_x + 1, panel.offset_y + 1, &fitted_strings, &mut cell_buffer);
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

    return cell_buffer;
}

fn write_strings_to_char_buffer(
    offset_x: usize,
    offset_y: usize,
    parsed: &Vec<String>, 
    char_buffer: &mut Vec<Vec<char>>) {
    assert!(parsed.len() > 0 && parsed[0].len() > 0);
    assert!(char_buffer.len() > parsed.len() + offset_y && 
            char_buffer[0].len() > parsed[0].len() + offset_x, 
        "char_buffer size: ({},{}) , parsed size: ({}, {}), offsets: ({}, {})",
        char_buffer[0].len(),
        char_buffer.len(),
        parsed[0].len(),
        parsed.len(),
        offset_x,
        offset_y);


    parsed.iter().enumerate().for_each(|(line_index, line)| {
        line.chars().enumerate().for_each(|(char_index, char_val)| {
            char_buffer[offset_y + line_index][offset_x + char_index] = char_val;
        });
    })
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
