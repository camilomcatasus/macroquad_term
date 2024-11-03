use macroquad::math::Rect;

use crate::{models::ProjectInfo, terminal_templates::TermSubState, TerminalState};

const PROJECT_SIDE_WIDTH: usize = 18;
const PROJECT_ART_HEIGHT: usize = 6;
const TERM_HEIGHT: usize = 22;
const TERM_WIDTH: usize = 80;


pub fn update_project_buffer( 
    terminal_state: &mut TerminalState, 
    project_data: Vec<ProjectInfo>) {
    let line = vec![false; TERM_WIDTH];
    let mut buffer_map = vec![line; TERM_HEIGHT];

    for x in 0..TERM_WIDTH {
        for y in 0..TERM_HEIGHT {

            if x == 0 
                || y == 0 
                || x == TERM_WIDTH - 1 
                || y == TERM_WIDTH - 1
                || x == TERM_WIDTH - 1 - PROJECT_SIDE_WIDTH 
                || (y == PROJECT_ART_HEIGHT && x > TERM_WIDTH - PROJECT_SIDE_WIDTH - 1) {

                buffer_map[y][x] = true;
            }
        }
    }

    let written_buffer: Vec<Vec<char>> = vec![vec![' '; TERM_WIDTH]; TERM_HEIGHT];

    let mut new_buffer: Vec<String> = Vec::new();
    for x in 0..TERM_WIDTH {
        let mut new_line = String::new();
        for y in 0..TERM_HEIGHT {

            if buffer_map[y][x] {
                let new_cell = parse_cell_from_neighbors(x > 0 && buffer_map[y][x-1], 
                    y < TERM_HEIGHT - 1 && buffer_map[y+1][x], 
                    x < TERM_WIDTH - 1 && buffer_map[y][x+1], 
                    y > 0 && buffer_map[y-1][x]);

                new_line.push(new_cell);
            }
            else {


            }
        }
        new_buffer.push(new_line);
    }

    if let TermSubState::Projects { selected_project_index, project_about_scroll, main_focus} = terminal_state.sub_state {
        
    }
    else {
        panic!("Update project buffer should only be called if sub_state is project");
    }
}


fn parse_cell_from_neighbors(up: bool,
    down:bool, left: bool, right: bool) -> char {
    match (up, down, left, right) {
        (true, true, false, false) => '┃',
        (false, false, true, true) => '━',
        (true, true, false, true) => '┣',
        (true, true, true, false) => '┫',
        (true, false, true, true) => '┻',
        (false, true, true, true) => '┳',
        (true, true, true, true) => '╋',
        (_, _, _, _) => panic!("Unhandled box cell parsing case"),

    }
}
