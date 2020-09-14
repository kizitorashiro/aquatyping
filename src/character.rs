use rand::{thread_rng, Rng};
use super::image2ascii::{Char2DArray, string2ascii, CharPosition};

fn generate_character(character: char, size: f32, ch: char) -> Char2DArray {
    let mut string = String::new();
    string.push(character);
    //let string = String::from_utf8(vec![character]);
    let mut c2d = string2ascii(&string, size, ch, Option::None, Option::None).unwrap();
    /*
    c2d.overwrite_fn('\u{0000}', |_, _, c| {
        c == ' '
    });
    */
    c2d
}

fn generate_character_randomly(character: char, min_size: f32, max_size: f32, chars: &Vec<char>) -> Char2DArray{
    let mut rng = thread_rng();
    let size = rng.gen_range(min_size, max_size);
    let index = rng.gen_range(0, chars.len());
    generate_character(character, size, chars[index])
}

pub struct Character {
    c2d: Char2DArray,
    pos: CharPosition,
    current_frame: u32,
    config: CharacterConfig,
    status: CharacterStatus,
}

pub struct CharacterConfig {
    pub area_wxh : (usize, usize),
    pub min_size: f32,
    pub max_size: f32,
    pub chars: Vec<char>,
    pub duration_ms: u32, // msec
    pub framerate: u32, 
}

pub enum CharacterStatus {
    APPEARED,    
    DISAPPEARED,
}

impl Character {
    pub fn new(ch: char, config: CharacterConfig) -> Self{
        let mut rng = thread_rng();
        let c2d = generate_character_randomly(ch, config.min_size, config.max_size, &config.chars); 
        let pos = CharPosition {
            x: rng.gen_range(0, 1 /*config.area_wxh.0 - c2d.width()*/) as i32,
            y: rng.gen_range(0, 1 /*config.area_wxh.1 - c2d.height()*/) as i32,
        };
        Character {
            c2d: c2d,
            pos: pos,
            current_frame: 0,
            config: config,
            status: CharacterStatus::APPEARED,
        }
    }
    pub fn update(&mut self) -> &CharacterStatus {
        let total_frame = ((self.config.duration_ms as f32 / 1000.0 as f32) * self.config.framerate as f32) as u32;
        self.current_frame += 1;
        if self.current_frame > total_frame {
            self.status = CharacterStatus::DISAPPEARED;
        }
        &self.status
    }

    pub fn get_data(&self) -> &Char2DArray {
        &self.c2d
    }

    pub fn get_position(&self) -> CharPosition {
        self.pos
    }
}