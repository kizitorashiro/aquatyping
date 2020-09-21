use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::io::BufReader;
use rand::{thread_rng, Rng};


#[derive(Serialize, Deserialize, Debug)]
pub struct Pict {
    pub id: String,
    pub ja: String,
    pub romaji: String,
    pub en: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PictInfo {
    picts: Vec<Pict>,
    titles: Vec<Pict>,
}

/*
pub enum PictIterType {
    RANDOM,
    SEQUENTIAL,
}
*/
/*
pub struct PictIter<'a>{
    pict_info: &'a PictInfo,
    index_series: Vec<usize>,
}
*/

/*
impl<'a> Iterator for PictIter<'a> {
    type Item = &'a Pict;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index_series.pop();
        match index {
            Some(i) => self.pict_info.picts.get(i),
            None => None,
        }
    }
}
*/

pub struct PictManager {
    pict_dir: String,
    pict_info: PictInfo,
}

impl PictManager {
    /*
    pub fn iter(&self, iter_type: PictIterType, size: usize) -> PictIter{
        let mut index_series = 
            match iter_type {
                PictIterType::SEQUENTIAL => generate_index_series(self.pict_info.picts.len(), size),
                PictIterType::RANDOM => generate_random_index_series(self.pict_info.picts.len(), size),
            };
        index_series.reverse();
        PictIter {
            pict_info: &self.pict_info,
            index_series: index_series,
        }
    }
    */
    pub fn index_series(&self, size: usize) -> Vec<usize> {
        //let mut index_series = generate_random_index_series(self.pict_info.picts.len(), size);
        let mut index_series = generate_index_series(self.pict_info.picts.len(), size);
        
        //let mut n = index_series.split_off(67);
        //n.reverse();
        //n

        index_series.reverse();
        index_series
        
    }

    pub fn get_pict(&self, index: usize) -> Option<&Pict> {
        self.pict_info.picts.get(index)
    }

    pub fn get_title_by_id(&self, id: &str) -> Option<&Pict> {
        for title in &self.pict_info.titles {
            if title.id == id {
                return Some(&title);
            }
        }
        None
    }
    
    pub fn get_pict_path(&self, pict: &Pict) -> String {
        let path_buf = Path::new(&self.pict_dir);
        let path_buf = path_buf.join(&pict.id);
        String::from(path_buf.to_str().unwrap())
    }

    pub fn get_pict_len(&self) -> usize {
        self.pict_info.picts.len()
    }

    pub fn new(pict_dir: &str) -> Self{
        
        let pict_dir = String::from(pict_dir);
        let filepath = Path::new(&pict_dir);
        let pict_json = filepath.join("picts_info.json");

        let reader = BufReader::new(File::open(pict_json).unwrap());
        let pict_info: PictInfo = serde_json::from_reader(reader).unwrap();

        PictManager{
            pict_dir: pict_dir,
            pict_info: pict_info,
        }
    }
}

fn generate_index_series(array_size: usize, series_size: usize) -> Vec<usize> {
    let mut index_series: Vec<usize> = Vec::new();
    loop {
        for i in 0..array_size {
            index_series.push(i);
            if index_series.len() >= series_size {
                return index_series;
            }
        }
    }
}

fn generate_random_index_series(array_size: usize, series_size: usize) -> Vec<usize> {
    let mut index_series: Vec<usize> = Vec::new();
    loop {
        let mut tmp: Vec<(u32, usize)> = Vec::new();
        for i in 0..array_size {
            let rnd: u32 = thread_rng().gen();
            tmp.push((rnd, i));
        }
        tmp.sort();
        for (_, index) in tmp.iter(){
            index_series.push(*index);
            if index_series.len() >= series_size {
                return index_series;
            }
        }
    }
}

/*
#[test]
fn pictmanager_works() {

    let manager = PictManager::new("/Users/shizuku/drawings");
    for pict in manager.iter(PictIterType::RANDOM, 10) {
        println!("{:?}", pict);
        println!("{}", manager.get_pict_path(pict));
    }
    assert_eq!(manager.get_pict_len(), 6);
}
*/


