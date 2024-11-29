use std::{sync::{Arc, Mutex}, thread};

use macroquad::{experimental::coroutines::start_coroutine, file::load_file, miniquad::fs::Response};
pub enum LoadingState {
    Loading,
    Finished(Response)
}

pub trait BackgroundLoadable {
    fn load_file(file_path: String) -> Self;

    fn check_file(&self) -> bool;

    fn get_file(&self) -> Vec<u8>;
}

pub fn background_load_file(path: &str) {
    let new_path = path.to_string();
    let coroutine = start_coroutine(async move {
        load_file(&new_path).await;
    });

    loop {
        coroutine.is_done();
        coroutine.retrieve();

    }

}

#[cfg(not(target_arch = "wasm32"))]
pub mod background_file {
    use super::*;
    pub struct BackgroundFile {
        pub file_loading_state: Arc<Mutex<LoadingState>>
    }

    impl BackgroundLoadable for BackgroundFile {

        fn load_file(file_path: String) -> Self {
            let file_reference = Arc::new(Mutex::new(LoadingState::Loading));
            let thread_file_reference = file_reference.clone();
            thread::spawn(move || {
                macroquad::miniquad::fs::load_file(&file_path, move |file_response| {
                    match thread_file_reference.lock() {
                        Ok(mut file_reference) => *file_reference = LoadingState::Finished(file_response),
                        Err(_) => panic!("Failed"),
                    }
                })
            });

            BackgroundFile {
                file_loading_state: file_reference
            }
        }

        fn check_file(&self) -> bool {
            let file_loading_state = self.file_loading_state.lock().expect("Could not lock file");

            match &(*file_loading_state) {
                LoadingState::Loading => false,
                LoadingState::Finished(_) => true,
            }
        }

        fn get_file(&self) -> Vec<u8> {
            let file_loading_state = self.file_loading_state.lock().expect("Could not lock file");
            match &(*file_loading_state) {
                LoadingState::Loading => panic!("Should not be loading"),
                LoadingState::Finished(val) => val.as_ref().expect("Error downloading file").clone(),

            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub mod background_file {
    use super::*;
    use log::debug;

    pub struct  BackgroundFile {
        pub file_path: String,
    }


    impl BackgroundLoadable for BackgroundFile {
        
        fn load_file(file_path: String) -> Self {
            debug!("Started loading file: {}", &file_path);
            BackgroundFile {
                file_path
            }
        }

        fn check_file(&self) -> bool {
            false
        }

        fn get_file(&self) -> Vec<u8> {
            Vec::new()
        }
    }
}
