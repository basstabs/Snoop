use legion::*;
use legion::world::SubWorld;
use legion::systems::{Builder, CommandBuffer};

use engine::codes::Codes;
use engine::game::Timestep;
use engine::physics::{DynamicBody, InteractsWithOneWay, ResetOneWayInteraction, RequestSizeChange, RequestSizeChangeSuccess, RequestSizeChangeFailure, Velocity};
use engine::space::{FLOATING_POINT_ERROR, Rect};
use engine::sprites::SpriteSheet;

use super::eventmap::Trigger;

const NUM_COMMANDS: usize = 6;

const PLAYER_LEFT: usize = 0;
const PLAYER_RIGHT: usize = 1;
const PLAYER_UP: usize = 2;
const PLAYER_DOWN: usize = 3;
const PLAYER_JUMP: usize = 4;
const PLAYER_INTERACT: usize = 5;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum InputState
{

    Up,
    Down,
    Pressed

}

pub struct InputCommand
{

    pub commands: Vec<InputState>

}

impl InputCommand
{

    pub fn new() -> InputCommand
    {

        return InputCommand { commands: vec![InputState::Up; NUM_COMMANDS] };

    }

    pub fn merge(&mut self, old: &InputCommand)
    {

        for i in 0..self.commands.len()
        {

            self.commands[i] = InputCommand::merge_states(self.commands[i], old.commands[i]);

        }

    }

    pub fn bool_to_state(down: bool) -> InputState
    {

        if down
        {

            return InputState::Down;

        }

        return InputState::Up;

    }

    fn merge_states(state: InputState, old: InputState) -> InputState
    {

        match (state, old)
        {

           (InputState::Down, InputState::Up) => return InputState::Pressed,
           (InputState::Down, InputState::Pressed) => return InputState::Pressed, //Remain pressed until it is polled
           _ => return state
        
        }

    }

    pub fn down(&mut self, index: usize) -> bool
    {

        if self.commands[index] == InputState::Pressed
        {

            self.commands[index] = InputState::Down;

            return true;

        }

        return self.commands[index] == InputState::Down;

    }

    pub fn up(&self, index: usize) -> bool
    {

        return self.commands[index] == InputState::Up;

    }

    pub fn pressed(&mut self, index: usize) -> bool
    {

        if self.commands[index] == InputState::Pressed
        {

            self.commands[index] = InputState::Down;

            return true;

        }

        return false;

    }

}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PlayerState
{

	Normal,
	Crouching

}

pub struct Player
{

    run_speed: f32,
    jump_speed: f32,
	crawl_speed: f32,
	state_time: i32,
	state: PlayerState,
	next: PlayerState

}

impl Player
{

    pub fn new(r: f32, j: f32, c: f32) -> Player
    {

        return Player { run_speed: r, jump_speed: j, crawl_speed: c, state_time: 0, state: PlayerState::Normal, next: PlayerState::Normal };

    }

	fn horizontal_speed(&self) -> f32
	{

		if self.state == PlayerState::Normal
		{

			return self.run_speed;

		}
		else if self.state == PlayerState::Crouching
		{

			return self.crawl_speed;

		}

		return 0.0

	}

	fn can_jump(&self) -> bool
	{

		return true;

	}

	fn change_state(&mut self, state: PlayerState)
	{

		self.next = state;

	}

	fn finish_state_change(&mut self)
	{

		self.state = self.next;

		self.state_time = 0;

	}

	fn revert_state_change(&mut self)
	{

		self.next = self.state;

	}

}

