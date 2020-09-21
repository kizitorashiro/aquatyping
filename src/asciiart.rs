use image2ascii::{Char2DArray, image2ascii};
use super::effector::{Effector, generate_effector, EffectorType, EffectorStatus, generate_appear_effector_randomly, generate_disappear_effector_randomly};
use super::behavior::{Behavior, generate_behavior, BehaviorType, generate_behavior_randomly};

#[derive(Debug)]
pub enum AsciiArtState{
    APPEAR,
    MOVE,
    DISAPPER,
    DISAPPERED,
}

pub struct AsciiArt {
    //context: AsciiArtContext,
    aa_file: String,
    aa_original: Char2DArray,
    aa_data: Char2DArray,
    aa_pos: (i32, i32),
    state: AsciiArtState,
    current_frame: u32,
    appear_effector: Box<dyn Effector>,
    behavior: Box<dyn Behavior>,
    disappear_effector: Box<dyn Effector>,
}

pub struct AsciiArtContext {
    pub stage_wxh: (usize, usize),
    pub aa_width: usize,
    pub framerate: u32,
}

impl AsciiArt {
    pub fn from_image_easy(image_file: &str, context: &AsciiArtContext) -> Result<AsciiArt, String> {
        let original_data = image2ascii(&image_file, context.aa_width as u32, Option::None, Option::None).unwrap();
        let aa_wxh = (original_data.buffer[0].len(), original_data.buffer.len());
        let effector_duration = 1;
        let ret = AsciiArt {
            aa_file: String::from(image_file),
            aa_original: original_data,
            aa_data: Char2DArray::new(aa_wxh.0, aa_wxh.1),
            aa_pos: (0,0),
            state: AsciiArtState::APPEAR,
            current_frame: 0,
            appear_effector   : generate_appear_effector_randomly(effector_duration, context.framerate),
            behavior     : generate_behavior_randomly(context.framerate, context.stage_wxh, aa_wxh),
            disappear_effector: generate_disappear_effector_randomly(effector_duration, context.framerate),
        };
        Ok(ret)
    }

    pub fn from_image (image_file: &str, context: &AsciiArtContext, appear: EffectorType, disappear: EffectorType, behavior: BehaviorType) -> Result<AsciiArt, String> {
        let original_data = image2ascii(&image_file, context.aa_width as u32, Option::None, Option::None).unwrap();
        let aa_wxh = (original_data.buffer[0].len(), original_data.buffer.len());
        let effector_duration = 1;
        let ret = AsciiArt {
            aa_file: String::from(image_file),
            aa_original: original_data,
            aa_data: Char2DArray::new(aa_wxh.0, aa_wxh.1),
            aa_pos: (0,0),
            state: AsciiArtState::APPEAR,
            current_frame: 0,
            appear_effector   : generate_effector(appear, effector_duration, context.framerate),
            behavior     : generate_behavior(behavior, context.framerate, context.stage_wxh, aa_wxh),
            disappear_effector: generate_effector(disappear, effector_duration, context.framerate),
        };
        Ok(ret)
    }
    
    pub fn update(&mut self) -> &AsciiArtState{
        
        // add white noise

        match self.state {
            AsciiArtState::APPEAR => {
                match self.appear_effector.update(&mut self.aa_data, &self.aa_original) {
                    EffectorStatus::DOING => {},
                    EffectorStatus::COMPLETED => {
                        self.state = AsciiArtState::MOVE;
                    }
                }
                &self.state
            },
            AsciiArtState::MOVE => {
                self.aa_pos = self.behavior.update();
                &self.state
            },
            AsciiArtState::DISAPPER => {
                match self.disappear_effector.update(&mut self.aa_data, &self.aa_original) {
                    EffectorStatus::DOING => {},
                    EffectorStatus::COMPLETED => {
                        self.state = AsciiArtState::DISAPPERED;
                    }
                }
                &self.state
            },
            AsciiArtState::DISAPPERED => {
                &self.state
            }
        }
    }
    
    pub fn disapper(&mut self){
        self.state = AsciiArtState::DISAPPER;
    }
    
    pub fn get_state(&self) -> &AsciiArtState {
        &self.state
    }

    pub fn get_data(&self) -> &Char2DArray{
        &self.aa_data
    }

    pub fn get_position(&self) -> (i32, i32) {
        self.aa_pos
    }

}
/*
#[test]
fn from_image_works(){
    let context = AsciiArtContext {
        stage_wxh: (600, 120),
        aa_width: 600,
        framerate: 10,
            
    };
    let appear = EffectorType::NO;
    let mut aa = AsciiArt::from_image(&String::from(""), &context, EffectorType::NO, EffectorType::NO, BehaviorType::NO);
    
    
}
*/
/*
#[test]
fn test001(){
    let mut infos : HashMap<String, AsciiArtInfo> = HashMap::new();
    let mut names : HashMap<String, String> = HashMap::new();
    names.insert(String::from("ja"), String::from("チンアナゴ"));
    let data = AsciiArtInfo{
        id: String::from("001"),
        filename: String::from("001_test.png"),
        names: names,
        category: String::from("fish"),
    };
    infos.insert(String::from("001"), data);
    println!("{}", serde_yaml::to_string(&infos).unwrap());

}
*/

/*
#[test]
fn test001(){
    test();
}
*/
/*
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct AsciiArtInfo {
    id: String,
    filename: String,
    ja: String, 
    en: String, 
    category: String,
}
*/

/*
pub fn test(){
    let a = AsciiArtInfo {
        id: String::from("A"),
        filename: String::from("A.png"),
        ja: String::from("あなご"),
        en: String::from("garden lee"),
        category: String::from("fish"),
    };
    let b = AsciiArtInfo {
        id: String::from("B"),
        filename: String::from("B.png"),
        ja: String::from("メガマウス"),
        en: String::from("garden lee"),
        category: String::from("fish"),
    };
    
    let mut m : HashMap<String, AsciiArtInfo> = HashMap::new();
    m.insert(String::from("A"), a);
    m.insert(String::from("B"), b);
    let s = serde_yaml::to_string(&m).unwrap();
    println!("{}", s);
    let  x : HashMap<String, AsciiArtInfo> = serde_yaml::from_str(&s).unwrap();

    if let Some(info) = x.get(&String::from("A")) {
        println!("{}", info.en);
    }


}
*/
