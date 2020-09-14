use std::thread;
use std::sync::Arc;
use std::sync::Mutex;
use std::io::{stdin, stdout, Write, Stdout};
//use tuikit::prelude::*;
//use rustbox::{Color, RustBox, Key};
//use rustbox::Event::{KeyEvent};
use termion::{clear};
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::color::Rgb;

use crossbeam_channel as channel;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub enum RenderColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Byte(u8, u8, u8),
    Default,
}

#[derive(Debug,Eq, PartialEq, Copy, Clone)]
pub enum UIKeyEvent {
    Char(char),
    ESC,
    Enter,
    Up,
    Down,
    Left,
    Right,
    Tab,
    Others,
}

pub trait UIGraphics: Send {
    //fn draw(&self, x: usize, y: usize, color: &RenderColor, bgcolor: &RenderColor, text: &str);
    fn draw_area(&self, color: &RenderColor, bgcolor: &RenderColor, rect: &Vec<Vec<char>>, offset: Option<(usize, usize)>);
    fn flush(&self);
}


pub trait UIFuncs {
    fn start_keyevent_thread(&mut self) -> channel::Receiver<UIKeyEvent>;
    fn get_graphics(&self) -> Box<dyn UIGraphics>;
}

pub enum UIFuncsType {
    DEBUG,
    TUI,
}
pub fn generate_uifuncs(uifuncs_type: UIFuncsType) -> Box<dyn UIFuncs> {
    match uifuncs_type {
        UIFuncsType::DEBUG => {
            Box::new(DebugUIFuncs::new())
        },
        UIFuncsType::TUI => {
            Box::new(TUIFuncs::new())
        }
    }
}

struct DebugGraphics {}
pub struct DebugUIFuncs {
    keyevent_thread: Option<thread::JoinHandle<()>>,
    dummy_tx: Option<channel::Sender::<UIKeyEvent>>,
}

impl UIGraphics for DebugGraphics {
    fn draw_area(&self, color: &RenderColor, bgcolor: &RenderColor, rect: &Vec<Vec<char>>, offset: Option<(usize, usize)>) {
        println!("draw_area -> fg: {:?} bg: {:?} (w: {}, h: {})", color, bgcolor, rect[0].len(), rect.len());
    }
    
    fn flush(&self) {
        println!("flush");
    }     
}

impl DebugUIFuncs {

    fn new() -> Self {
        DebugUIFuncs {
            keyevent_thread: Option::None,
            dummy_tx: Option::None,
        }
    }
    fn dummy_keyevent(&self, keyevent: UIKeyEvent) {
        if let Some(tx) = &self.dummy_tx {
            tx.send(keyevent).unwrap();
        }
        //self.dummy_tx.unwrap().send(keyevent).unwrap();
    }
}

impl UIFuncs for DebugUIFuncs {
    fn start_keyevent_thread(&mut self) -> channel::Receiver<UIKeyEvent>{
        let (chan_tx, chan_rx) = channel::unbounded::<UIKeyEvent>();
        let (dummy_tx, dummy_rx) = channel::unbounded::<UIKeyEvent>();
        self.dummy_tx = Some(dummy_tx);

        let th = thread::spawn(move || {
            loop {
                channel::select! {
                    recv(dummy_rx) -> received => {
                        match received {
                            Ok(keyevent) => {
                                match keyevent {
                                    UIKeyEvent::ESC => { break; },
                                    _ => {
                                        if let Err(_) = chan_tx.send(keyevent) {
                                        break;
                                        }
                                    }
                                }
                            },
                            Err(_) => {
                                break;
                            }
                        }
                    }
                }
            }
        });

        self.keyevent_thread = Some(th);
        chan_rx
    }
    fn get_graphics(&self) -> Box<dyn UIGraphics> {
        Box::new(DebugGraphics{})
    }

}
pub struct TUIGraphics {
}

impl TUIGraphics {
    fn draw(&self, stdout: &mut Stdout, x: usize, y: usize, color: &RenderColor, bgcolor: &RenderColor, text: &str){
        write!(stdout, "{}{}{}", 
            termion::cursor::Goto(x as u16, y as u16), 
            &TUIFuncs::to_termion_color(color),
            //&TUIFuncs::to_termion_bgcolor(bgcolor),
            //termion::color::Fg(termion::color::Rgb(0,0,255)),
            //termion::color::Fg(termion::color::Blue),
            //termion::color::Bg(TUIFuncs::to_termion_color(bgcolor)),
            text
        ).unwrap();
    }
}

