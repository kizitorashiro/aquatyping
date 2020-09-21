use rand::{thread_rng, Rng};
use image2ascii::{Char2DArray, CharPosition};

#[derive(Debug, PartialEq)]
pub enum EffectorType {
    NO,
    FADEIN(FadeDirection),
    FADEOUT(FadeDirection),
}

#[derive(Debug, PartialEq)]
pub enum EffectorStatus {
    DOING,
    COMPLETED,
}

const TRANSPARENT_CHAR: char = ' ';

pub trait Effector {
    fn update(&mut self, data: &mut Char2DArray, original_data: &Char2DArray) -> EffectorStatus;
}

pub fn generate_appear_effector_randomly(duration: u32, framerate: u32) -> Box<dyn Effector> {
    let mut rng = thread_rng();
    let direction:usize = rng.gen_range(0,4);
    let direction = match direction {
        0 => FadeDirection::UP,
        1 => FadeDirection::DOWN,
        2 => FadeDirection::LEFT,
        3 => FadeDirection::RIGHT,
        _ => FadeDirection::UP,
    };
    generate_effector(EffectorType::FADEIN(direction), duration, framerate)
}

pub fn generate_disappear_effector_randomly(duration: u32, framerate: u32) -> Box<dyn Effector> {
    let mut rng = thread_rng();
    let direction:usize = rng.gen_range(0,4);
    let direction = match direction {
        0 => FadeDirection::UP,
        1 => FadeDirection::DOWN,
        2 => FadeDirection::LEFT,
        3 => FadeDirection::RIGHT,
        _ => FadeDirection::UP,
    };
    generate_effector(EffectorType::FADEOUT(direction), duration, framerate)
}



pub fn generate_effector(effector_type: EffectorType, duration: u32, framerate: u32) -> Box<dyn Effector> {
    match effector_type {
        EffectorType::NO => {
            Box::new(NoEffector {
            })
        },
        EffectorType::FADEIN(direction) => {
            Box::new(FadeInEffector::new(direction, duration, framerate, FadeType::FADEIN))
        },
        EffectorType::FADEOUT(direction) => {
            Box::new(FadeInEffector::new(direction, duration, framerate, FadeType::FADEOUT))
        },
        _ => panic!("not found effector: {:?}", effector_type),
    }
}

struct NoEffector {
}

impl NoEffector {
    fn new() -> NoEffector{
        NoEffector {
        }
    }
}

impl Effector for NoEffector {
    fn update(&mut self, data: &mut Char2DArray, original_data: &Char2DArray) -> EffectorStatus {
        data.overwrite_rect(original_data, CharPosition{x:0, y:0}, Option::None);
        EffectorStatus::COMPLETED
    }
}

#[derive(Debug, PartialEq)]
pub enum FadeDirection{
    UP,
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Debug, PartialEq)]
enum FadeType {
    FADEIN,
    FADEOUT,
}

struct FadeInEffector {
    current_frame: u32,
    direction: FadeDirection,
    duration: u32, // sec
    framerate: u32, // frame per sec
    fade_type: FadeType,
}

impl FadeInEffector {
    fn new(direction: FadeDirection, duration: u32, framerate: u32, fade_type: FadeType) -> FadeInEffector {
        FadeInEffector{
            current_frame: 0,
            direction: direction,
            duration: duration,
            framerate: framerate,
            fade_type: fade_type,
        }
    }
}

