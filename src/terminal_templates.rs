use macroquad::math::Rect;
use crate::TermSubState;



pub const LOAD_TEMPLATE: [&'static str; 10] = [
"▄████▄   ▄▄▄       ███▄ ▄███▓ ▄▄▄▄    █    ██  ▄████▄   ██░ ██  ▄▄▄           ▓█████▄ ▓█████  ██▒   █▓",
"▒██▀ ▀█  ▒████▄    ▓██▒▀█▀ ██▒▓█████▄  ██  ▓██▒▒██▀ ▀█  ▓██░ ██▒▒████▄         ▒██▀ ██▌▓█   ▀ ▓██░   █▒",
"▒▓█    ▄ ▒██  ▀█▄  ▓██    ▓██░▒██▒ ▄██▓██  ▒██░▒▓█    ▄ ▒██▀▀██░▒██  ▀█▄       ░██   █▌▒███    ▓██  █▒░",
"▒▓▓▄ ▄██▒░██▄▄▄▄██ ▒██    ▒██ ▒██░█▀  ▓▓█  ░██░▒▓▓▄ ▄██▒░▓█ ░██ ░██▄▄▄▄██      ░▓█▄   ▌▒▓█  ▄   ▒██ █░░",
"▒ ▓███▀ ░ ▓█   ▓██▒▒██▒   ░██▒░▓█  ▀█▓▒▒█████▓ ▒ ▓███▀ ░░▓█▒░██▓ ▓█   ▓██▒ ██▓ ░▒████▓ ░▒████▒   ▒▀█░  ",
"░ ░▒ ▒  ░ ▒▒   ▓▒█░░ ▒░   ░  ░░▒▓███▀▒░▒▓▒ ▒ ▒ ░ ░▒ ▒  ░ ▒ ░░▒░▒ ▒▒   ▓▒█░ ▒▓▒  ▒▒▓  ▒ ░░ ▒░ ░   ░ ▐░  ",
  "░  ▒     ▒   ▒▒ ░░  ░      ░▒░▒   ░ ░░▒░ ░ ░   ░  ▒    ▒ ░▒░ ░  ▒   ▒▒ ░ ░▒   ░ ▒  ▒  ░ ░  ░   ░ ░░  ",
"░          ░   ▒   ░      ░    ░    ░  ░░░ ░ ░ ░         ░  ░░ ░  ░   ▒    ░    ░ ░  ░    ░        ░░  ",
"░ ░            ░  ░       ░    ░         ░     ░ ░       ░  ░  ░      ░  ░  ░     ░       ░  ░      ░  ",
"░                                   ░          ░                            ░   ░                  ░   ",
];

pub const MAIN_TEMPLATE: [&'static str; 11] = [
"Welcome to CAMBUCHA.DEV (TM) TermLink",
"Select One",
"|==========================================|",
"|                                          |",
"|                                          |",
"|               1.Projects                 |",
"|               2.Resume                   |",
"|               3.Contact                  |",
"|                                          |",
"|                                          |",
"|==========================================|",
];

pub fn generate_highlight_box(index: usize) -> Option<Rect> {
    let coords = MAIN_TEMPLATE.iter().enumerate().find_map(|line| {
        match line.1.find(&index.to_string()) {
            Some(val) => Some((val, line.0)),
            None => None
        }
    })?;

    let count = MAIN_TEMPLATE[coords.1][coords.0..].find(" ")?;

    Some(Rect { x: (coords.0 ) as f32, y: (coords.1 ) as f32, w: count as f32, h: 1 as f32 } )
}

pub const SAND_SPINNER: [&'static str;35] = [
    		"⠁",
			"⠂",
			"⠄",
			"⡀",
			"⡈",
			"⡐",
			"⡠",
			"⣀",
			"⣁",
			"⣂",
			"⣄",
			"⣌",
			"⣔",
			"⣤",
			"⣥",
			"⣦",
			"⣮",
			"⣶",
			"⣷",
			"⣿",
			"⡿",
			"⠿",
			"⢟",
			"⠟",
			"⡛",
			"⠛",
			"⠫",
			"⢋",
			"⠋",
			"⠍",
			"⡉",
			"⠉",
			"⠑",
			"⠡",
			"⢁"
];


pub const BALLOON_SPINNER: [&'static str; 7] = [
    ".",
    "o",
    "O",
    "°",
    "O",
    "o",
    "."
];

pub const BALLOON_SPINNER_CHARS: [char; 7] = [
    '.',
    'o',
    'O',
    '°',
    'O',
    'o',
    '.'
];
