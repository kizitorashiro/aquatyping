use crossbeam_channel as channel;
use std::time::{Duration, SystemTime, Instant};
use super::uifuncs::{UIFuncs, UIGraphics, UIKeyEvent};
use super::audiofuncs::{AudioFuncs};
use super::stage::{Stage, StageConfig};
use super::command::{Command, CommandClient, start_command_server, ColorConfig};
use super::pict::{PictManager, Pict};

trait Controller{
    fn handle_key_event(&mut self, context: &mut ControlContext, keyevent: &UIKeyEvent) -> ControlState;
    fn handle_timer_event(&mut self, context: &mut ControlContext) -> ControlState;
}

pub enum ControlState {
    EXECUTING,
    FINISHED(ControlMode),
}

#[derive(Debug)]
pub enum ControlMode {
    TITLE,
    TYPING,
//    RESULT,
}

struct ControlContext {
    //pict_manager: PictManager,
    pict_dir: String,
    command_client: CommandClient,
    num_of_targets: usize,
    results: Vec<TypingResult>,
    //results: &'a Vec<Result>,
}

fn generate_controller(mode: ControlMode, context: &mut ControlContext) -> Box<dyn Controller> {
    match mode {
        ControlMode::TITLE => {
            Box::new(TitleController::new(context))
        },
        ControlMode::TYPING => {
            Box::new(TypingController::new(context))
        },
    }
} 

pub fn control(mut uifuncs: Box<dyn UIFuncs>, color_config: ColorConfig, audiofuncs: Box<dyn AudioFuncs>, pict_dir: &str, stage_config: StageConfig, num_of_targets: usize) {

    let graphics = uifuncs.get_graphics();
    let keyevent_rx = uifuncs.start_keyevent_thread();

    let command_client = start_command_server(graphics, stage_config, color_config, audiofuncs);

    //let pict_manager = PictManager::new(pict_dir);
    // commandserverとkeyeventスレッドを終わらせる

    let mut context = ControlContext {
        //pict_manager: pict_manager,
        pict_dir: pict_dir.to_string(),
        command_client: command_client,
        num_of_targets: num_of_targets,
        results: Vec::new(),
    };

    let mut mode = ControlMode::TITLE;
    let mut controller = generate_controller(mode, &mut context);

    loop {
        channel::select! {
            recv(keyevent_rx) -> received => {
                match received {
                    Ok(event) => {
                        match event {
                            UIKeyEvent::ESC => {
                                break;
                            },
                            _ => {
                                if let ControlState::FINISHED(next_mode) = controller.handle_key_event(&mut context, &event ) {
                                    mode = next_mode;
                                    controller = generate_controller(mode, &mut context);
                                }
                            }
                        }
                    },
                    Err(_) => {
                        panic!("error");
                    }
                }
            },
            default(Duration::from_millis(100)) => {
                if let ControlState::FINISHED(next_mode) = controller.handle_timer_event(&mut context) {
                    mode = next_mode;
                    controller = generate_controller(mode, &mut context); 
                }
            }
        }
    }
    

}


struct TitleController {

}

impl TitleController {
    fn new(context: &mut ControlContext) -> Self {
        let pict_manager = PictManager::new(&context.pict_dir);
        let pict = pict_manager.get_title_by_id("T01.png");
        let filename = pict_manager.get_pict_path(pict.unwrap());
        context.command_client.title(&filename);
        context.command_client.telop("PRESS SPACE KEY", 0);
        TitleController {}
    }
}

impl Controller for TitleController {
    fn handle_key_event(&mut self, context: &mut ControlContext, keyevent: &UIKeyEvent) -> ControlState{
        match *keyevent {
            UIKeyEvent::Char(' ') => {
                ControlState::FINISHED(ControlMode::TYPING)
            },
            _ => {
                ControlState::EXECUTING
            }
        }
    }
    fn handle_timer_event(&mut self, context: &mut ControlContext) -> ControlState {
        ControlState::EXECUTING
    }
}

struct TypingController {
    pict_manager: PictManager,
    index_series: Vec<usize>,
    typing_info: Option<TypingInfo>,
}

enum TypingStatus {
    TYPING,
    IDLING,
}

struct TypingInfo {
    filepath: String,
    words: String,
    words_ja: String,
    pos: usize,
    start_time: Instant,
    typo: u32,
    status: TypingStatus,
}

struct TypingResult {
    filepath: String,
    words: String,
    time: u128,
    typo: u32,
}


impl TypingController {
    