impl Effector for FadeInEffector{
    fn update(&mut self, data: &mut Char2DArray, original_data: &Char2DArray) -> EffectorStatus {
        let total_frame = self.duration * self.framerate;
        let ratio: f32 = self.current_frame as f32 / total_frame as f32;
        data.copy_from(original_data);

        match self.direction {
            FadeDirection::DOWN => {
                let hidden_index = (original_data.height() as f32 * ratio) as usize;
                data.overwrite_fn(TRANSPARENT_CHAR, |_, y, _| {
                    match self.fade_type {
                        FadeType::FADEIN =>  (y >= hidden_index),
                        FadeType::FADEOUT => (y < hidden_index),
                    }
                });
            },
            FadeDirection::UP => {
                let hidden_index = original_data.height() - (original_data.height() as f32 * ratio) as usize;
                data.overwrite_fn(TRANSPARENT_CHAR, |_, y, _| {
                    match self.fade_type {
                        FadeType::FADEIN =>  (y < hidden_index),
                        FadeType::FADEOUT => (y >= hidden_index),
                    }
                });
            },
            FadeDirection::LEFT => {
                let hidden_index = (original_data.width() as f32 * ratio) as usize;
                data.overwrite_fn(TRANSPARENT_CHAR, |x, _, _| {
                    match self.fade_type {
                        FadeType::FADEIN => (x >= hidden_index),
                        FadeType::FADEOUT => (x < hidden_index),
                    }
                });
            },
            FadeDirection::RIGHT => {
                let hidden_index = original_data.width() - (original_data.width() as f32 * ratio) as usize;
                data.overwrite_fn(TRANSPARENT_CHAR, |x, _, _| {
                    match self.fade_type {
                        FadeType::FADEIN => (x < hidden_index),
                        FadeType::FADEOUT => (x >= hidden_index),
                    }
                });
            }
        }
        self.current_frame += 1;
        if self.current_frame > total_frame {
            EffectorStatus::COMPLETED
        } else {
            EffectorStatus::DOING
        }
    }
}


#[test]
fn generate_noeffector_works(){
    let mut noeffector = generate_effector(EffectorType::NO, 1, 10);
    let original_data = Char2DArray::from(vec![
        vec!['A', 'B', 'C'],
        vec!['X', 'Y', 'Z'],
    ]);

 
    let mut data = Char2DArray::new(3,2);

    let ret = noeffector.update(&mut data, &original_data);

    assert_eq!(ret, EffectorStatus::COMPLETED);

    assert_eq!(data.buffer[0], ['A', 'B', 'C']);
    assert_eq!(data.buffer[1], ['X', 'Y', 'Z']);

}


#[test]
fn fadeineffector_down_works(){
    let mut effector = generate_effector(EffectorType::FADEIN(FadeDirection::DOWN), 1, 10);
    let original_data = Char2DArray::from(vec![
        vec!['A', 'B', 'C', 'D'],
        vec!['E', 'F', 'G', 'H'],
        vec!['I', 'J', 'K', 'L'],
        vec!['M', 'N', 'O', 'P'],
        vec!['Q', 'R', 'S', 'T'],
        vec!['U', 'V', 'W', 'X'],
        vec!['Y', 'Z', 'a', 'b'],
        vec!['c', 'd', 'e', 'f'],
        vec!['g', 'h', 'i', 'j'],
        vec!['k', 'l', 'm', 'n'],
    ]);
    let mut data = Char2DArray::new(4, 10);
    let ret = effector.update(&mut data, &original_data);
    assert_eq!(ret, EffectorStatus::DOING);
    assert_eq!(data.buffer[0], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[1], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[2], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[3], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[4], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[5], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[6], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[7], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[8], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[9], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);

    let ret = effector.update(&mut data, &original_data);
    assert_eq!(ret, EffectorStatus::DOING);
    assert_eq!(data.buffer[0], ['A', 'B', 'C', 'D']);
    assert_eq!(data.buffer[1], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[2], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[3], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[4], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[5], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[6], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[7], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[8], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[9], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);

    let ret = effector.update(&mut data, &original_data);
    assert_eq!(ret, EffectorStatus::DOING);
    assert_eq!(data.buffer[0], ['A', 'B', 'C', 'D']);
    assert_eq!(data.buffer[1], ['E', 'F', 'G', 'H']);
    assert_eq!(data.buffer[2], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[3], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[4], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[5], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[6], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[7], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[8], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[9], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);

    let ret = effector.update(&mut data, &original_data);
    assert_eq!(ret, EffectorStatus::DOING);
    assert_eq!(data.buffer[0], ['A', 'B', 'C', 'D']);
    assert_eq!(data.buffer[1], ['E', 'F', 'G', 'H']);
    assert_eq!(data.buffer[2], ['I', 'J', 'K', 'L']);
    assert_eq!(data.buffer[3], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[4], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[5], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[6], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[7], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[8], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[9], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);

    let ret = effector.update(&mut data, &original_data);
    let ret = effector.update(&mut data, &original_data);
    let ret = effector.update(&mut data, &original_data);
    let ret = effector.update(&mut data, &original_data);
    let ret = effector.update(&mut data, &original_data);
    let ret = effector.update(&mut data, &original_data);
    assert_eq!(ret, EffectorStatus::DOING);
    let ret = effector.update(&mut data, &original_data);
    assert_eq!(ret, EffectorStatus::COMPLETED);
    assert_eq!(data.buffer[0], ['A', 'B', 'C', 'D']);
    assert_eq!(data.buffer[1], ['E', 'F', 'G', 'H']);
    assert_eq!(data.buffer[2], ['I', 'J', 'K', 'L']);
    assert_eq!(data.buffer[3], ['M', 'N', 'O', 'P']);
    assert_eq!(data.buffer[4], ['Q', 'R', 'S', 'T']);
    assert_eq!(data.buffer[5], ['U', 'V', 'W', 'X']);
    assert_eq!(data.buffer[6], ['Y', 'Z', 'a', 'b']);
    assert_eq!(data.buffer[7], ['c', 'd', 'e', 'f']);
    assert_eq!(data.buffer[8], ['g', 'h', 'i', 'j']);
    assert_eq!(data.buffer[9], ['k', 'l', 'm', 'n']);


}

