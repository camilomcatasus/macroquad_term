use macroquad::{miniquad::window::screen_size, prelude::*};
use terminal_templates::{generate_highlight_box, TermSubState, BALLOON_SPINNER, LOAD_TEMPLATE, SAND_SPINNER};
use ui::UiContext;


use std::default::Default;
mod terminal_templates;
mod models;
mod ui;
mod projects;

#[macroquad::main("TerminalSite")]
async fn main() {

    let (mut screen_w,mut screen_h) = screen_size();
    let mut term_render_target = render_target(screen_w as u32, screen_h as u32);
    term_render_target.texture.set_filter(FilterMode::Nearest);

    let font = load_ttf_font("TerminalFont.ttf").await.unwrap();

    let mut terminal_state = TerminalState::default();
    terminal_state.font_size = 18.;
    terminal_state.terminal_width_px = screen_w;
    terminal_state.terminal_height_px = screen_h;
    terminal_state.font = Some(font.clone());
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

    const LOADING_STEP_TIME: f32 = 0.2;
    
    let mut ui_context = ui::UiContext::default();
    let ui_skin = ui::create_ui_skin(&font);
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

        match terminal_state.sub_state {
            TermSubState::Load { step, mut timer } => {
                timer += get_frame_time();
                if timer > LOADING_STEP_TIME {
                    timer = 0f32;
                    terminal_state.sub_state = TermSubState::Load { step, timer };
                    step_loading_state(&mut terminal_state);
                }
                else {
                    terminal_state.sub_state = TermSubState::Load { step, timer };
                }
            }
            _ => (),
        }

        clear_background(DARKGRAY);
        draw_terminal(&terminal_state);

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

#[derive(Default)]
struct TerminalState {
    line_buffer: Vec<String>,
    highlighted_boxes: Vec<Rect>,
    line_index: usize,
    cursor_x: f32,
    cursor_y: f32,
    font_size: f32,
    terminal_width_px: f32,
    terminal_height_px: f32,
    sub_state: TermSubState,
    font: Option<Font>,
}

fn draw_terminal(terminal_state: &TerminalState) {

    let (screen_w, screen_h) = screen_size();
    
    let term_col_count = terminal_state.line_buffer.iter()
        .map(|line| line.chars().count())
        .max().unwrap() as f32;
    let term_row_count = terminal_state.line_buffer.len() as f32;

    let vertical_padding = (screen_h - term_row_count * terminal_state.font_size) / 2f32;
    let horizontal_padding = (screen_w - term_col_count * terminal_state.font_size / 2f32) / 2f32;

    for rect in &terminal_state.highlighted_boxes {
        draw_rectangle(horizontal_padding + rect.x as f32 * terminal_state.font_size / 2f32 - screen_w / 2f32, 
            vertical_padding + rect.y as f32 * terminal_state.font_size - screen_h / 2f32 + (terminal_state.font_size / 5f32), 
            rect.w * terminal_state.font_size / 2f32, 
            rect.h * terminal_state.font_size, WHITE);

    }

    let mut row = 1f32;
    for index in terminal_state.line_index..(terminal_state.line_buffer.len()) {
        let mut col = 1f32;

        for character in terminal_state.line_buffer[index].chars() {
            let char_x = horizontal_padding + col * terminal_state.font_size / 2f32 - screen_w / 2f32;
            let char_y = vertical_padding + row * terminal_state.font_size - screen_h / 2f32;

            if let Some(font) = &terminal_state.font {
                draw_text_ex(&character.to_string(), char_x, char_y, TextParams {
                    font: Some(font),
                    font_size: terminal_state.font_size as u16,
                    color: GREEN,
                    ..Default::default()
                });
            }
            col += 1f32;

        }
        row += 1.;
        
    }


}

fn setup_loading_state(terminal_state: &mut TerminalState) {
    let (screen_x, screen_y) = screen_size();
    terminal_state.sub_state = TermSubState::Load { step: 0 , timer: 0f32};
    terminal_state.line_buffer = LOAD_TEMPLATE.to_vec().iter().map(|val| val.to_string()).collect();
    let rect_length = terminal_state.line_buffer[0].chars().count();
    let padding = " ".repeat(( rect_length - 9 )/ 2);
    let loading_str = format!("{}{} Loading", padding, BALLOON_SPINNER[0]);
    terminal_state.line_buffer.push("".to_string());
    terminal_state.line_buffer.push(loading_str);

}

fn step_loading_state(terminal_state: &mut TerminalState) {
    if let TermSubState::Load { ref mut step, timer: _} = terminal_state.sub_state {

        let prev_spinner_string = BALLOON_SPINNER[*step % BALLOON_SPINNER.len()];
        if *step >= BALLOON_SPINNER.len() * 2 {
            setup_main_state(terminal_state);
            return;
        } else {
            *step = *step + 1;
        }

        let new_spinner_string = BALLOON_SPINNER[*step % BALLOON_SPINNER.len()];
        let new_line = terminal_state.line_buffer.last().expect("").replacen(prev_spinner_string, new_spinner_string, 1);
        let buffer_length = terminal_state.line_buffer.len();
        terminal_state.line_buffer[buffer_length - 1] = new_line;
    }
}

fn setup_main_state(terminal_state: &mut TerminalState) {
    let (screen_x, screen_y) = screen_size();

    terminal_state.sub_state = TermSubState::Main { index: 0 };

    terminal_state.font_size = 48.;
    terminal_state.line_buffer = terminal_templates::MAIN_TEMPLATE
        .to_vec().iter().map(|val| val.to_string()).collect();

    terminal_state.highlighted_boxes = vec![generate_highlight_box(1).expect("Should return a box")];


}

fn handle_input(terminal_state: &mut TerminalState, ui_context: &UiContext) {
    match terminal_state.sub_state {
        TermSubState::Main { ref mut index } => {
            let mut index_changed = false;
            if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) || ui_context.down_pressed {
                index_changed = true;
                *index = (*index + 1) % 3;
            }
            else if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) || ui_context.up_pressed {
                index_changed = true;
                if *index == 0 {
                    *index = 2;
                }
                else {
                    *index -= 1;
                }
            }
            else if is_key_pressed(KeyCode::Enter) || ui_context.enter_pressed {
                terminal_state.sub_state = match index {
                    0 => {
                        TermSubState::Projects { selected_project_index: 0, project_about_scroll: 0, main_focus: true }
                    }
                    1 => {
                        TermSubState::Resume {  }
                    }
                    2 => {
                        TermSubState::Contact {  }
                    }
                    _ => panic!("")

                };
                return;
            }

            if index_changed {
                terminal_state.highlighted_boxes = vec![generate_highlight_box(*index + 1).expect("Input should always return a box")];
            }
        }

        _ => ()

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
