use legion::*;
use legion::systems::{Builder, CommandBuffer};

use super::super::engine::game::Timestep;
use super::super::engine::physics::{InteractsWithOneWay, ResetOneWayInteraction, TopCollision, Velocity};
use super::super::engine::space::FLOATING_POINT_ERROR;
use super::super::engine::sprites::SpriteSheet;

pub struct InputCommand
{

    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub jump: bool

}

pub struct CharacterNormal {}

struct CharacterCrouching {}

pub struct Character
{

    run_speed: f32,
    jump_speed: f32,
	crawl_speed: f32,
	state_time: i32

}

impl Character
{

    pub fn new(r: f32, j: f32, c: f32) -> Character
    {

        return Character { run_speed: r, jump_speed: j, crawl_speed: c, state_time: 0 };

    }

}

#[system(for_each)]
pub fn character_state_update(character: &mut Character, #[resource] step: &Timestep)
{

	character.state_time += step.step;

}

#[system(for_each)]
pub fn character_move(character: &mut Character, velocity: &mut Velocity, _top: &TopCollision, #[resource] step: &Timestep, #[resource] input: &InputCommand)
{

    velocity.x = 0.0;

	let horizontal_speed = character.run_speed;

    if input.left
    {

        velocity.x = -horizontal_speed;

    }

    if input.right
    {

        velocity.x = horizontal_speed;

    }

    velocity.x *= step.step as f32 / 1000.0;

	if input.jump && !input.down
	{

		velocity.y = -character.jump_speed;

	}

}

#[system(for_each)]
pub fn character_jump(character: &Character, velocity: &mut Velocity, _top: &TopCollision, #[resource] input: &InputCommand)
{

    if input.jump && !input.down
    {

        velocity.y = -character.jump_speed;

    }

}

#[system(for_each)]
pub fn character_oneway(_character: &Character, _interacts: &InteractsWithOneWay, cmd: &mut CommandBuffer, entity: &Entity, #[resource] input: &InputCommand)
{

    if input.jump && input.down
    {

		cmd.remove_component::<InteractsWithOneWay>(*entity);
		cmd.add_component(*entity, ResetOneWayInteraction::new(100));

    }

}

#[system(for_each)]
pub fn character_run_animation(_character: &Character, velocity: &Velocity, _top: &TopCollision, sprite: &mut SpriteSheet)
{

    if velocity.x.abs() <= FLOATING_POINT_ERROR
    {

        sprite.run(0, true);

    }
    else
    {

        sprite.run(2, true);

    }

}

#[system(for_each)]
#[filter(!component::<TopCollision>())]
pub fn character_drop_animation(_character: &Character, velocity: &Velocity, sprite: &mut SpriteSheet)
{

    if velocity.y < 0.0
    {

        sprite.run(4, true);

    }
    else
    {

        sprite.run(6, true);

    }

}

pub fn schedule_early_systems(schedule: &mut Builder)
{

	schedule.add_system(character_move_system());
    schedule.add_system(character_jump_system());
	schedule.add_system(character_oneway_system());
		
}

pub fn schedule_animation_systems(schedule: &mut Builder)
{

   schedule.add_system(character_run_animation_system());
   schedule.add_system(character_drop_animation_system());

}
