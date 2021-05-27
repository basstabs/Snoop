use ron::de::from_reader;
use serde::Deserialize;
use parametrizer::Parametrizer;

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
struct Platform
{

    body: Body,
    x_param: String,
    y_param: String

}

#[derive(Deserialize)]
struct CollisionMap
{

    bodies: Vec<Body>,
    platforms: Vec<Platform>

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

            for platform in m.platforms
            {

                let x_param = Parametrizer::new(&platform.x_param).unwrap();
                let y_param = Parametrizer::new(&platform.y_param).unwrap();

                if platform.body.oneway
                {

                    world.push((

                            OneWayBody { body: platform.body.body },
                            Kinematic::new(x_param, y_param)

                    ));

                }
                else
                {

                    world.push((

                            StaticBody { body: platform.body.body },
                            Kinematic::new(x_param, y_param)

                    ));

                }

            }

        }
        Err(e) => panic!("Unable to parse collision map RON file {} with error {}", file, e) 

    }

}
