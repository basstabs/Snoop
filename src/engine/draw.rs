use sfml::graphics::{Color, RectangleShape, Shape, Sprite, Texture, Transformable};
use sfml::system::Vector2f;
use sfml::SfBox;

use legion::*;

use std::collections::HashMap;

use super::space::Rect;
use super::sprites::{Sheet, SpriteAtlas};

pub struct Stroke
{

    color: Color,
    fill: Color,
    thickness: f32

}

impl Stroke
{

    pub fn new(color: Color, fill: Color, thickness: f32) -> Stroke
    {

        return Stroke { color, fill, thickness };

    }

}

pub struct Draw
{

    textures: Vec<(SfBox<Texture>, Option<SpriteAtlas>)>,
    map: HashMap<String, usize>

}

impl Draw
{

    pub fn new() -> Draw
    {

        return Draw { textures: Vec::new(), map: HashMap::new() };

    }

    pub fn get_texture(&mut self, file: &str, image_directory: &str, atlas_directory: &str) -> usize
    {

        //Check if the file has already been loaded
        match self.map.get(file)
        {

            Some(i) => return *i,
            None => {}

        }

        //It has not, so we need to load it
        let index = self.textures.len();
        self.map.insert(file.to_string(), index);

        let texture = Texture::from_file(&format!("{}{}.png", image_directory, file)).expect(&format!("Unable to load texture {}", file));

        let atlas_file = format!("{}{}.ron", atlas_directory, file);
        if std::path::Path::new(&atlas_file).exists()
        {

            let atlas = SpriteAtlas::from_file(&atlas_file);

            self.textures.push((texture, Some(atlas)));

        }
        else
        {

            self.textures.push((texture, None));

        }

        return index;

    }

    pub fn get_src(&self, texture: usize, frame: usize) -> &Rect
    {

        match &self.textures[texture].1
        {

            Some(a) => return a.get_src(frame),
            None => panic!("Texture given by {} does not have an associated atlas.", texture)

        }

    }

    //Ignores dest.x and dest.y
    pub fn create_sprite(&self, id: usize, src: &Rect, dest: &Rect) -> Sprite
    {

        if src.width == 0.0
        {

            panic!("Cannot create sprite from texture {} with zero width.", id);

        }

        if src.height == 0.0
        {

            panic!("Cannot create sprite from texture {} with zero height.", id);

        }

        let texture = &self.textures[id].0;

        let mut sprite = Sprite::with_texture_and_rect(texture, &sfml::graphics::Rect::new(src.x as i32, src.y as i32, src.width as i32, src.height as i32));
        sprite.set_scale(Vector2f::new(dest.width / src.width, dest.height / src.height));
        sprite.set_position(Vector2f::new(dest.x, dest.y));

        return sprite;

    }

    pub fn create_rect(&self, stroke: Stroke, r: &Rect) -> RectangleShape
    {

        let mut rect = RectangleShape::new();
        rect.set_size(Vector2f::new(r.width, r.height));
        
        rect.set_outline_thickness(stroke.thickness);
        rect.set_outline_color(stroke.color);
        rect.set_fill_color(stroke.fill);
        rect.set_position(Vector2f::new(r.x, r.y));

        return rect;

    }

}
