use macroquad::prelude::*;
use crate::models::Cell;
use crate::{
    markdown_renderer::render_markdown, models::{CellPanel, ProjectInfo, TermSubState, TerminalState}, 
    utils::write_cell_panels_with_border
};

const PROJECT_SIDE_WIDTH: usize = 21;
const PROJECT_ART_HEIGHT: usize = 8;
const TERM_HEIGHT: usize = 20;
const TERM_WIDTH: usize = 80;

pub const ABOUT_PANEL_INDEX: usize = 0;
pub const ART_PANEL_INDEX: usize = 1;
pub const PROJECTS_PANEL_INDEX: usize = 2;


pub async fn setup_projects(terminal_state: &mut TerminalState) {

    let mut projects_str: Vec<String> = terminal_state.projects.iter().map(|project: &ProjectInfo| project.name.clone()).collect();

    let project = &terminal_state.projects[0];

    
    if let None = terminal_state.loaded_projects.get(&project.markdown) {
        let file_data = load_file(&format!("projects/{}", &project.markdown)).await.expect("Could not load markdown");
        let text = String::from_utf8(file_data).expect("Could not parse markdown");
        terminal_state.loaded_projects.insert(project.markdown.clone(), text);
    };

    let markdown = &terminal_state.loaded_projects[&project.markdown];

    let mut about_panel = render_markdown(markdown, 
        TERM_WIDTH - PROJECT_SIDE_WIDTH - 1, 
        TERM_HEIGHT - 2);

    about_panel.offset_x = 1;
    about_panel.offset_y = 1;

    about_panel.fit_buffer();

    let art_panel = CellPanel::from_strings(&project.ascii_art, 
        PROJECT_SIDE_WIDTH - 2, 
        PROJECT_ART_HEIGHT - 2, 
        TERM_WIDTH - PROJECT_SIDE_WIDTH + 1, 1);

    let mut projects_panel = CellPanel::from_strings(&projects_str, 
        PROJECT_SIDE_WIDTH - 2,
        TERM_HEIGHT - PROJECT_ART_HEIGHT - 1, 
        TERM_WIDTH - PROJECT_SIDE_WIDTH + 1, 
        PROJECT_ART_HEIGHT );

    if let Some(cell_line) = projects_panel.cells.get_mut(0) {
        for cell in cell_line.iter_mut() {
            cell.background_color = Some(&WHITE);
        }
        cell_line.insert(0, Cell::new(' '));
        cell_line.insert(0, Cell::new('>'));
    }

    projects_panel.box_color = Some(&WHITE);
    projects_panel.fit_buffer();

    let cell_project_panels = vec![
        about_panel,
        art_panel,
        projects_panel
    ];
    
    //let first_project_str = &projects_str[0].clone();

    terminal_state.cell_buffer = write_cell_panels_with_border(&cell_project_panels, TERM_WIDTH, TERM_HEIGHT);
    terminal_state.sub_state = TermSubState::Projects { 
        selected_project_index: 0, 
        project_about_scroll: 0, 
        main_focus: false, 
        cell_panels: cell_project_panels,
    };
}

pub async fn update_project_buffer( 
    terminal_state: &mut TerminalState, 
) {
    if let TermSubState::Projects { selected_project_index, project_about_scroll, ref main_focus, ref mut cell_panels, .. } = terminal_state.sub_state {
        
        let selected_project = &terminal_state.projects[selected_project_index];
        
        if let None = terminal_state.loaded_projects.get(&selected_project.markdown) {
            let file_data = load_file(&format!("projects/{}", &selected_project.markdown)).await.expect("");
            let text = String::from_utf8(file_data).expect("Could not parse markdown text");
            terminal_state.loaded_projects.insert(selected_project.markdown.clone(), text);
        }


        let markdown = &terminal_state.loaded_projects[&selected_project.markdown];

        let mut about_panel = render_markdown(markdown, TERM_WIDTH - PROJECT_SIDE_WIDTH - 1, TERM_HEIGHT - 2);
        about_panel.offset_y = 1;
        about_panel.offset_x = 1;
        about_panel.index = project_about_scroll;
        about_panel.fit_buffer();

        cell_panels[ABOUT_PANEL_INDEX] = about_panel;

        if let Some(projects_panel) = cell_panels.get_mut(PROJECTS_PANEL_INDEX) {

            projects_panel.cells.iter_mut().for_each(|cell_line| {
                if cell_line[0].char == '>' {
                    cell_line.remove(0);
                    cell_line.remove(0);
                    
                    cell_line.iter_mut().for_each(|cell| {
                        cell.background_color = None;
                    })
                }
            });

            if let Some(cell_line) = projects_panel.cells.get_mut(selected_project_index) {
                cell_line.iter_mut().for_each(|cell| {
                    cell.background_color = Some(&WHITE);
                });

                cell_line.insert(0, Cell::new(' '));
                cell_line.insert(0, Cell::new('>'));
            }
            projects_panel.fit_buffer();
        }

        if let Some(art_panel) = cell_panels.get_mut(ART_PANEL_INDEX) {
            art_panel.update_from_strings(&terminal_state.projects[selected_project_index].ascii_art);
        }

        match main_focus {
            true => {
                cell_panels[PROJECTS_PANEL_INDEX].box_color = None; 
                cell_panels[ABOUT_PANEL_INDEX].box_color = Some(&WHITE);
                terminal_state.cell_buffer = write_cell_panels_with_border(&cell_panels, TERM_WIDTH, TERM_HEIGHT);
            },
            false => {
                cell_panels[ABOUT_PANEL_INDEX].box_color = None;
                cell_panels[PROJECTS_PANEL_INDEX].box_color = Some(&WHITE);
                terminal_state.cell_buffer = write_cell_panels_with_border(&cell_panels, TERM_WIDTH, TERM_HEIGHT);
            }

        }
    }
    else {
        panic!("Update project buffer should only be called if sub_state is project");
    }
}
