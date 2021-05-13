use ron::de::from_reader;
use serde::Deserialize;

use legion::*;

use std::fs::File;
use std::collections::HashMap;

use super::space::{Point, Rect};
use super::draw::Draw;
use super::game::Timestep;

#[derive(Deserialize)]
pub struct SpriteAtlas
{

   frames: Vec<Rect>

}

impl SpriteAtlas
{

    pub fn from_file(file: &str) -> SpriteAtlas
    {

        let f = File::open(file).expect(&format!("Unable to open atlas file {}", file));

        match from_reader(f)
        {

           Ok(a) => return a,
           Err(e) => panic!("Failed to parse atlas RON file {} with error: {}", file, e)

        };

    }

    pub fn get_src(&self, frame: usize) -> &Rect
    {

        return &self.frames[frame];

    }

}

#[derive(Deserialize)]
struct Frame
{

    src: usize,
    time: i32,
    offset: Point

}

#[derive(Deserialize)]
pub struct Sheet
{

    animations: Vec<Vec<Frame>>

}

pub struct Sheets
{

    sheets: Vec<Sheet>,
    map: HashMap<String, usize>

}

impl Sheets
{

    pub fn new() -> Sheets
    {

        return Sheets { sheets: Vec::new(), map: HashMap::new() };

    }

    fn get_sheet(&mut self, file: &str, sheet_directory: &str) -> usize
    {

        //Check if the file has already been loaded
        match self.map.get(file)
        {

            Some(i) => return *i,
            None => {}

        }

        //It has not, so we need to load it
        let index = self.sheets.len();
        self.map.insert(file.to_string(), index);

        let f = File::open(&format!("{}{}.ron", sheet_directory, file)).expect(&format!("Unable to open spritesheet file {}", file));
        let parse: Result<Sheet, _>  = from_reader(f);

        match parse
        {

           Ok(s) => 
           {

                self.sheets.push(s);

                return index;

           },
           Err(e) => panic!("Failed to parse spritesheet RON file {} with error: {}", file, e)

        };  

        return index;

    }

}

pub struct SpriteSheet
{

    pub texture: usize,
    sheet: usize,
    current_animation: usize,
    current_frame: usize,
    running: bool,
    time: i32,
    repeat: bool

}

impl SpriteSheet
{

    fn new(texture: usize, sheet: usize) -> SpriteSheet
    {

        return SpriteSheet { texture: texture, sheet: sheet, current_animation: 0, current_frame: 0, running: false, time: 0, repeat: false };

    }

    pub fn from_files(draw: &mut Draw, sheets: &mut Sheets, texture_file: &str, image_directory: &str, atlas_directory: &str, sheet_file: &str, sheet_directory: &str) -> SpriteSheet
    {

        let texture = draw.get_texture(texture_file, image_directory, atlas_directory);
        let sheet = sheets.get_sheet(sheet_file, sheet_directory);

        return SpriteSheet::new(texture, sheet);

    }

    pub fn get_sheet<'a>(&self, sheets: &'a Sheets) -> &'a Sheet
    {

        return &sheets.sheets[self.sheet];

    }

    pub fn get_src<'a>(&self, draw: &'a Draw, sheet: &'a Sheet) -> &'a Rect
    {

        return draw.get_src(self.texture, sheet.animations[self.current_animation][self.current_frame].src);

    }

    pub fn get_offset<'a>(&self, sheet: &'a Sheet) -> &'a Point
    {

        return &sheet.animations[self.current_animation][self.current_frame].offset;

    }

    fn stop(&mut self)
    {

        self.current_animation = 0;
        self.current_frame = 0;

        self.time = 0;

        self.repeat = false;
        self.running = false;

    }

    pub fn play(&mut self, animation: usize, repeat: bool)
    {

        self.stop();

        self.current_animation = animation;
        self.repeat = repeat;

        self.running = true;

    }

    pub fn update(&mut self, step: i32, sheet: &Sheet)
    {

        if self.running
        {

            let current_time = sheet.animations[self.current_animation][self.current_frame].time;

            self.time += step;

            if self.time >= current_time
            {

                self.time -= current_time;

                self.current_frame += 1;

                if self.current_frame >= sheet.animations[self.current_animation].len()
                {

                    if self.repeat
                    {

                        self.current_frame = 0;

                    }
                    else
                    {

                        self.stop();

                    }

                }

            }

        }

    }

}

#[system(for_each)]
pub fn update_spritesheets(sheet: &mut SpriteSheet, #[resource] step: &Timestep, #[resource] sheets: &Sheets)
{

   sheet.update(step.step, &sheets.sheets[sheet.sheet]); 

}
