
/*
mod uifuncs;
use uifuncs::{generate_uifuncs, UIFuncs, UIFuncsType, RenderColor};
mod stage;
use stage::{StageConfig};
mod command;
use command::{start_command_server};

use std::thread;
use std::time::{Duration};
*/

use std::thread;
use std::time::Duration;

use aquatyping;
use aquatyping::uifuncs;
use aquatyping::stage;
use aquatyping::command;
use aquatyping::audiofuncs;
use aquatyping::controller;


fn main() {


    let funcs = uifuncs::generate_uifuncs(uifuncs::UIFuncsType::TUI);
    //let funcs = generate_uifuncs(UIFuncsType::DEBUG);
    //let graphics = funcs.get_graphics();
    let config = stage::StageConfig {
        //stage_wxh: (400, 120),
        stage_wxh: (640, 180), //140
        aa_width: 300, //250
        framerate: 10,
    };
    let audio = audiofuncs::generate_audiofuncs(audiofuncs::AudioFuncsType::OsCommand);
    let color_config = command::ColorConfig {
        normal: uifuncs::RenderColor::Black,
        normal_bg: uifuncs::RenderColor::White,
        info: uifuncs::RenderColor::Blue,
        info_bg: uifuncs::RenderColor::White,
    };

    controller::control(funcs, color_config, audio, "/Users/shizuku/drawings/", config, 120);
    /*
    let command_client = command::start_command_server(graphics, config, color_config, audio);
    command_client.appear("/Users/shizuku/drawings/001_megamouse_shark.png", "MEGAMOUSE SHARK");
    command_client.telop("MEGAMOUSE SHARK", 0);
    thread::sleep(Duration::from_secs(10));
    command_client.telop("MEGAMOUSE SHARK", 1);
    thread::sleep(Duration::from_secs(10));
    command_client.telop("MEGAMOUSE SHARK", 2);
    thread::sleep(Duration::from_secs(10));
    command_client.disappear();
    thread::sleep(Duration::from_secs(20));
    */
    
 
}