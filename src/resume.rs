use macroquad::prelude::*;
use crate::{models::{Cell, TermSubState, TerminalState}, utils::print_cells};

pub async fn setup_resume(terminal_state: &mut TerminalState) {
    const RESUME_W : usize = 80;
    const RESUME_H : usize = 20;

    terminal_state.cell_buffer = vec![vec![Cell::default(); RESUME_W]; RESUME_H];
    log::info!("Attempting to download resume md");
    let mark_down_text = String::from_utf8(load_file("resume.md").await.expect("Could not load resume")).expect("Could not decode resume");
    log::info!("Downloaded resume md");
    let mut markdown_panel = crate::markdown_renderer::render_markdown(&mark_down_text, RESUME_W, RESUME_H);
    log::info!("Rendered Markdown");


    markdown_panel.fit_buffer();
    markdown_panel.write_to_buffer(&mut terminal_state.cell_buffer);

    print_cells(&markdown_panel.cells);

    terminal_state.sub_state = TermSubState::Resume(markdown_panel);
}

pub fn update_resume_buffer(terminal_state: &mut TerminalState) {
    if let TermSubState::Resume(ref mut markdown_panel) = terminal_state.sub_state {
        markdown_panel.write_to_buffer(&mut terminal_state.cell_buffer);
    }
}
