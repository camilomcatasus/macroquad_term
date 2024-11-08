use macroquad::{miniquad::window::screen_size, prelude::*};
use models::{Cell, FontType, Panel, TermSubState, TerminalState};
use projects::{setup_projects, update_project_buffer};
use terminal_templates::{generate_highlight_box, BALLOON_SPINNER, BALLOON_SPINNER_CHARS, LOAD_TEMPLATE, SAND_SPINNER};
use ui::UiContext;
use utils::{highlight_cells, overflow_sub, reset_all_highlights};


use std::default::Default;
mod terminal_templates;
mod models;
mod ui;
mod projects;
mod utils;
mod resume;

#[macroquad::main("TerminalSite")]
async fn main() {


    let projects_file = load_file("projects.json").await.expect("could not load projects.json");
    let projects_string = String::from_utf8(projects_file).expect("Projects were not in utf8");

    let project_data: Vec<models::ProjectInfo> = serde_json::from_str(&projects_string).expect("Could not deserialize project info");

    let (mut screen_w,mut screen_h) = screen_size();
    let mut term_render_target = render_target(screen_w as u32, screen_h as u32);
    term_render_target.texture.set_filter(FilterMode::Nearest);

    let default_font = load_ttf_font("TerminalFont.ttf").await.unwrap();

    let mut terminal_state = TerminalState::default();
    terminal_state.font_size = 18.;
    terminal_state.terminal_width_px = screen_w;
    terminal_state.terminal_height_px = screen_h;
    terminal_state.default_font = Some(default_font.clone());
    terminal_state.projects = project_data;
    setup_loading_state(&mut terminal_state);

    let mut material_params = MaterialParams::default();

    let mut time = 0.1f32;
    material_params.uniforms.push(UniformDesc::new("iTime", UniformType::Float1));

    let material = load_material(
        ShaderSource::Glsl {
            vertex: CRT_VERTEX_SHADER,
            fragment: CRT_FRAGMENT_SHADER,
        },
        material_params,
    ).unwrap();

    const LOADING_STEP_TIME: f32 = 0.08;
    
    let mut ui_context = ui::UiContext::default();
    let ui_skin = ui::create_ui_skin(&default_font);
    loop {
        let (new_screen_w, new_screen_h) = screen_size();
        if new_screen_w != screen_w || new_screen_h != screen_h {
            screen_h = new_screen_h;
            screen_w = new_screen_w;

            term_render_target = render_target(screen_w as u32, screen_h as u32);
            term_render_target.texture.set_filter(FilterMode::Nearest);


        }

        set_default_camera();
        ui::handle_ui(screen_w, screen_h, &mut ui_context, &ui_skin);
        handle_input(&mut terminal_state, &ui_context);
        ui_context.reset();

        set_camera(&Camera2D {
            zoom: vec2(1./(screen_w /2f32), 1./(screen_h / 2f32)),
            target: vec2(0.0, 0.0),
            render_target: Some(term_render_target.clone()),
            ..Default::default()
        });

        match &terminal_state.sub_state {
            TermSubState::Load { step, mut timer } => {
                timer += get_frame_time();
                if timer > LOADING_STEP_TIME {
                    timer = 0f32;
                    terminal_state.sub_state = TermSubState::Load { step: *step, timer };
                    step_loading_state(&mut terminal_state);
                }
                else {
                    terminal_state.sub_state = TermSubState::Load { step: *step, timer };
                }
            }
            TermSubState::Projects { selected_project_index, project_about_scroll, main_focus, panels } => {

                //projects::update_project_buffer(&mut terminal_state, &project_data);
            }
            _ => (),
        }

        clear_background(DARKGRAY);
        draw_terminal_cells(&terminal_state);

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
        next_frame().await
    }
}

