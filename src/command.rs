// use super::Graphics::GraphicsType
use crossbeam_channel as channel;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;
// use std::time::Instant;


//use super::uifuncs::{TUIFuncs, TUIGraphics, generate_uifuncs, UIFuncsType};
use super::uifuncs::{UIGraphics, RenderColor};
use super::stage::{Stage, StageConfig};
use super::audiofuncs::{AudioFuncs};


pub enum Command {
    AppearCommand(HashMap<String, String>),
    DisappearCommand(HashMap<String, String>),
    TelopCommand(HashMap<String, String>),
    TitleCommand(HashMap<String, String>),
    SubTelopCommand(HashMap<String, String>),
    SpeechCommand(HashMap<String, String>),
    CharacterCommand(HashMap<String, String>),
}

pub struct CommandClient {
    chan_tx: channel::Sender<Command>,
}

#[derive(Debug, Copy , Clone)]
pub struct ColorConfig {
    pub normal: RenderColor,
    pub normal_bg: RenderColor,
    pub info: RenderColor,
    pub info_bg: RenderColor,
}

impl CommandClient {
    pub fn appear(&self, filename: &str, name: &str) {
        let mut params = HashMap::new();
        params.insert(String::from("filename"), filename.to_string());
        params.insert(String::from("name"), name.to_string());
        let cmd = Command::AppearCommand(params);
        self.chan_tx.send(cmd).unwrap();
    }
    pub fn disappear(&self, name: &str) {
        let mut params = HashMap::new();
        params.insert(String::from("name"), name.to_string());
        let cmd = Command::DisappearCommand(params);
        self.chan_tx.send(cmd).unwrap();
    }
    pub fn telop(&self, text: &str, pos: usize) {
        let mut params = HashMap::new();
        params.insert(String::from("text"), text.to_string());
        params.insert(String::from("pos"), pos.to_string());
        let cmd = Command::TelopCommand(params);
        self.chan_tx.send(cmd).unwrap();
    }
    pub fn title(&self, filename: &str) {
        let mut params = HashMap::new();
        params.insert(String::from("filename"), filename.to_string());
        let cmd = Command::TitleCommand(params);
        self.chan_tx.send(cmd).unwrap();
    }
    pub fn subtelop(&self, text: &str, pos: usize) {
        let mut params = HashMap::new();
        params.insert(String::from("text"), text.to_string());
        params.insert(String::from("pos"), pos.to_string());
        let cmd = Command::SubTelopCommand(params);
        self.chan_tx.send(cmd).unwrap();
    }
    pub fn speech(&self, text: &str, lang: &str) {
        let mut params = HashMap::new();
        params.insert(String::from("text"), text.to_string());
        params.insert(String::from("lang"), lang.to_string());
        let cmd = Command::SpeechCommand(params);
        self.chan_tx.send(cmd).unwrap();
    }
    pub fn character(&self, ch: char) {
        let mut params = HashMap::new();
        params.insert(String::from("ch"), ch.to_string());
        let cmd = Command::CharacterCommand(params);
        self.chan_tx.send(cmd).unwrap();
    }
}

pub fn start_command_server(graphics: Box<dyn UIGraphics>, config: StageConfig, color_config: ColorConfig, audio: Box<dyn AudioFuncs>) -> CommandClient {
    let (chan_tx, chan_rx) = channel::unbounded::<Command>();
    thread::spawn(move || {
        let mut stage = Stage::new(config);
        let mut current_color = color_config.normal;
        let mut current_bg = color_config.normal_bg;
        let interval = 1000 / config.framerate as u64;

        loop {
            channel::select! {
                recv(chan_rx) -> received => {
                    match received {
                        Ok(cmd) => {
                            match cmd {
                                Command::AppearCommand(data) => {
                                    if let Some(filename) = data.get("filename") {
                                        stage.appear(filename);
                                    }
                                    if let Some(name) = data.get("name") {
                                        //audio.speech(name);
                                    }
                                },
                                Command::DisappearCommand(data) => {
                                    if let Some(name) = data.get("name") {
                                        //audio.speech_lang(&format!("{},Get", name), "ja");
                                    }
                                    stage.disappear();
                                },
                                Command::TelopCommand(data) => {
                                    let mut text = "";
                                    if let Some(text_data) = data.get("text") {
                                        text = text_data;
                                    }

                                    let pos: usize = data.get("pos").unwrap_or(&String::from("0")).parse().unwrap();
                                    let offset = stage.telop_offset();
                                    /*
                                    if pos > 0  && pos < text.len() {
                                        let ch = text.chars().nth(pos-1).unwrap().to_ascii_lowercase();
                                        audio.speech(&ch.to_string());
                                    }
                                    */
                                    graphics.draw_area(&current_color, &current_bg, &stage.update_telop(text, pos).buffer, Some((0, offset)));
                                    graphics.flush();
                                },
                                Command::TitleCommand(data) => {
                                    if let Some(filename) = data.get("filename") {
                                        stage.title(filename);
                                    }
                                },
                                Command::SubTelopCommand(data) => {
                                    let mut text = "";
                                    if let Some(text_data) = data.get("text") {
                                        text = text_data;
                                    }

                                    let pos: usize = data.get("pos").unwrap_or(&String::from("0")).parse().unwrap();
                                    let offset = stage.subtelop_offset();
                                    graphics.draw_area(&current_color, &current_bg, &stage.update_subtelop(text, pos).buffer, Some((0, offset)));
                                    graphics.flush();
                                },
                                Command::SpeechCommand(data) => {
                                    if let Some(text) = data.get("text") {
                                        if let Some(lang) = data.get("lang") {
                                            audio.speech_lang(text, lang);
                                        } else {
                                            audio.speech(text);
                                        }
                                    }
                                },
                                Command::CharacterCommand(data) => {
                                    if let Some(ch) = data.get("ch") {
                                        if let Some(character) = ch.chars().nth(0) {
                                            stage.update_character(character);
                                        }
                                    }
                                }
                            }
                        },
                        Err(_) => {
                            break;
                        }
                    }
                },
                default(Duration::from_millis(interval)) => {
                    if stage.has_typed_char() {
                        current_color = color_config.info;
                        current_bg = color_config.info_bg
                    } else {
                        current_color = color_config.normal;
                        current_bg = color_config.normal_bg;
                    }
                    graphics.draw_area(&current_color, &current_bg, &stage.update_pict().buffer, Option::None);
                    graphics.flush();
                }
            }
        }
    });

    CommandClient {
        chan_tx: chan_tx,
    }

}

/*
#[test]
fn command_works() {
    let funcs = generate_uifuncs(UIFuncsType::TUI);
    //let funcs = generate_uifuncs(UIFuncsType::DEBUG);
    let graphics = funcs.get_graphics();
    let config = StageConfig {
        stage_wxh: (600, 150),
        aa_width: 250,
        framerate: 10,
    };
    let command_client = start_command_server(graphics, config, RenderColor::Default, RenderColor::Default, RenderColor::Blue);
    command_client.appear("/Users/shizuku/drawings/001_megamouse_shark.png");
    command_client.telop("hello", 0);
    thread::sleep(Duration::from_secs(10));
    command_client.telop("hello", 1);
    thread::sleep(Duration::from_secs(10));
    command_client.telop("hello", 2);
    thread::sleep(Duration::from_secs(10));
    command_client.disappear();
    thread::sleep(Duration::from_secs(20));

}
*/