#[test]
fn fadeineffector_up_works(){
    let mut effector = generate_effector(EffectorType::FADEIN(FadeDirection::UP), 1, 10);
    let original_data = Char2DArray::from(vec![
        vec!['A', 'B', 'C', 'D'],
        vec!['E', 'F', 'G', 'H'],
        vec!['I', 'J', 'K', 'L'],
        vec!['M', 'N', 'O', 'P'],
        vec!['Q', 'R', 'S', 'T'],
        vec!['U', 'V', 'W', 'X'],
        vec!['Y', 'Z', 'a', 'b'],
        vec!['c', 'd', 'e', 'f'],
        vec!['g', 'h', 'i', 'j'],
        vec!['k', 'l', 'm', 'n'],
    ]);
    let mut data = Char2DArray::new(4, 10);
    let ret = effector.update(&mut data, &original_data);
    assert_eq!(ret, EffectorStatus::DOING);
    assert_eq!(data.buffer[0], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[1], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[2], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[3], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[4], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[5], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[6], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[7], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[8], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[9], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);

    let ret = effector.update(&mut data, &original_data);
    assert_eq!(ret, EffectorStatus::DOING);
    assert_eq!(data.buffer[0], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[1], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[2], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[3], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[4], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[5], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[6], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[7], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[8], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[9], ['k', 'l', 'm', 'n']);

    let ret = effector.update(&mut data, &original_data);
    assert_eq!(ret, EffectorStatus::DOING);
    assert_eq!(data.buffer[0], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[1], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[2], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[3], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[4], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[5], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[6], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[7], [TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,TRANSPARENT_CHAR,]);
    assert_eq!(data.buffer[8], ['g', 'h', 'i', 'j']);
    assert_eq!(data.buffer[9], ['k', 'l', 'm', 'n']);

    let ret = effector.update(&mut data, &original_data);
    let ret = effector.update(&mut data, &original_data);
    let ret = effector.update(&mut data, &original_data);
    let ret = effector.update(&mut data, &original_data);
    let ret = effector.update(&mut data, &original_data);
    let ret = effector.update(&mut data, &original_data);
    let ret = effector.update(&mut data, &original_data);
    assert_eq!(ret, EffectorStatus::DOING);
    let ret = effector.update(&mut data, &original_data);
    assert_eq!(ret, EffectorStatus::COMPLETED);
    assert_eq!(data.buffer[0], ['A', 'B', 'C', 'D']);
    assert_eq!(data.buffer[1], ['E', 'F', 'G', 'H']);
    assert_eq!(data.buffer[2], ['I', 'J', 'K', 'L']);
    assert_eq!(data.buffer[3], ['M', 'N', 'O', 'P']);
    assert_eq!(data.buffer[4], ['Q', 'R', 'S', 'T']);
    assert_eq!(data.buffer[5], ['U', 'V', 'W', 'X']);
    assert_eq!(data.buffer[6], ['Y', 'Z', 'a', 'b']);
    assert_eq!(data.buffer[7], ['c', 'd', 'e', 'f']);
    assert_eq!(data.buffer[8], ['g', 'h', 'i', 'j']);
    assert_eq!(data.buffer[9], ['k', 'l', 'm', 'n']);


}