pub fn draw_terminal_cells(terminal_state: &TerminalState) {
    let (screen_w, screen_h) = screen_size();

    let mut font = match &terminal_state.default_font {
        Some(val) => val,
        None => panic!("Could not load font correctly")
    };

    let mut previous_font_type = FontType::Default;
    
    let term_col_count = terminal_state.cell_buffer[0].len() as f32;
    let term_row_count = terminal_state.cell_buffer.len() as f32;

    let vertical_padding = (screen_h - term_row_count * terminal_state.font_size) / 2f32;
    let horizontal_padding = (screen_w - term_col_count * terminal_state.font_size / 2f32) / 2f32;

    for (cell_y, cell_line) in terminal_state.cell_buffer.iter().enumerate() {
        for (cell_x, cell) in cell_line.iter().enumerate() {

            if cell.font_type != previous_font_type {

                font = Some(match cell.font_type {
                    FontType::Default => terminal_state.default_font,
                });
            }


            if let Some(background_color) = cell.background_color {
                draw_rectangle(horizontal_padding + cell_x as f32 * terminal_state.font_size / 2f32 - screen_w / 2f32, 
                    vertical_padding + cell_y as f32 * terminal_state.font_size - screen_h / 2f32 + (terminal_state.font_size / 5f32), 
                    terminal_state.font_size / 2f32, 
                    terminal_state.font_size, 
                    *background_color);
            }

            let char_x = horizontal_padding + cell_x as f32 * terminal_state.font_size / 2f32 - screen_w / 2f32;
            let char_y = vertical_padding + cell_y as f32 * terminal_state.font_size - screen_h / 2f32;
            draw_text_ex(&cell.char.to_string(), char_x, char_y, TextParams {
                font: Some(font),
                font_size: terminal_state.font_size as u16,
                color: *cell.foreground_color,
                ..Default::default()

            });
        }
    }

}

fn setup_loading_state(terminal_state: &mut TerminalState) {
    terminal_state.sub_state = TermSubState::Load { step: 0 , timer: 0f32};
    terminal_state.cell_buffer = LOAD_TEMPLATE.iter().map(|line| {
        line.chars().map(|c| {
            models::Cell {
                char: c,
                foreground_color: &GREEN,
                background_color: None,
                font_type: FontType::Default
            }
        }).collect()

    }).collect();
    let rect_length = terminal_state.cell_buffer[0].len();
    let padding = " ".repeat(( rect_length - 9 )/ 2);
    let loading_str : Vec<Cell> = format!("{}{} Loading", padding, BALLOON_SPINNER[0]).chars().map(|c| {
        Cell {
            char: c,
            background_color: None,
            foreground_color: &GREEN,
            font_type: FontType::Default
        }

    }).collect();
    terminal_state.cell_buffer.push(loading_str);
}

fn step_loading_state(terminal_state: &mut TerminalState) {
    if let TermSubState::Load { ref mut step, timer: _} = terminal_state.sub_state {

        let prev_spinner_char = BALLOON_SPINNER_CHARS[*step % BALLOON_SPINNER_CHARS.len()];
        if *step >= BALLOON_SPINNER_CHARS.len() * 2 {
            setup_main_state(terminal_state);
            return;
        } else {
            *step = *step + 1;
        }

        let new_spinner_char = BALLOON_SPINNER_CHARS[*step % BALLOON_SPINNER_CHARS.len()];

        let cell_opt : Option<&mut Cell> = terminal_state.cell_buffer.last_mut().unwrap().iter_mut().find(|cell| {
            cell.char == prev_spinner_char
        });

        if let Some(cell) = cell_opt {
            cell.char = new_spinner_char;
        }
    }
}

pub fn setup_main_state(terminal_state: &mut TerminalState) {

    terminal_state.sub_state = TermSubState::Main { index: 0 };

    terminal_state.font_size = 48.;
    terminal_state.cell_buffer = terminal_templates::MAIN_TEMPLATE.iter().map(|line| {
        line.chars().map(|c| {
            Cell {
                char: c,
                background_color: None,
                foreground_color: &GREEN,
                font_type: FontType::Default
            }
        }).collect()
    }).collect();
    terminal_state.line_buffer = terminal_templates::MAIN_TEMPLATE
        .to_vec().iter().map(|val| val.to_string()).collect();

    let high_light_box = generate_highlight_box(1).expect("Problem generating highlight box from main template");
    highlight_cells(&high_light_box, terminal_state, &WHITE)
    //terminal_state.highlighted_boxes = vec![generate_highlight_box(1).expect("Should return a box")];

}