#[system(for_each)]
fn player_state_update(player: &mut Player, #[resource] step: &Timestep)
{

	player.state_time += step.step;

}

#[system(for_each)]
fn player_move(player: &mut Player, velocity: &mut Velocity, dynamic: &DynamicBody, #[resource] step: &Timestep, #[resource] input: &mut InputCommand)
{

	if dynamic.top_collision == 0
	{

	    velocity.x = 0.0;

		let horizontal_speed = player.horizontal_speed();

    	if input.down(PLAYER_LEFT)
    	{

        	velocity.x = -horizontal_speed;

    	}

    	if input.down(PLAYER_RIGHT)
    	{

        	velocity.x = horizontal_speed;

    	}

    	velocity.x *= step.step as f32 / 1000.0;

		if input.pressed(PLAYER_JUMP) && input.up(PLAYER_DOWN) && player.can_jump()
    	{

        	velocity.y = -player.jump_speed;

    	}

	}

}

#[system(for_each)]
fn player_oneway(_player: &Player, _interacts: &InteractsWithOneWay, cmd: &mut CommandBuffer, entity: &Entity, #[resource] input: &mut InputCommand)
{

    if input.pressed(PLAYER_JUMP) && input.down(PLAYER_DOWN)
    {

		cmd.remove_component::<InteractsWithOneWay>(*entity);
		cmd.add_component(*entity, ResetOneWayInteraction::new(100));

    }

}

#[system(for_each)]
#[write_component(Trigger)]
fn player_trigger(_player: &Player, dynamic: &DynamicBody, world: &mut SubWorld, cmd: &mut CommandBuffer,#[resource] input: &mut InputCommand, #[resource] codes: &mut Codes)
{

    let mut trigger_query  = <(&mut Trigger, Entity)>::query();

    for (trigger, entity) in trigger_query.iter_mut(world)
    {

        if input.pressed(PLAYER_INTERACT) && Rect::intersects(&dynamic.body, &trigger.rect)
        {

            codes.insert(trigger.code);
            trigger.count -= 1;

            if trigger.count == 0
            {

                cmd.remove(*entity);

            }

        }

    }

}

#[system(for_each)]
fn player_state(player: &mut Player, dynamic: &DynamicBody, cmd: &mut CommandBuffer, entity: &Entity, #[resource] input: &mut InputCommand)
{

	if player.state == PlayerState::Normal
	{

		if input.down(PLAYER_DOWN) && input.up(PLAYER_JUMP) && dynamic.top_collision == 0
		{

			player.change_state(PlayerState::Crouching);

			cmd.add_component(*entity, RequestSizeChange { width: dynamic.body.width, height: dynamic.body.height * 0.5 });

		}

	}

	if player.state == PlayerState::Crouching
	{

		if input.down(PLAYER_UP) || input.down(PLAYER_JUMP) || dynamic.top_collision > 0
		{

			player.change_state(PlayerState::Normal);

			cmd.add_component(*entity, RequestSizeChange { width: dynamic.body.width, height: dynamic.body.height * 2.0 } );

		}

	}

}

#[system(for_each)]
fn player_resize_failure(player: &mut Player, _failure: &RequestSizeChangeFailure, cmd: &mut CommandBuffer, entity: &Entity)
{

	player.revert_state_change();

	cmd.remove_component::<RequestSizeChangeFailure>(*entity);

}

#[system(for_each)]
fn player_resize_success(player: &mut Player, _success: &RequestSizeChangeSuccess, cmd: &mut CommandBuffer, entity: &Entity)
{

	player.finish_state_change();

	cmd.remove_component::<RequestSizeChangeFailure>(*entity);

}

#[system(for_each)]
fn player_animation(_player: &Player, velocity: &Velocity, dynamic: &DynamicBody, sprite: &mut SpriteSheet)
{

	if dynamic.top_collision == 0
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
	else
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

}

pub fn schedule_early_systems(schedule: &mut Builder)
{

	schedule.add_system(player_resize_success_system());
	schedule.add_system(player_resize_failure_system());
	schedule.add_system(player_state_update_system());
	schedule.add_system(player_move_system());
	schedule.add_system(player_oneway_system());
    schedule.add_system(player_trigger_system());
    schedule.add_system(player_state_system());
		
}

pub fn schedule_animation_systems(schedule: &mut Builder)
{

   schedule.add_system(player_animation_system());

}
