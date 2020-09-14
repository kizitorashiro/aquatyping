use rand::{thread_rng, Rng};

pub trait Behavior {
    fn update(&mut self) -> (i32, i32);
}

#[derive(Debug)]
pub enum BehaviorType {
    NO,
    UPANDDOWN,
}

pub fn generate_behavior_randomly(framerate: u32, stage_wxh: (usize, usize), aa_wxh: (usize, usize)) -> Box<dyn Behavior> {
    generate_behavior(BehaviorType::UPANDDOWN, framerate, stage_wxh, aa_wxh)
}

pub fn generate_behavior(behavior_type: BehaviorType, framerate: u32, stage_wxh: (usize, usize), aa_wxh: (usize, usize)) -> Box<dyn Behavior> {
    match behavior_type {
        BehaviorType::NO => {
            Box::new(NoBehavior::new())
        },
        BehaviorType::UPANDDOWN => {
            Box::new(UpAndDownBehavior::new(framerate, stage_wxh, aa_wxh))
        }
        _ => panic!("not found effector: {:?}", behavior_type),
    }
}

struct NoBehavior{
}

impl NoBehavior {
    fn new() -> NoBehavior{
        NoBehavior {
        }
    }
}

impl Behavior for NoBehavior{
    fn update(&mut self) -> (i32, i32){
        (0, 0)
    }
}

#[derive(Debug)]
struct UpAndDownBehavior {
    framerate: u32,
    current_frame: u32,
    stage_wxh: (usize, usize),
    aa_wxh: (usize, usize),
    h_period: f32, // 水平方向の周期
    v_period: f32, // 垂直方向の周期
    h_amp: f32, // 水平方向の振幅 (stage_whx - aa_wxh) / 2　に対する割合
    v_amp: f32, // 垂直方向の振幅 (stage_whx - aa_wxh) / 2　に対する割合
}

impl UpAndDownBehavior {
    // ランダムで振動数と振幅
    fn new(framerate: u32, stage_wxh: (usize, usize), aa_wxh: (usize, usize)) -> UpAndDownBehavior{
        let mut rng = thread_rng();
        let h_period: f32 =  rng.gen_range(2.0, 7.0); // 1.0-4.0
        let v_period: f32 =  rng.gen_range(2.0, 7.0); // 1.0-4.0
        let h_amp: f32 = rng.gen_range(0.5, 0.6);
        let v_amp: f32 = rng.gen_range(0.5, 0.6);
        UpAndDownBehavior {
            framerate: framerate,
            current_frame: 0,
            stage_wxh: stage_wxh,
            aa_wxh: aa_wxh,
            h_period: h_period,
            v_period: v_period,
            h_amp: h_amp,
            v_amp: v_amp,
        }
    }
}

impl Behavior for UpAndDownBehavior {
    fn update(&mut self) -> (i32, i32) {
        let t = self.current_frame as f32 / self.framerate as f32;
        
        // horizontal
        let h_amp = (self.stage_wxh.0 - self.aa_wxh.0) as f32 * self.h_amp;
        let h_w = 2.0 * 3.14 * (1.0 / self.h_period);
        let x = - (h_w * t).sin() * h_amp;
        
        // vertical
        let v_amp = (self.stage_wxh.1 - self.aa_wxh.1) as f32 * self.v_amp;
        let v_w = 2.0 * 3.14 * (1.0 / self.v_period);
        let y = - (v_w * t).sin() * v_amp;

        self.current_frame += 1;
        (x as i32, y as i32)
    }
}

#[test]
fn upanddownbehavior_works() {
    let stage_wxh = (600, 120);
    let aa_wxh = (300, 80);
    let mut behavior = generate_behavior(BehaviorType::UPANDDOWN, 10, stage_wxh, aa_wxh);
    let pos = behavior.update();
    assert_eq!(pos, (0, 0));
    let pos = behavior.update();
}