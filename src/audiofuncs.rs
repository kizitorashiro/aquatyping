use std::process::Command;

pub trait AudioFuncs: Send {
    fn speech(&self, text: &str);
    fn speech_lang(&self, text: &str, lang: &str);
    //fn play(filepath: &str);
}

#[derive(Debug, Copy, Clone)]
pub enum AudioFuncsType {
    OsCommand,
}

pub fn generate_audiofuncs(audiofuncs_type: AudioFuncsType) -> Box<dyn AudioFuncs> {
    match audiofuncs_type {
        AudioFuncsType::OsCommand => {
            Box::new(OsCommandAudioFuncs::new())
        }
    }
}

pub struct OsCommandAudioFuncs {

}

impl OsCommandAudioFuncs {
    pub fn new() -> Self {
        OsCommandAudioFuncs{}
    }
}

impl AudioFuncs for OsCommandAudioFuncs {
    #[cfg(target_os = "macos")]
    fn speech(&self, text: &str) {
        let t = text.to_string();
        std::thread::spawn(move || {
            let mut c = Command::new("say")
            .args(&[&t])
            .spawn()
            .expect("fail to execute say command");
            c.wait();    
        });
    }
    #[cfg(target_os = "macos")]
    fn speech_lang(&self, text: &str, lang: &str) {
        let text = text.to_string();
        match lang {
            "en" => {
                std::thread::spawn(move || {
                    let mut c = Command::new("say")
                        .args(&[&text])
                        .spawn()
                        .expect("fail to execute say command");//.kill();
                    c.wait();
                });
            },
            "ja" => {
                std::thread::spawn(move || {
                    let mut c = Command::new("say")
                        .args(&["-v","Otoya",&text])
                        .spawn()
                        .expect("fail to execute say -v command");//.kill();
                    c.wait();
                });
            },
            _ => {
            }
        }
    }


    #[cfg(target_os = "windows")]
    fn speech(&self, text: &str) {
    }
}

#[test]
fn speech_works() {
    let funcs = generate_audiofuncs(AudioFuncsType::OsCommand);
    println!("call");
    funcs.speech("hello megamouse shark");
    println!("end");
    funcs.speech("bye");
}