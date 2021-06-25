use ron::de::from_reader;
use serde::Deserialize;
use parametrizer::Parametrizer;

use legion::*;

use std::fs::File;

use engine::codes::{Codes, ConsumeWatcher, Watcher, WatcherData};
use engine::space::Rect;
use engine::physics::{Kinematic, OneWayBody, StaticBody};

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
    y_param: String,
    max: i32,
    watchers: Vec<WatcherData>

} 

#[derive(Deserialize)]
struct CollisionMap
{

    bodies: Vec<Body>,
    platforms: Vec<Platform>

}

pub fn load_collision(world: &mut World, codes: &mut Codes, file: &str, directory: &str)
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

                    world.push(
                    (
                           
                           OneWayBody { body: body.body },

                    ));

                }
                else
                {

                    world.push(
                    (

                            StaticBody { body: body.body },

                    ));

                }

            }

            for platform in m.platforms
            {

                let x_param = Parametrizer::new(&platform.x_param).unwrap();
                let y_param = Parametrizer::new(&platform.y_param).unwrap();

                let entity: Entity = world.push((Kinematic::new(x_param, y_param, platform.max), ));
                let mut entry = world.entry(entity).unwrap();

                if platform.body.oneway
                {

                    entry.add_component(OneWayBody { body: platform.body.body });

                }
                else
                {

                    entry.add_component(StaticBody { body: platform.body.body });

                }

                for data in platform.watchers.iter()
                {

                    if data.consume
                    {

                        entry.add_component(ConsumeWatcher { code: codes.get_code(&data.code) });

                    }
                    else
                    {

                        entry.add_component(Watcher { code: codes.get_code(&data.code), activated: false });

                    }

                }

            }

        }

        Err(e) => panic!("Unable to parse collision map RON file {} with error {}", file, e) 

    }

}
