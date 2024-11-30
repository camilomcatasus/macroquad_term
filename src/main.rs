
use macroquad::{miniquad::window::{dpi_scale, screen_size}, prelude::*};
use models::{Cell, FontType, TermSubState, TerminalState};
use opener::open_url;
use projects::{setup_projects, update_project_buffer};
use resume::{setup_resume, update_resume_buffer};
use terminal_templates::{generate_highlight_box, BALLOON_SPINNER, BALLOON_SPINNER_CHARS, LOAD_TEMPLATE};
use ui::UiContext;
use utils::{highlight_cells, overflow_sub, reset_all_highlights};
use std::{cmp::min, default::Default};

mod opener;
mod background_loading;
mod loading_screen;
mod terminal_templates;
mod models;
mod ui;
mod projects;
mod utils;
mod resume;
mod markdown_renderer;

fn window_conf() -> Conf {
    Conf {
        window_title: "CamTerm".to_string(),
        fullscreen: true,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let (mut screen_w,mut screen_h) = screen_size();
    dpi_scale();


    debug!("Miniquad screen size: {:?}", (screen_w, screen_h));
    debug!("Dpi scale: {}", dpi_scale());
    let mut term_render_target = render_target(screen_w as u32, screen_h as u32);
    term_render_target.texture.set_filter(FilterMode::Nearest);

    let mut terminal_state = TerminalState {
        default_font: None,
        projects: Vec::new(),
        font_size: 38f32,
        terminal_width_px: screen_w,
        terminal_height_px: screen_h,
        ..Default::default()
    };

    let mut material_params = MaterialParams::default();

    material_params.uniforms.push(UniformDesc::new("iTime", UniformType::Float1));

    let material = load_material(
        ShaderSource::Glsl {
            vertex: CRT_VERTEX_SHADER,
            fragment: CRT_FRAGMENT_SHADER,
        },
        material_params,
    ).unwrap();

    loading_screen::run_loading_screen(&mut terminal_state, &material).await;
    setup_main_state(&mut terminal_state);

    let mut ui_context = ui::UiContext::default();
    let mut time = 0.1f32;
    let ui_skin = ui::create_ui_skin(terminal_state.default_font.as_ref().unwrap());
    let button_skin = ui::button_ui_skin(terminal_state.default_font.as_ref().unwrap());
    loop {
        let (new_screen_w, new_screen_h) = screen_size();
        if new_screen_w != screen_w || new_screen_h != screen_h {
            screen_h = new_screen_h;
            screen_w = new_screen_w;

            term_render_target = render_target(screen_w as u32, screen_h as u32);
            term_render_target.texture.set_filter(FilterMode::Nearest);
            debug!("New screen size: {:?}", (screen_w, screen_h));
        }

        set_default_camera();
        ui::handle_ui(screen_w, screen_h, &mut ui_context, &ui_skin, &button_skin, &mut terminal_state);
        handle_input(&mut terminal_state, &ui_context).await;
        ui_context.reset();

        set_camera(&Camera2D {
            zoom: vec2(1./(screen_w /2f32), 1./(screen_h / 2f32)),
            target: vec2(0.0, 0.0),
            render_target: Some(term_render_target.clone()),
            ..Default::default()
        });

        clear_background(DARKGRAY);
        draw_terminal_cells(&mut terminal_state, Some(&ui_context));
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

const FONT_RATIO : f32 = 40.0 / 1440.0;

pub fn draw_terminal_cells(terminal_state: &mut TerminalState, ui_context: Option<&UiContext>) {
    let (screen_w, screen_h) = screen_size();

    let mut font = terminal_state.default_font.as_ref();

    let mut previous_font_type = &FontType::Default;
    
    let term_col_count = terminal_state.cell_buffer[0].len() as f32;
    let term_row_count = terminal_state.cell_buffer.len() as f32;

    terminal_state.font_size = screen_h * FONT_RATIO;

    let vertical_padding = (screen_h - term_row_count * terminal_state.font_size) / 2f32;
    let horizontal_padding = (screen_w - term_col_count * terminal_state.font_size / 2f32) / 2f32;

    for (cell_y, cell_line) in terminal_state.cell_buffer.iter().enumerate() {
        for (cell_x, cell) in cell_line.iter().enumerate() {
            if let Some(background_color) = cell.background_color {
                draw_rectangle(horizontal_padding + cell_x as f32 * terminal_state.font_size / 2f32 - screen_w / 2f32, 
                    vertical_padding + (cell_y as f32 - 1f32) * terminal_state.font_size - screen_h / 2f32 + (terminal_state.font_size / 5f32), 
                    terminal_state.font_size / 2f32, 
                    terminal_state.font_size, 
                    *background_color);
            }
        }
    }

    for (cell_y, cell_line) in terminal_state.cell_buffer.iter().enumerate() {
        for (cell_x, cell) in cell_line.iter().enumerate() {

            if &cell.font_type != previous_font_type {

                font = match cell.font_type {
                    FontType::Default => terminal_state.default_font.as_ref(),
                    FontType::ResumeBold => terminal_state.resume_bold_font.as_ref(),
                    FontType::ResumeItalic => terminal_state.resume_italic_font.as_ref(),
                    FontType::ResumeDefault => terminal_state.resume_normal_font.as_ref(),
                    FontType::ResumeItalicBold => terminal_state.resume_italic_bold_font.as_ref(),
                };

                previous_font_type = &cell.font_type;
            }
            
            if let Some(inner_ui_context) = ui_context {
                if inner_ui_context.back_pressed {
                    let test = 0;
                }
            }

            let char_x = horizontal_padding + cell_x as f32 * terminal_state.font_size / 2f32 - screen_w / 2f32;
            let char_y = vertical_padding + cell_y as f32 * terminal_state.font_size - screen_h / 2f32;
            draw_text_ex(&cell.char.to_string(), char_x, char_y, TextParams {
                font,
                font_size: terminal_state.font_size as u16,
                color: *cell.foreground_color,
                ..Default::default()

            });
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

async fn handle_input(terminal_state: &mut TerminalState, ui_context: &UiContext) {
    let down_input = is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) || ui_context.down_pressed || mouse_wheel().1 < 0.0;
    let up_input = is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) || ui_context.up_pressed || mouse_wheel().1 > 0.0;
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
                        setup_projects(terminal_state).await;
                    }
                    1 => {
                        setup_resume(terminal_state).await;
                    }
                    2 => {
                        open_url("mailto:camilomcatasus@gmail.com");
                    }
                    _ => panic!("")

                };
                return;
            }
        }
        TermSubState::Projects { ref mut selected_project_index, ref mut main_focus, ref mut project_about_scroll,..} => {
            if back_pressed {
                setup_main_state(terminal_state);
                return;
            }
            if !*main_focus {
                if up_input {
                    *selected_project_index = overflow_sub(&selected_project_index, project_count);
                }
                if down_input {
                    *selected_project_index = (*selected_project_index + 1) % project_count;
                }

            }
            else {
                if up_input {
                    *project_about_scroll= project_about_scroll.saturating_sub(1);
                }
                if down_input {
                    *project_about_scroll += 1;
                }
            }
            if left_pressed {
                *main_focus = true;
            }
            if right_pressed {
                *main_focus = false;
            }

            update_project_buffer(terminal_state).await;
        }
        TermSubState::Resume(ref mut resume_panel) => {
            if back_pressed {
                setup_main_state(terminal_state);
                return;
            }

            if up_input && resume_panel.index != 0 {
                resume_panel.index = resume_panel.index.saturating_sub(3);
            }
            if down_input && resume_panel.index != resume_panel.fitted_buffer.len() - resume_panel.height - 1 {
                resume_panel.index += 3;
                resume_panel.index = min(resume_panel.index, resume_panel.fitted_buffer.len() - resume_panel.height - 1);
            }

            update_resume_buffer(terminal_state);

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
