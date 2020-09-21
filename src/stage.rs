use rand::{thread_rng, Rng};
use image2ascii::{Char2DArray, CharPosition};
use super::asciiart::{AsciiArt, AsciiArtContext, AsciiArtState};
use super::background::{Background, BackgroundType, generate_background_randomly};
use super::effector::EffectorType;
use super::behavior::BehaviorType;
use image2ascii::string2ascii;
use super::character::{Character, CharacterConfig, CharacterStatus};


pub struct Stage {
    pict_area: Char2DArray,
    telop_area: Char2DArray,
    subtelop_area: Char2DArray,
    ascii_art: Option<AsciiArt>,
    background: Option<Box<dyn Background>>,
    config: StageConfig,
    //typed_char: Option<char>,
    typed_char: Option<Character>,

}

#[derive(Debug, Copy, Clone)]
pub struct StageConfig {
    pub stage_wxh: (usize, usize),
    pub aa_width: usize,
    pub framerate: u32,
}

impl Stage {

    pub fn new(config: StageConfig) -> Self {
        let pict_area_height = ((config.stage_wxh.1 as f32 / 6.0) * 4.0) as usize;
        let telop_area_height = ((config.stage_wxh.1 - pict_area_height) as f32 / 2.0) as usize;
        let subtelop_area_height = config.stage_wxh.1 - pict_area_height - telop_area_height;
        Stage {
            pict_area: Char2DArray::new(config.stage_wxh.0, pict_area_height),
            telop_area: Char2DArray::new(config.stage_wxh.0, telop_area_height),
            subtelop_area: Char2DArray::new(config.stage_wxh.0, subtelop_area_height),
            ascii_art: Option::None,
            background: Option::None,
            config: config,
            typed_char: Option::None,
        }
    }

    pub fn title(&mut self, image_file: &str) {
        let context = AsciiArtContext{
            stage_wxh: self.config.stage_wxh,
            aa_width: self.config.stage_wxh.0 / 2,
            framerate: self.config.framerate,
        };
        if let Ok(aa) = AsciiArt::from_image(image_file, &context, EffectorType::NO, EffectorType::NO, BehaviorType::NO) {
            self.ascii_art = Option::Some(aa);
        } else {
            self.ascii_art = Option::None;
        }
        self.background = None;
    }

    pub fn telop_offset(&self) -> usize {
        self.pict_area.height()
    }

    pub fn subtelop_offset(&self) -> usize {
        self.pict_area.height() + self.telop_area.height()
    }

    pub fn appear(&mut self, image_file: &str){
        let context = AsciiArtContext{
            stage_wxh: self.config.stage_wxh,
            aa_width: self.config.aa_width,
            framerate: self.config.framerate,
        };
        if let Ok(aa) = AsciiArt::from_image_easy(image_file, &context) {
            self.ascii_art = Option::Some(aa);
        } else {
            self.ascii_art = Option::None;
        }
        self.background = Some(generate_background_randomly());
    }
    
    pub fn disappear(&mut self){
        if let Some(aa) = &mut self.ascii_art {
            aa.disapper();
        }
    }

    

    pub fn update_telop(&mut self, text: &str, pos: usize) -> &Char2DArray {
        let mut rng = thread_rng();
        let mut clear = Char2DArray::new(self.config.stage_wxh.0, self.telop_area.height());
        clear.overwrite_char_all(' ');
        self.telop_area.overwrite_rect(&clear, CharPosition{x:0,y:0}, Option::None);
        if let Ok(telop) = string2ascii(text, self.telop_area.height() as f32, '@', Some((pos, '-')), Some("./font/wqy-microhei/WenQuanYiMicroHei.ttf")) {
            self.telop_area.overwrite_rect(&telop, CharPosition{x:0, y:0}, Option::None);
            //self.telop_area = telop;
        } else {
            self.telop_area.overwrite_char_all(' ');
        }

        /*
        if pos >= 1 {
            let config = CharacterConfig {
                area_wxh : (self.pict_area.width(), self.pict_area.height()),
                min_size: self.pict_area.height() as f32 / 3.0,
                max_size: self.pict_area.height() as f32 / 1.0,
                chars: vec!['@','*','+','-'],
                duration_ms: 500,
                framerate: self.config.framerate,
            };
            self.typed_char = Some(Character::new(text.chars().nth(pos-1).unwrap(), config));
        }
        */
        &self.telop_area
    }