    fn new(context: &mut ControlContext) -> TypingController {
        context.results = Vec::new();
        let pict_manager = PictManager::new(&context.pict_dir);
        let index_series = pict_manager.index_series(context.num_of_targets);
        TypingController {
            pict_manager: pict_manager,
            index_series: index_series,
            typing_info: None,
        }
    }
    
    fn load_pict(&mut self, command_client: &CommandClient) -> bool {
        let index = self.index_series.pop();
        match index {
            Some(i) => {
                if let Some(pict) = self.pict_manager.get_pict(i) {
                    let typing_info = TypingInfo {
                        filepath: self.pict_manager.get_pict_path(pict),
                        words: pict.en.to_string(),
                        words_ja: pict.ja.to_string(),
                        pos: 0,
                        start_time: Instant::now(),
                        typo: 0,
                        status: TypingStatus::TYPING,
                    };
                    command_client.appear(&self.pict_manager.get_pict_path(pict), &pict.en);
                    command_client.telop(&typing_info.words, 0);
                    self.typing_info = Some(typing_info);
                    true
                } else {
                    false
                }
            },
            None => {
                false
            }
        }
    }

    fn unload_pict(&mut self, command_client: &CommandClient) -> Option<TypingResult> {
    
        if let Some(info) = &mut self.typing_info {
            command_client.disappear(&info.words_ja);
        
            let result = TypingResult {
                filepath: (&info.filepath).to_string(),
                words: (&info.words).to_string(),
                time: info.start_time.elapsed().as_millis(),
                typo: info.typo,
            };
            info.status = TypingStatus::IDLING;
            info.start_time = Instant::now(); 
            Some(result)
        } else {
            None
        }
    }

    fn handle_input_char(&mut self, input_ch: char, command_client: &CommandClient) -> Option<TypingResult>{
        if let Some(info) = &mut self.typing_info {
            match info.status {
                TypingStatus::TYPING => {
                    let target = info.words.chars().nth(info.pos).unwrap();
                    //panic!("chchchchc  {} {}", input_ch, target);
                    
                    if target.to_ascii_lowercase() == input_ch {
                        info.pos += 1;
                        command_client.telop(&info.words, info.pos);

                        if info.pos >= info.words.len() {
                            return self.unload_pict(command_client);
                        }
                    } else {
                        info.typo += 1;
                    }
                    
                },
                TypingStatus::IDLING => {}
            }
        }
        None
    }

    fn handle_periodical_event(&mut self, command_client: &CommandClient) -> bool{
        if let Some(info) = &mut self.typing_info {
            match info.status {
                TypingStatus::TYPING => {},
                TypingStatus::IDLING => {
                    if info.start_time.elapsed().as_millis() > 3000 {
                        return self.load_pict(command_client);
                    }
                }
            }
        } else {
            return self.load_pict(command_client);
        }
        true
    }

}

impl Controller for TypingController {
    fn handle_key_event(&mut self, context: &mut ControlContext, keyevent: &UIKeyEvent) -> ControlState {
        match *keyevent {
            UIKeyEvent::Char(ch) => {
                if let Some(result) = self.handle_input_char(ch, &context.command_client) {
                    context.results.push(result);
                }
            },
            _ => {}
        }
        
        ControlState::EXECUTING
    }

    fn handle_timer_event(&mut self, context: &mut ControlContext) -> ControlState{
        if self.handle_periodical_event(&context.command_client) {
            ControlState::EXECUTING
        } else {
            ControlState::FINISHED(ControlMode::TITLE)
        }
    }
}

#[test]
fn controller_works() {
    let funcs = super::uifuncs::generate_uifuncs(super::uifuncs::UIFuncsType::DEBUG);
    //let funcs = generate_uifuncs(UIFuncsType::DEBUG);
    //let graphics = funcs.get_graphics();
    let config = super::stage::StageConfig {
        stage_wxh: (400, 120),
        aa_width: 250, //250
        framerate: 10,
    };
    let audio = super::audiofuncs::generate_audiofuncs(super::audiofuncs::AudioFuncsType::OsCommand);
    let color_config = super::command::ColorConfig {
        normal: super::uifuncs::RenderColor::Black,
        normal_bg: super::uifuncs::RenderColor::White,
        info: super::uifuncs::RenderColor::Blue,
        info_bg: super::uifuncs::RenderColor::White,
    };

    super::controller::control(funcs, color_config, audio, "/Users/shizuku/drawings/", config, 2);



}