impl UIGraphics for TUIGraphics { 

    fn draw_area(&self, color: &RenderColor, bgcolor: &RenderColor, area: &Vec<Vec<char>>, offset: Option<(usize,usize)>) {
        let mut stdout = stdout().into_raw_mode().unwrap();

        let (offset_x, offset_y) = offset.unwrap_or((0, 0));

        for (i, line) in area.iter().enumerate() {
            let mut line_string = String::new();
            for ch in line {
                line_string.push(*ch);
            }
            self.draw(&mut stdout, offset_x, offset_y + i, color, bgcolor, &line_string);
        }
        stdout.flush().unwrap();
    }

    fn flush(&self) {

    }

}

pub struct TUIFuncs {
    keyevent_thread: Option<thread::JoinHandle<()>>,
}

impl TUIFuncs {
    pub fn new() -> TUIFuncs {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}", termion::clear::All, termion::cursor::Hide);
        stdout.flush().unwrap();
        TUIFuncs {
            keyevent_thread: Option::None,
        }
    }

    fn to_termion_color(color: &RenderColor) -> String { //termion::color::Rgb {

        match color {
            RenderColor::Black =>   { format!("{}", termion::color::Fg(termion::color::Black)) },
            RenderColor::Red =>     { format!("{}", termion::color::Fg(termion::color::Red)) },
            RenderColor::Green =>   { format!("{}", termion::color::Fg(termion::color::Green)) },
            RenderColor::Yellow =>  { format!("{}", termion::color::Fg(termion::color::Yellow)) },
            RenderColor::Blue =>    { format!("{}", termion::color::Fg(termion::color::Blue)) },
            RenderColor::Magenta => { format!("{}", termion::color::Fg(termion::color::Magenta)) },
            RenderColor::Cyan =>    { format!("{}", termion::color::Fg(termion::color::Cyan)) },
            RenderColor::White =>   { format!("{}", termion::color::Fg(termion::color::White)) },
            RenderColor::Byte(r,g,b) => { format!("{}", termion::color::Fg(termion::color::Rgb(*r,*g,*b))) },
            RenderColor::Default => { format!("{}", termion::color::Fg(termion::color::Black)) },
            _ => { format!("{}", termion::color::Fg(termion::color::Black)) },
        }

    }

    fn to_termion_bgcolor(color: &RenderColor) -> String { //termion::color::Rgb {

        match color {
            RenderColor::Black =>   { format!("{}", termion::color::Bg(termion::color::Black)) },
            RenderColor::Red =>     { format!("{}", termion::color::Bg(termion::color::Red)) },
            RenderColor::Green =>   { format!("{}", termion::color::Bg(termion::color::Green)) },
            RenderColor::Yellow =>  { format!("{}", termion::color::Bg(termion::color::Yellow)) },
            RenderColor::Blue =>    { format!("{}", termion::color::Bg(termion::color::Blue)) },
            RenderColor::Magenta => { format!("{}", termion::color::Bg(termion::color::Magenta)) },
            RenderColor::Cyan =>    { format!("{}", termion::color::Bg(termion::color::Cyan)) },
            RenderColor::White =>   { format!("{}", termion::color::Bg(termion::color::White)) },
            RenderColor::Byte(r,g,b) => { format!("{}", termion::color::Bg(termion::color::Rgb(*r,*g,*b))) },
            RenderColor::Default => { format!("{}", termion::color::Bg(termion::color::Black)) },
            _ => { format!("{}", termion::color::Bg(termion::color::Black)) },
        }

    }
}

impl UIFuncs for TUIFuncs {

    fn get_graphics(&self) -> Box<dyn UIGraphics> {
        let graphics = TUIGraphics {
        };
        Box::new(graphics)
    }


