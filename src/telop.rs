use super::image2ascii::{Char2DArray, string2ascii};

struct Telop {
    text: String, 
    size: f32,
    pos: usize,
    ch: char,
    ch2nd: char,
    data: Char2DArray,
}

impl Telop {

    fn new(text: &str, size: f32, ch: char, ch2nd: char) -> Self {
        Telop {
            text: String::from(text),
            size: size,
            pos: 0,
            ch: ch,
            ch2nd: ch2nd,
            data: string2ascii(text, size, ch, Some((0, ch2nd)), Option::None).unwrap(),
        }
    }
    fn forward(&mut self) -> &Char2DArray{
        self.pos += 1;
        self.data = string2ascii(&self.text, self.size, self.ch, Some((self.pos, self.ch2nd)), Option::None).unwrap();
        self.get_data()
    }
    fn get_data(&self) -> &Char2DArray {
        &self.data
    }
}

