use crate::draw_terminal_cells;
use crate::models::{
    Cell,
    FontType, ProjectInfo
};
use crate::terminal_templates::BALLOON_SPINNER;
use crate::utils::generate_cell_line;
use crate::{models::{TermSubState, TerminalState}, terminal_templates::{BALLOON_SPINNER_CHARS, LOAD_TEMPLATE}};
use macroquad::experimental::coroutines::{start_coroutine, Coroutine};
use macroquad::miniquad::window::screen_size;
use macroquad::prelude::*;

fn setup_loading_state(terminal_state: &mut TerminalState, first_file: &str) -> (usize, usize) {
    terminal_state.sub_state = TermSubState::Load;
    terminal_state.cell_buffer = LOAD_TEMPLATE.iter().map(|line| {
        line.chars().map(|c| {
            Cell {
                char: c,
                foreground_color: &GREEN,
                background_color: None,
                font_type: FontType::Default
            }
        }).collect()

    }).collect();
    let rect_length = terminal_state.cell_buffer[0].len();
    let loading_str_len = 9;
    let padding = " ".repeat(( rect_length - loading_str_len )/ 2);
    let loading_str : Vec<Cell> = generate_cell_line(&format!("{}{} Loading", padding, BALLOON_SPINNER[0]));
    let first_file_padding = " ".repeat(( rect_length - first_file.len() ) / 2);
    terminal_state.cell_buffer.push(loading_str);

    let first_file_str = generate_cell_line(&format!("{}{}", first_file_padding, first_file));
    terminal_state.cell_buffer.push(first_file_str);

    let cell_x = ( rect_length - loading_str_len )/ 2;
    let cell_y = terminal_state.cell_buffer.len() - 2;
    (cell_x, cell_y)
}

pub fn start_file_coroutine(file_path: &str) -> Coroutine<Result<Vec<u8>, macroquad::Error>> {
    let file_path = file_path.to_string();
    return start_coroutine(async move {
        return macroquad::file::load_file(&file_path).await;
    });
}

pub async fn run_loading_screen(terminal_state: &mut TerminalState, material: &Material) {
    const FILES_TO_LOAD : [&'static str; 6]= [
        "fonts/TerminalFont.ttf", 
        "fonts/UbuntuMonoNerdFontMono-Bold.ttf", 
        "fonts/UbuntuMonoNerdFontMono-BoldItalic.ttf",
        "fonts/UbuntuMonoNerdFontMono-Italic.ttf",
        "fonts/UbuntuMonoNerdFontMono-Regular.ttf",
        "projects.json"
    ];
    let animated_cell_pos = setup_loading_state(terminal_state, FILES_TO_LOAD[0]);
    const ANIM_TIME_STEP : f32 = 0.1f32;
    let mut current_frame_time = 0f32;

    let mut file_loading_index = 0;
    let mut animation_step = 0;
    let mut time = 0f32;

    let mut downloading_coroutine = start_file_coroutine(FILES_TO_LOAD[file_loading_index]);
    let (mut screen_w, mut screen_h) = screen_size();
    let mut term_render_target = render_target(screen_w as u32, screen_h as u32);
    term_render_target.texture.set_filter(FilterMode::Nearest);

    while file_loading_index < FILES_TO_LOAD.len() {
        let (new_screen_w, new_screen_h) = screen_size();
        if new_screen_w != screen_w || new_screen_h != screen_h {
            screen_h = new_screen_h;
            screen_w = new_screen_w;

            term_render_target = render_target(screen_w as u32, screen_h as u32);
            term_render_target.texture.set_filter(FilterMode::Nearest);
        }

        set_camera(&Camera2D {
            zoom: vec2(1./(screen_w /2f32), 1./(screen_h / 2f32)),
            target: vec2(0.0, 0.0),
            render_target: Some(term_render_target.clone()),
            ..Default::default()
        });
        
        current_frame_time += get_frame_time();
        if current_frame_time >= ANIM_TIME_STEP {
            current_frame_time = 0f32;
            animation_step =  (animation_step + 1) % BALLOON_SPINNER_CHARS.len();
            //TODO: Animate
            //
            terminal_state.cell_buffer[animated_cell_pos.1][animated_cell_pos.0].char = BALLOON_SPINNER_CHARS[animation_step];

            if animation_step == 0 {
                if downloading_coroutine.is_done() {
                    debug!("font {} loaded!", FILES_TO_LOAD[file_loading_index]);
                    let bytes = downloading_coroutine.retrieve().expect("Future not done").expect("Issue downloading file");
                    match file_loading_index {
                        0 => {
                            let new_font = load_ttf_font_from_bytes(&bytes).expect("");
                            terminal_state.default_font = Some(new_font);
                        },
                        1 => {
                            let new_font = load_ttf_font_from_bytes(&bytes).expect("");
                            terminal_state.resume_bold_font = Some(new_font);
                        }
                        2 => {
                            let new_font = load_ttf_font_from_bytes(&bytes).expect("");
                            terminal_state.resume_italic_bold_font = Some(new_font);
                        }
                        3 => {
                            let new_font = load_ttf_font_from_bytes(&bytes).expect("");
                            terminal_state.resume_italic_font = Some(new_font);
                        }
                        4 => {
                            if let Ok(new_font) = load_ttf_font_from_bytes(&bytes) {
                                terminal_state.resume_normal_font = Some(new_font);
                            }
                            else {
                                error!("Could not load font: {:?}", bytes);
                            }
                        }
                        5 => {
                            let project_data: Vec<ProjectInfo> = serde_json::from_slice(&bytes).expect("Could not decode json");
                            terminal_state.projects = project_data;
                        }
                        _ => ()//panic!("Font loading index not supported")
                    }
                    file_loading_index += 1;

                    if file_loading_index < FILES_TO_LOAD.len() {
                        downloading_coroutine = start_file_coroutine(FILES_TO_LOAD[file_loading_index]);
                        terminal_state.cell_buffer.pop();
                        let buffer_width = terminal_state.cell_buffer[0].len();
                        let padding = " ".repeat((buffer_width - FILES_TO_LOAD[file_loading_index].len()) / 2);
                        let new_file_line = generate_cell_line(&format!("{}{}", padding, FILES_TO_LOAD[file_loading_index]));
                        terminal_state.cell_buffer.push(new_file_line);
                    }
                }
            }
        }

        clear_background(DARKGRAY);
        draw_terminal_cells(terminal_state, None);
        set_default_camera();
        gl_use_material(&material);
        material.set_uniform("iTime", time);
        time += 0.02;

        draw_texture_ex(
            &term_render_target.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        gl_use_default_material();
        next_frame().await;
    }

}
