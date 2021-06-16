use legion::*;
use legion::systems::{Builder, CommandBuffer};

use std::collections::{HashMap, HashSet};

pub struct Codes
{

    listing: HashMap<String, u128>,
	active: HashSet<u128>,
    current_code: u128

}

impl Codes
{

	pub fn new() -> Codes
	{

		return Codes { listing: HashMap::new(), active: HashSet::new(), current_code: 0 };

	}

	pub fn insert(&mut self, code: u128)
	{

		self.active.insert(code);

	}

	pub fn contains(&self, code: u128) -> bool
	{

		return self.active.contains(&code);

	}

	fn consume(&mut self, code: u128) -> bool
	{

		if self.active.contains(&code)
		{

			self.active.remove(&code);

			return true;

		}

		return false;

	}

    pub fn get_code(&mut self, name: String) -> u128
    {

        let code = self.current_code;

        self.current_code += 1;

        return code;

    }

    pub fn codes_interact(passive: u128, active: u128) -> bool
    {

        return true;

    }

}

pub struct Watcher
{

    code: u128,
    activated: bool

}

pub struct ConsumeWatcher
{

    code: u128

}

pub struct Activate
{

    pub code: u128

}

#[system(for_each)]
fn watcher(watcher: &mut Watcher, cmd: &mut CommandBuffer, entity: &Entity, #[resource] codes: &Codes)
{

    if !watcher.activated && codes.contains(watcher.code)
    {

        watcher.activated = true;

        cmd.add_component(*entity, Activate { code: watcher.code });

    }

}

#[system(for_each)]
fn consume_watcher(watcher: &ConsumeWatcher, cmd: &mut CommandBuffer, entity: &Entity, #[resource] codes: &mut Codes)
{

    if codes.consume(watcher.code)
    {

        cmd.add_component(*entity, Activate { code: watcher.code });

    }

}

pub fn schedule_watcher_systems(schedule: &mut Builder)
{

    schedule.add_system(watcher_system());
    schedule.add_system(consume_watcher_system());

}
