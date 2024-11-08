use macroquad::{color::WHITE, input::utils, math::Rect};

use crate::{models::{Panel, ProjectInfo, TermSubState, TerminalState}, terminal_templates::generate_highlight_box, utils::{generate_cells_from_panels, highlight_cells}};

const PROJECT_SIDE_WIDTH: usize = 21;
const PROJECT_ART_HEIGHT: usize = 8;
const TERM_HEIGHT: usize = 20;
const TERM_WIDTH: usize = 80;

pub const ABOUT_PANEL_INDEX: usize = 0;
pub const ART_PANEL_INDEX: usize = 1;
pub const PROJECTS_PANEL_INDEX: usize = 2;

pub fn setup_projects(terminal_state: &mut TerminalState) {

    let mut projects_str: Vec<String> = terminal_state.projects.iter().map(|project: &ProjectInfo| project.name.clone()).collect();
    projects_str[0] = format!("> {}", &projects_str[0]);

    let project = &terminal_state.projects[0];
    let project_panels = vec![
        // About panel
        Panel {
            text: project.about.clone(),
            index: 0,
            width: TERM_WIDTH - PROJECT_SIDE_WIDTH + 1,
            height: TERM_HEIGHT,
            offset_y: 0,
            offset_x: 0
        },
        Panel {
            text: project.ascii_art.clone(),
            index: 0,
            width: PROJECT_SIDE_WIDTH,
            height: PROJECT_ART_HEIGHT + 1,
            offset_x: TERM_WIDTH - PROJECT_SIDE_WIDTH,
            offset_y: 0
        },
        Panel {
            text: projects_str,
            index: 0,
            width: PROJECT_SIDE_WIDTH,
            height: TERM_HEIGHT - PROJECT_ART_HEIGHT,
            offset_y: PROJECT_ART_HEIGHT,
            offset_x: TERM_WIDTH - PROJECT_SIDE_WIDTH
        }
    ];
    
    //let first_project_str = &projects_str[0].clone();

    let highlighted_project = Rect {
        x: (project_panels[PROJECTS_PANEL_INDEX].offset_x ) as f32,
        y: (project_panels[PROJECTS_PANEL_INDEX].offset_y + 1) as f32,
        w: project_panels[PROJECTS_PANEL_INDEX].text[0].len() as f32,
        h: 1f32
    };

    highlight_cells(&highlighted_project, terminal_state, &WHITE);
    terminal_state.cell_buffer = generate_cells_from_panels(&project_panels, TERM_WIDTH, TERM_HEIGHT);
    terminal_state.sub_state = TermSubState::Projects { selected_project_index: 0, 
        project_about_scroll: 0, 
        main_focus: true, 
        panels: project_panels 
    };
}

pub fn update_project_buffer( 
    terminal_state: &mut TerminalState, 
) {
    //print_bool_map(&buffer_map);

    //for project


    let mut highlighted_project: Option<Rect> = None;

    if let TermSubState::Projects { selected_project_index, project_about_scroll, ref main_focus , ref mut panels} = terminal_state.sub_state {
        

        //let fitted_about_strings = fit_strings_to_size(TERM_WIDTH - PROJECT_SIDE_WIDTH - 3, TERM_HEIGHT - 2, project_about_scroll, &project_data[selected_project_index].about);
        //write_strs_to_char_buffer(1, 1, &fitted_about_strings, &mut written_buffer);

        //write_strings_to_char_buffer(TERM_WIDTH - PROJECT_SIDE_WIDTH - 1, 1, &project_data[selected_project_index].ascii_art, &mut written_buffer);

        //println!("{:#?}", new_buffer);
        panels[PROJECTS_PANEL_INDEX].text.iter_mut().for_each(|line| {
            if &line[0..2] == "> " {
                *line = line[2..].to_string();
            }

        });

        if *main_focus {
            panels[PROJECTS_PANEL_INDEX].text[selected_project_index] = format!("> {}", panels[PROJECTS_PANEL_INDEX].text[selected_project_index]);
        }

        {

        highlighted_project = Some(Rect {
            x: (panels[PROJECTS_PANEL_INDEX].offset_x + 1) as f32,
            y: (panels[PROJECTS_PANEL_INDEX].offset_y + selected_project_index) as f32,
            w: (panels[PROJECTS_PANEL_INDEX].text[selected_project_index].len()) as f32,
            h: 1f32,
        });

        }
        terminal_state.cell_buffer = generate_cells_from_panels(panels, TERM_WIDTH, TERM_HEIGHT);
    }
    else {
        panic!("Update project buffer should only be called if sub_state is project");
    }

    if let Some(rect) = highlighted_project {
        highlight_cells(&rect, terminal_state, &WHITE);

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

fn print_bool_map(buf: &Vec<Vec<bool>>) {
    for line in buf {
        for c in line {
            match c {
                true => print!("1"),
                false => print!("0")
            }
        }
        println!("");
    }
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
        (false, true, false, true) => '┏',
        (false, true, true, false) => '┓',
        (true, false, false, true) => '┗',
        (true, false, true, false) => '┛',
        (u, d, l, r) => panic!("up: {}, down: {}, left: {}, right: {}", u, d, l , r),

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_parsed() {
        let mut char_buffer = vec![vec![' '; 8]; 8];
        let parsed = vec![
            String::from("_ _"),
            String::from("0^0")
        ];

        write_strings_to_char_buffer(2, 2, &parsed, &mut char_buffer);


        char_buffer.iter().for_each(|char_row| {
            let output_str: String = char_row.into_iter().collect();
            println!("{}", output_str);
        });


        assert!(char_buffer.eq(&vec![
            vec![' '; 8],
            vec![' '; 8],
            vec![' ', ' ', '_', ' ', '_', ' ', ' ', ' '],
            vec![' ', ' ', '0', '^', '0', ' ', ' ', ' '],
            vec![' '; 8],
            vec![' '; 8],
            vec![' '; 8],
            vec![' '; 8],
        ]));

    }
}
