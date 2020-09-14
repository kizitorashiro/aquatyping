
use rand::{thread_rng, Rng};
use rand::rngs::ThreadRng;
use std::collections::HashSet;
use super::image2ascii::{Char2DArray};


pub trait Background {
    fn update(&mut self, stage: &mut Char2DArray);
}

pub enum BackgroundType {
    RANDOM,
    NONE,
}

pub fn generate_background_randomly() -> Box<dyn Background> {
    let mut rng = thread_rng();
    let index: u32 = rng.gen_range(0, 2);
    let bgtype = match index {
        // 0 => BackgroundType::RANDOM,
        // 1 => BackgroundType::NONE,
        _ => BackgroundType::RANDOM,
    };
    generate_background(bgtype)
}

pub fn generate_background(background_type: BackgroundType) -> Box<dyn Background> {
    match background_type {
        BackgroundType::RANDOM => {
            let chars = vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '`', '.'];
            Box::new(RandomBackground::new(chars))
        },
        BackgroundType::NONE => {
            Box::new(NoneBackground{})
        }
    }
}

struct RandomBackground {
    rng: ThreadRng,
    chars: Vec<char>,
}

impl RandomBackground {
    pub fn new(chars: Vec<char>) -> Self{
        RandomBackground {
            rng: thread_rng(),
            chars: chars,
        }
    }
}

impl Background for RandomBackground {
    fn update(&mut self, stage: &mut Char2DArray) {
        for y in 0..stage.height() {
            for x in 0..stage.width() {
                let index: usize = self.rng.gen_range(0, self.chars.len());
                stage.buffer[y][x] = self.chars[index];
            }
        }
    }
}

struct NoneBackground {}
impl Background for NoneBackground {
    fn update(&mut self, stage: &mut Char2DArray){
        stage.overwrite_char_all(' ');
    }
}

#[test]
fn randombackground_works() {
    let mut bg = generate_background(BackgroundType::RANDOM);
    let mut stage = Char2DArray::new(20, 10);
    stage.overwrite_char_all('@');
    let mut chars = HashSet::new();
    chars.insert(' ');
    chars.insert('`');
    chars.insert('.');

    bg.update(&mut stage);
    for y in 0..stage.height() {
        for x in 0..stage.width() {
            assert!(chars.get(&stage.buffer[y][x]).is_some());
        }
    }
}

#[test]
fn nonebackground_works() {
    let mut bg = generate_background(BackgroundType::NONE);
    let mut stage = Char2DArray::new(20, 10);
    stage.overwrite_char_all('@');
    bg.update(&mut stage);
    for y in 0..stage.height() {
        for x in 0..stage.width() {
            assert_eq!(stage.buffer[y][x], ' ');
        }
    }
}