fn handle_input(terminal_state: &mut TerminalState, ui_context: &UiContext) {
    let down_input = is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) || ui_context.down_pressed;
    let up_input = is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) || ui_context.up_pressed;
    let left_pressed = is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) || ui_context.left_pressed;
    let right_pressed = is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) || ui_context.right_pressed;
    let enter_pressed = is_key_pressed(KeyCode::Enter) || ui_context.enter_pressed;
    let back_pressed = is_key_pressed(KeyCode::Backspace);

    //Check to see if we need to handle input
    if !(down_input || up_input || left_pressed || right_pressed || enter_pressed || back_pressed) {
        return;
    }

    let project_count = terminal_state.projects.len();

    let mut new_main_index : Option<usize> = None;


    match &mut terminal_state.sub_state {
        TermSubState::Main { ref mut index } => {
            let mut index_changed = false;
            if down_input {
                index_changed = true;
                *index = (*index + 1) % 3;
            }
            else if up_input {
                index_changed = true;
                if *index == 0 {
                    *index = 2;
                }
                else {
                    *index -= 1;
                }
            }
            if index_changed {
                new_main_index = Some(*index);
            }
            else if enter_pressed {
                match index {
                    0 => {
                        setup_projects(terminal_state);
                    }
                    1 => {
                        //TermSubState::Resume {  }
                    }
                    2 => {
                        //TermSubState::Contact {  }
                    }
                    _ => panic!("")

                };
                return;
            }
        }
        TermSubState::Projects { ref mut selected_project_index, project_about_scroll, main_focus, ref mut panels } => {
            if back_pressed {
                setup_main_state(terminal_state);
                return;
            }
            if *main_focus {
                if up_input {
                    *selected_project_index = overflow_sub(&selected_project_index, project_count);
                }
                if down_input {
                    *selected_project_index = (*selected_project_index + 1) % project_count;
                }
            }

            panels[projects::ABOUT_PANEL_INDEX].text = terminal_state.projects[*selected_project_index].about.clone();
            panels[projects::ART_PANEL_INDEX].text = terminal_state.projects[*selected_project_index].ascii_art.clone();

            update_project_buffer(terminal_state);
        }

        _ => ()

    }

    if let Some(index) = new_main_index {
        reset_all_highlights(terminal_state);
        let highlight_box = generate_highlight_box(index + 1).expect("Couldn't create highlight box");
        highlight_cells(&highlight_box, terminal_state, &WHITE);
    }
}


const CRT_FRAGMENT_SHADER: &'static str = r#"#version 100
precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;
uniform float iTime;

// https://www.shadertoy.com/view/XtlSD7

vec2 CRTCurveUV(vec2 uv)
{
    uv = uv * 2.0 - 1.0;
    vec2 offset = abs( uv.yx ) / vec2( 6.0, 4.0 );
    uv = uv + uv * offset * offset;
    uv = uv * 0.5 + 0.5;
    return uv;
}

void DrawVignette( inout vec3 color, vec2 uv )
{
    float vignette = uv.x * uv.y * ( 1.0 - uv.x ) * ( 1.0 - uv.y );
    vignette = clamp( pow( 16.0 * vignette, 0.3 ), 0.0, 1.0 );
    color *= vignette;
}


void DrawScanline( inout vec3 color, vec2 uv )
{
    float scanline 	= clamp( 0.95 + 0.05 * cos( 3.14 * ( uv.y + 0.008 * iTime ) * 240.0 * 1.0 ), 0.0, 1.0 );
    float grille 	= 0.85 + 0.15 * clamp( 1.5 * cos( 3.14 * uv.x * 640.0 * 1.0 ), 0.0, 1.0 );
    color *= scanline * grille * 1.2;
}

void main() {
    vec2 crtUV = CRTCurveUV(uv);
    vec3 res = texture2D(Texture, uv).rgb * color.rgb;
    if (crtUV.x < 0.0 || crtUV.x > 1.0 || crtUV.y < 0.0 || crtUV.y > 1.0)
    {
        res = vec3(0.0, 0.0, 0.0);
    }
    DrawVignette(res, crtUV);
    DrawScanline(res, uv);
    gl_FragColor = vec4(res, 1.0);

}
"#;

const CRT_VERTEX_SHADER: &'static str = "#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;

varying lowp vec2 uv;
varying lowp vec4 color;

uniform mat4 Model;
uniform mat4 Projection;

void main() {
    gl_Position = Projection * Model * vec4(position, 1);
    color = color0 / 255.0;
    uv = texcoord;
}
";
