use ron::de::from_reader;
use serde::Deserialize; 

use legion::*;
use legion::systems::Builder;

use std::fs::File;

#[derive(Deserialize)]
struct EventMap
{
}

struct Trigger
{

    code: u128,
    

}

pub fn schedule_event_systems(schedule: &mut Builder)
{
}

pub fn load_events(world: &mut World, file: &str, directory: &str)
{

    let f = File::open(&format!("{}{}.ron", directory, file)).expect(&format!("Unable to open event map file {}", file));
    let parse: Result<EventMap, _> = from_reader(f);

    match parse
    {

        Ok(e) =>
        {



        }
        Err(e) => panic!("Unable to parse event map RON file {} with error {}", file, e) 

    }

}

