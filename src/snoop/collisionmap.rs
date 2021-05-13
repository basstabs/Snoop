use ron::de::from_reader;
use serde::Deserialize;

use legion::*;

use std::fs::File;

use super::super::engine::space::Rect;
use super::super::engine::physics::{Kinematic, OneWayBody, StaticBody, Velocity};

#[derive(Deserialize)]
struct Body
{

    body: Rect,
    oneway: bool

}

#[derive(Deserialize)]
struct CollisionMap
{

    bodies: Vec<Body>

}

pub fn load_collision(world: &mut World, file: &str, directory: &str)
{

    let f = File::open(&format!("{}{}.ron", directory, file)).expect(&format!("Unable to open collision map file {}", file));
    let parse: Result<CollisionMap, _> = from_reader(f);

    match parse
    {

        Ok(m) =>
        {

            for body in m.bodies
            {

                if body.oneway
                {

                    world.push((
                           
                           OneWayBody { body: body.body },

                    ));

                }
                else
                {

                    world.push((

                            StaticBody { body: body.body },

                    ));

                }

            }

        }
        Err(e) => panic!("Unable to parse collision map RON file {} with error {}", file, e) 

    }

}
