use ron::de::from_reader;
use serde::Deserialize;

use legion::*;

use std::fs::File;

use super::super::engine::camera::WorldSize;

use super::collisionmap;
use super::eventmap;

#[derive(Deserialize)]
struct Level
{

    width: f32,
    height: f32,
    collision: (String, String),
    event: (String, String)

}

pub fn load_level(world: &mut World, resources: &mut Resources, file: &str, directory: &str)
{

    let f = File::open(&format!("{}{}.ron", directory, file)).expect(&format!("Unable to open level file {}", file));
    let parse: Result<Level, _> = from_reader(f);

    match parse
    {

        Ok(l) =>
        {

            resources.insert(WorldSize { width: l.width, height: l.height });
            
            collisionmap::load_collision(world, &l.collision.0, &l.collision.1);
            eventmap::load_events(world, &l.event.0, &l.event.1);

        }
        Err(e) => panic!("Unable to parse level RON file {} with error {}", file, e) 

    }

}
