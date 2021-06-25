use ron::de::from_reader;
use serde::Deserialize; 

use legion::*;
use legion::systems::Builder;

use std::fs::File;

use super::super::engine::space::Rect;
use super::super::engine::codes::Codes;

pub struct Trigger
{

    pub code: u128,
    pub rect: Rect,
    pub count: i32
    
}

#[derive(Deserialize)]
struct EventMap
{

    triggers: Vec<TriggerData>

}

#[derive(Deserialize)]
struct TriggerData
{

    code: String,
    rect: Rect,
    count: i32

}

pub fn load_events(world: &mut World, codes: &mut Codes, file: &str, directory: &str)
{

    let f = File::open(&format!("{}{}.ron", directory, file)).expect(&format!("Unable to open event map file {}", file));
    let parse: Result<EventMap, _> = from_reader(f);

    match parse
    {

        Ok(e) =>
        {

            for trigger in e.triggers
            {

                world.push(
                (

                    Trigger { code: codes.get_code(trigger.code), rect: trigger.rect, count: trigger.count },

                ));

            }

        }
        Err(e) => panic!("Unable to parse event map RON file {} with error {}", file, e) 

    }

}