    fn start_keyevent_thread(&mut self) -> channel::Receiver<UIKeyEvent> {
        let (chan_tx, chan_rx) = channel::unbounded::<UIKeyEvent>();
        let th = thread::spawn(move || {
            //while let Ok(event) = term.lock().unwrap().poll_event(false) {
            let stdin = stdin();
            for c in stdin.keys() {
                match c.unwrap() {
                    Key::Char(ch) => {
                        println!("{}", ch);
                        if let Err(_) = chan_tx.send(UIKeyEvent::Char(ch)) {
                            break;
                        }
                    },
                    Key::Esc => { break; },
                    _ => {},
                }
            }
        });
        self.keyevent_thread = Some(th);
        chan_rx
    }
}

/*
pub struct TUIGraphics {
    term: Arc<Mutex<RustBox>>,
}

impl UIGraphics for TUIGraphics { 
    fn draw(&self, x: usize, y: usize, color: &RenderColor, bgcolor: &RenderColor, text: &str){
        self.term.lock().unwrap().print(y, x, rustbox::RB_NORMAL, TUIFuncs::to_tuikit_color(color), TUIFuncs::to_tuikit_color(bgcolor), text);
    }

    fn draw_area(&self, color: &RenderColor, bgcolor: &RenderColor, area: &Vec<Vec<char>>, offset: Option<(usize,usize)>) {
        let (offset_x, offset_y) = offset.unwrap_or((0, 0));

        //self.term.clear(); self.term.present();
        for (i, line) in area.iter().enumerate() {
            let mut line_string = String::new();
            for ch in line {
                line_string.push(*ch);
            }
            
            //self.draw(0, i, color, bgcolor, &line_string);
            //println!("{:?} {:?}", offset_x, offset_y + i);
            self.draw(offset_x, offset_y + i, color, bgcolor, &line_string);
        }
    }

    fn flush(&self) {
        self.term.lock().unwrap().present();
    }

}

pub struct TUIFuncs {
    term: Arc<Mutex<RustBox>>,
    keyevent_thread: Option<thread::JoinHandle<()>>,
}

impl TUIFuncs {
    pub fn new() -> TUIFuncs {
        let rustbox = RustBox::init(Default::default()).unwrap();
        let term = Arc::new(Mutex::new(rustbox));
        TUIFuncs {
            term: term,
            keyevent_thread: Option::None,
        }
    }

    fn to_uifuncs_keyevent(event: rustbox::Event) -> UIKeyEvent {
        match event {
            rustbox::Event::KeyEvent(key) => {
                match key {
                    Key::Char(ch) => UIKeyEvent::Char(ch),
                    Key::Esc => UIKeyEvent::ESC,
                    Key::Enter => UIKeyEvent::Enter,
                    _ => UIKeyEvent::Others,
                }
            },
            _ => UIKeyEvent::Others,
        }

        /*
        match tuikit_event {
            Event::Key(tuikit_keyevent) => {
                match tuikit_keyevent {
                    Key::Char(key) => UIKeyEvent::Char(key),
                    Key::ESC => UIKeyEvent::ESC,
                    Key::Enter => UIKeyEvent::Enter,
                    Key::Up => UIKeyEvent::Up,
                    Key::Down => UIKeyEvent::Down,
                    Key::Left => UIKeyEvent::Left,
                    Key::Right => UIKeyEvent::Right,
                    Key::Tab => UIKeyEvent::Tab,        
                    _ => UIKeyEvent::Others,
                }
            },
            _ => UIKeyEvent::Others,
        }
        */
    }

    fn to_tuikit_color(color: &RenderColor) -> Color {
        match color {
            RenderColor::Black =>   { Color::Black },
            RenderColor::Red =>     { Color::Red },
            RenderColor::Green =>   { Color::Green },
            RenderColor::Yellow =>  { Color::Yellow },
            RenderColor::Blue =>    { Color::Blue },
            RenderColor::Magenta => { Color::Magenta },
            RenderColor::Cyan =>    { Color::Cyan },
            RenderColor::White =>   { Color::White} ,
            RenderColor::Default => { Color::Default},
            _ => { Color::Black },
        }
    }

}

impl UIFuncs for TUIFuncs {

    fn get_graphics(&self) -> Box<dyn UIGraphics> {
        let term = self.term.clone();
        let graphics = TUIGraphics {
            term: term,
        };
        Box::new(graphics)
    }