    pub fn update_character(&mut self, ch: char) {
        let config = CharacterConfig {
            area_wxh : (self.pict_area.width(), self.pict_area.height()),
            min_size: self.pict_area.height() as f32 / 3.0,
            max_size: self.pict_area.height() as f32 / 1.0,
            chars: vec!['@','*','+','-'],
            duration_ms: 500,
            framerate: self.config.framerate,
        };
        self.typed_char = Some(Character::new(ch, config));
    }

    pub fn update_subtelop(&mut self, text: &str, pos: usize) -> &Char2DArray {
        let mut clear = Char2DArray::new(self.config.stage_wxh.0, self.subtelop_area.height());
        clear.overwrite_char_all(' ');
        self.subtelop_area.overwrite_rect(&clear, CharPosition{x:0,y:0}, Option::None);
        if let Ok(telop) = string2ascii(text, self.subtelop_area.height() as f32, '@', Some((pos, '-')), Some("./font/wqy-microhei/WenQuanYiMicroHei.ttf")) {
            self.subtelop_area.overwrite_rect(&telop, CharPosition{x:0, y:0}, Option::None);
        } else {
            self.subtelop_area.overwrite_char_all(' ');
        }
        &self.subtelop_area
    }


    pub fn clear_pict(&mut self) -> &Char2DArray {
        if let Some(background) = &mut self.background {
            background.update(&mut self.pict_area);
        } else {
            self.pict_area.overwrite_char_all(' ');
        }
        &self.pict_area
    }

    pub fn update_pict(&mut self) -> &Char2DArray {
        if let Some(background) = &mut self.background {
            background.update(&mut self.pict_area);
        } else {
            self.pict_area.overwrite_char_all(' ');
        }


        if let Some(aa) = &mut self.ascii_art {
            match *aa.update() {
                AsciiArtState::APPEAR => {},
                AsciiArtState::DISAPPER => {},
                AsciiArtState::DISAPPERED => {},
                AsciiArtState::MOVE => {},
            }
            let pos = aa.get_position();
            let pos = CharPosition {
                x: pos.0,
                y: pos.1,
            };
            //self.pict_area.overwrite_rect_center(aa.get_data(), pos, Some('\u{0000}'));
            self.pict_area.overwrite_rect_center(aa.get_data(), pos, Some(' '));
            // ' 'は透過させてみる
        }
        
        if let Some(ch) = &mut self.typed_char {
            /*
            let ch_area = generate_character_randomly(ch, self.pict_area.height() as f32 / 4.0, self.pict_area.height() as f32 / 2.0, vec!['@','*','+','-']);
            let pos = CharPosition {
                x: rng.gen_range(0, self.pict_area.width() - ch_area.width()) as i32,
                y: rng.gen_range(0, self.pict_area.height() - ch_area.height()) as i32,
            };
            */
            match *ch.update() {
                CharacterStatus::APPEARED => {
                    self.pict_area.overwrite_rect(&ch.get_data(), ch.get_position(), Some(' '));
                },
                CharacterStatus::DISAPPEARED => {
                    self.typed_char = None;
                }
            }
        }
        &self.pict_area

    }

    pub fn has_typed_char(&self) -> bool {
        match self.typed_char {
            Some(_) => true,
            None => false,
        }
    }

}

#[test]
fn stage_new_works() {
    let config = StageConfig{
        stage_wxh: (600, 150),
        aa_width: 250,
        framerate: 10,

    };
    let mut stage = Stage::new(config);
    stage.appear("/Users/shizuku/drawings/001_megamouse_shark.png");
    
    stage.pict_area.debug_print();
    stage.update_telop("MEGAMOUSE SHARK", 0).debug_print();
    
    


    //stage.pict_area.debug_print();
    //stage.appear("/Users/shizuku/drawings/000_blackout.png");
    //stage.appear("/Users/shizuku/drawings/004_flathead.png");
    //stage.appear("/Users/shizuku/drawings/002_striped_loach.png");
    stage.update_pict().debug_print();
    stage.update_telop("MEGAMOUSE SHARK", 1).debug_print();


    for i in 0..10 {
        stage.update_pict().debug_print();
        stage.update_telop("MEGAMOUSE SHARK", 2).debug_print();
    }

    /*
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_telop("ABCDEFG", 3);
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.disappear();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.appear("/Users/shizuku/drawings/002_striped_loach.png");
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    stage.update_pict().debug_print();
    */        


}
