use macroquad::{color::{Color, GREEN}, math::{RectOffset, Vec2}, text::Font, ui::{hash, root_ui, Skin}};

#[derive(Default)]
pub struct UiContext {
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub right_pressed: bool,
    pub left_pressed: bool,
    pub enter_pressed: bool,
}

impl UiContext {
    pub fn reset(&mut self) {
        self.up_pressed = false;
        self.down_pressed = false;
        self.left_pressed = false;
        self.right_pressed = false;
        self.enter_pressed = false;
    }
}

const BUTTON_SPACING : f32 = 20f32;
const FONT_SIZE: f32 = 40f32;
const MARGIN: f32 = 10f32;

const TRANSPARENT: Color = Color::new(0f32, 0f32, 0f32, 0f32);
const TRANSPARENT_WHITE: Color = Color::new(1f32, 1f32, 1f32, 0.25f32);

pub fn create_ui_skin(font: &Font) -> Skin {

    let button_style = root_ui()
        .style_builder()
        .with_font(font)
        .unwrap()
        .text_color(GREEN)
        .font_size(40)
        .color(TRANSPARENT)
        .color_hovered(TRANSPARENT_WHITE)
        .margin(RectOffset::new(10.0, 10.0, 10.0, 10.0) )
        .build();

    Skin {
        button_style,
        ..root_ui().default_skin()
    }

}

pub fn handle_ui(screen_w: f32, screen_h: f32, ui_context: &mut UiContext, ui_skin: &Skin) {
    root_ui().push_skin(ui_skin);
    root_ui().group(hash!(), Vec2::new(screen_w - 1f32, screen_h - 1f32), |ui| {
        // UP Button
        
        let up_button_position = generate_pos(2f32, 2f32, screen_w, screen_h);
        ui_context.up_pressed = ui.button(Some(up_button_position), "↑");

        // ENTER Button
        let enter_button_position = generate_pos(1f32, 2f32, screen_w, screen_h);
        ui_context.enter_pressed = ui.button(Some(enter_button_position), "↵"); 

        // DOWN Button
        let down_button_position = generate_pos(2f32, 1f32, screen_w, screen_h);
        ui_context.down_pressed = ui.button(Some(down_button_position), "↓");

        let left_button_position = generate_pos(3f32, 1f32, screen_w, screen_h);
        ui_context.left_pressed = ui.button(Some(left_button_position), "←");

        let right_button_position =  generate_pos(1f32, 1f32, screen_w, screen_h)   ;
        ui_context.right_pressed = ui.button(Some(right_button_position), "→");
    });
    root_ui().pop_skin();
}

fn generate_pos(buttons_from_right: f32, buttons_from_bottom: f32, screen_w: f32, screen_h: f32) -> Vec2 {
    let button_width = FONT_SIZE / 2f32 + MARGIN * 2.0;
    let button_height = FONT_SIZE + MARGIN * 2.0;
    Vec2 {
        x: screen_w - ((button_width + BUTTON_SPACING) * buttons_from_right ),
        y: screen_h - ((button_height + BUTTON_SPACING) * buttons_from_bottom),

    }
}