    fn start_keyevent_thread(&mut self) -> channel::Receiver<UIKeyEvent> {
        let (chan_tx, chan_rx) = channel::unbounded::<UIKeyEvent>();
        let term = self.term.clone();
        let th = thread::spawn(move || {
            //while let Ok(event) = term.lock().unwrap().poll_event(false) {
            loop {
                let pe = term.lock().unwrap().poll_event(false);
                if let Ok(event) = pe {
                    let uifuncs_keyevent = TUIFuncs::to_uifuncs_keyevent(event);
                    match uifuncs_keyevent {
                        UIKeyEvent::ESC => { break; },
                        _ => {
                        
                            if let Err(_) = chan_tx.send(uifuncs_keyevent) {
                                break;
                            }
                        
                        }
                    }
                } else {
                    break;
                }
            
            }
        });
        self.keyevent_thread = Some(th);
        chan_rx
    }
}
*/

/*
pub struct TUIGraphics {
    term: Arc<Term>,
}

impl UIGraphics for TUIGraphics { 
    fn draw(&self, x: usize, y: usize, color: &RenderColor, bgcolor: &RenderColor, text: &str){
        let attr = Attr {
            fg: TUIFuncs::to_tuikit_color(color),
            bg: TUIFuncs::to_tuikit_color(bgcolor),
            effect: Effect::UNDERLINE,
        };
        self.term.print_with_attr(y, x, text, attr);
    }

    fn draw_area(&self, color: &RenderColor, bgcolor: &RenderColor, area: &Vec<Vec<char>>, offset: Option<(usize,usize)>) {
        let (offset_x, offset_y) = offset.unwrap_or((0, 0));

        //self.term.clear(); self.term.present();
        for (i, line) in area.iter().enumerate() {
            let mut line_string = String::new();
            for ch in line {
                line_string.push(*ch);
            }
            
            //self.draw(0, i, color, bgcolor, &line_string);
            //println!("{:?} {:?}", offset_x, offset_y + i);
            self.draw(offset_x, offset_y + i, color, bgcolor, &line_string);
        }
    }

    fn flush(&self) {
        self.term.present();
    }

}

pub struct TUIFuncs {
    term: Arc<Term>,
    keyevent_thread: Option<thread::JoinHandle<()>>,
}

impl  TUIFuncs {
    pub fn new() -> TUIFuncs {
        let term = Arc::new(Term::new().unwrap());
        TUIFuncs {
            term: term,
            keyevent_thread: Option::None,
        }
    }

    fn to_uifuncs_keyevent(tuikit_event: Event) -> UIKeyEvent {
        match tuikit_event {
            Event::Key(tuikit_keyevent) => {
                match tuikit_keyevent {
                    Key::Char(key) => UIKeyEvent::Char(key),
                    Key::ESC => UIKeyEvent::ESC,
                    Key::Enter => UIKeyEvent::Enter,
                    Key::Up => UIKeyEvent::Up,
                    Key::Down => UIKeyEvent::Down,
                    Key::Left => UIKeyEvent::Left,
                    Key::Right => UIKeyEvent::Right,
                    Key::Tab => UIKeyEvent::Tab,        
                    _ => UIKeyEvent::Others,
                }
            },
            _ => UIKeyEvent::Others,
        }
    }

    fn to_tuikit_color(color: &RenderColor) -> tuikit::prelude::Color { 
        match color {
            RenderColor::Black =>   { tuikit::prelude::Color::BLACK },
            RenderColor::Red =>     { tuikit::prelude::Color::RED },
            RenderColor::Green =>   { tuikit::prelude::Color::GREEN },
            RenderColor::Yellow =>  { tuikit::prelude::Color::YELLOW },
            RenderColor::Blue =>    { tuikit::prelude::Color::BLUE },
            RenderColor::Magenta => { tuikit::prelude::Color::MAGENTA },
            RenderColor::Cyan =>    { tuikit::prelude::Color::CYAN },
            RenderColor::White =>   { tuikit::prelude::Color::WHITE} ,
            RenderColor::Default => { tuikit::prelude::Color::Default},
            RenderColor::Byte(r,g,b) => { tuikit::prelude::Color::Rgb(*r,*g,*b) },
            _ => { tuikit::prelude::Color::BLACK },
        }
    }

}

impl UIFuncs for TUIFuncs {

    fn get_graphics(&self) -> Box<dyn UIGraphics> {
        let term = self.term.clone();
        let graphics = TUIGraphics {
            term: term,
        };
        Box::new(graphics)
    }


    fn start_keyevent_thread(&mut self) -> channel::Receiver<UIKeyEvent> {
        let (chan_tx, chan_rx) = channel::unbounded::<UIKeyEvent>();
        let term = self.term.clone();
        let th = thread::spawn(move || {
            while let Ok(event) = term.poll_event() {
                
                let uifuncs_keyevent = TUIFuncs::to_uifuncs_keyevent(event);
                match uifuncs_keyevent {
                    UIKeyEvent::ESC => { break; },
                    _ => {
                    
                        if let Err(_) = chan_tx.send(uifuncs_keyevent) {
                            break;
                        }
                    
                    }
                }
                
            }
        });
        self.keyevent_thread = Some(th);
        chan_rx
    }
}
*/
/*
#[test]
fn debuguifuncs_works() {
    let mut debugfuncs = DebugUIFuncs::new();
    let mut graphics = debugfuncs.get_graphics();
    let chan_rx = debugfuncs.start_keyevent_thread();

    let events = vec![
        UIKeyEvent::Char('a'),
        UIKeyEvent::Char('b'),
        UIKeyEvent::ESC,
    ];

    for event in events {
        debugfuncs.dummy_keyevent(event);
        channel::select! {
            recv(chan_rx) -> received => {
                match received {
                    Ok(keyevent) => {
                        println!("recv {:?}", keyevent);
                    },
                    Err(_) => {
                        println!("chan closed");
                    }
                }
            }
        }
    }


}
*/
/*
#[test]
fn tuifuncs_works() {
    let mut tuifuncs = TUIFuncs::new();
    let ch_rx = tuifuncs.start_keyevent_thread();
    let mut i = 0;
    let graphics = tuifuncs.get_graphics();
    let th = thread::spawn(move || {
        graphics.draw(0, 0, &RenderColor::Black, &RenderColor::White, "draw thread");
        graphics.draw(0, 1, &RenderColor::Black, &RenderColor::White, "second line");
        graphics.draw(1, 2, &RenderColor::Black, &RenderColor::White, "third line");
        
        graphics.flush();
    });

    loop {
        channel::select! {
            recv(ch_rx) -> received => {
                match received {
                    Ok(keyevent) => {
                        match keyevent {
                            UIKeyEvent::ESC => {
                                panic!("hello");
                                //break;
                                //tuifuncs.draw(0, 0, RenderColor::Blue, RenderColor::White, &ch.to_string());
                                //tuifuncs.flush();
                            },
                            _ => {
                                //tuifuncs.draw(0, 0, RenderColor::Green, RenderColor::White, &format!("{}", i));
                                i += 1;
                                //tuifuncs.flush();
                            }
                        }
                    },
                    _ => {
                        break;
                        //tuifuncs.draw(0, 0, RenderColor::Yellow, RenderColor::White, "receive error");
                        //tuifuncs.flush();
                    }
                }
            },
            default(Duration::from_millis(10000)) => {
                //break;
                //tuifuncs.draw(0, 0, RenderColor::Red, RenderColor::White, "1sec elapsed");
                //tuifuncs.flush();
            }
        }
    }
    // tuifuncs.keyevent_thread.unwrap().join();
}
*/
/*

#[test]
fn DebugRenderer_works() {
    let renderer = DebugRenderer{};
    renderer.draw(10, 20, RenderColor::Black, RenderColor::White, "hello");
    renderer.flush();
}

#[test]
fn tuirenderer_works() {
    let term = Term::new().unwrap();
    let renderer = TUIRenderer::new(&term);
    renderer.draw(0, 0, RenderColor::Blue, RenderColor::Yellow, "hello");
    renderer.flush();

    //let renderer = generate_renderer(RendererType::TUI);
    //renderer.draw(0, 0, Color::Blue, Color::Yellow, "rustbox");
    //renderer.flush();

}
*/