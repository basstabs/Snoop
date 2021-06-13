use legion::*;
use legion::world::SubWorld;
use legion::systems::{Builder, CommandBuffer};

use parametrizer::Parametrizer;

use super::codes::{Activate, ConsumeWatcher, Watcher};
use super::game::Timestep;
use super::space::{FLOATING_POINT_ERROR, Rect};

pub struct Velocity
{

    pub x: f32,
    pub y: f32

}

impl Velocity
{

    pub fn new(x: f32, y: f32) -> Velocity
    {

        return Velocity { x, y };

    }

    pub fn add(&mut self, (x, y): (f32, f32))
    {

        self.x += x;
        self.y += y;

    }

}

pub struct StaticBody
{

    pub body: Rect

}

pub struct OneWayBody
{

    pub body: Rect

}

pub struct InteractsWithOneWay {}

pub struct ResetOneWayInteraction
{

    time: i32,
    duration: i32

}

impl ResetOneWayInteraction
{

    pub fn new(duration: i32) -> ResetOneWayInteraction
    {

		return ResetOneWayInteraction { time: 0, duration: duration };

    }

}

pub struct Kinematic
{

    time: i32,
    direction: i32,
    x_param: Parametrizer<f32>,
    y_param: Parametrizer<f32>

}

impl Kinematic
{

    pub fn new(x_param: Parametrizer<f32>, y_param: Parametrizer<f32>) -> Kinematic
    {

        return Kinematic { time: 0, x_param: x_param, y_param: y_param, direction: 1 };

    }

    //Check to see if the dynamic body should be pushed by the kinematic
    fn should_move(kinematic: &Rect, dynamic: &mut Rect) -> bool
    {

        let shift = if dynamic.y >= kinematic.y { -1.0 } else { 1.0 };
        dynamic.y += shift;

        let intersects = Rect::intersects(kinematic, dynamic);

        dynamic.y -= shift;

        return intersects;

    }

    fn should_move_oneway(kinematic: &Rect, dynamic: &mut Rect) -> bool
    {

		let mut above = dynamic.bottom() <= kinematic.y;

		dynamic.y += 1.0;

		above = above && Rect::intersects(kinematic, dynamic);

		dynamic.y -= 1.0;

		return above;	

    }	

    fn update(&mut self, step: i32) -> (f32, f32)
    {

        self.time += step * self.direction;

        let t = self.time as f32 / 1000.0;

        let x = self.x_param.evaluate(t);
        let y = self.y_param.evaluate(t);

        return (x, y);

    }

    fn reverse(&mut self)
    {

        self.direction *= -1;

    }

    fn stop(&mut self)
    {

        if self.time >= 1000
        {

            self.time = 1000;
            self.direction = 0;

        }
        else if self.time <= 0
        {

            self.time = 0;
            self.stop();

        }

    }

    fn start(&mut self, direction: i32)
    {

        self.direction = direction;

    }

}

pub struct DynamicBody
{

    pub body: Rect,
    pub left: bool,
	pub top_collision: i32,
	temp_velocity: Velocity //Used to capture velocity added per frame from being moved by a moving platform. Necessary for accurate collisions

}

impl DynamicBody
{

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> DynamicBody
    {

        return DynamicBody { body: Rect { x, y, width, height }, left: false, top_collision: 1, temp_velocity: Velocity::new(0.0, 0.0) };

    }

    pub fn x(&self) -> f32
    {

        return self.body.x;

    }

    pub fn y(&self) -> f32
    {

        return self.body.y;

    }

    pub fn width(&self) -> f32
    {

        return self.body.width;

    }

    pub fn height(&self) -> f32
    {

        return self.body.height;

    }

    pub fn right(&self) -> f32
    {

        return self.body.right();

    }

}

pub struct RequestSizeChange
{

	pub width: f32,
	pub height: f32

}

pub struct RequestSizeChangeSuccess {}
pub struct RequestSizeChangeFailure {}

pub struct Gravity
{

    pub force: f32,
    pub max: f32

}

pub struct HasGravity {}

#[system(for_each)]
fn reset_temp_velocity(dynamic: &mut DynamicBody)
{

	dynamic.temp_velocity = Velocity::new(0.0, 0.0);

}

#[system(for_each)]
fn kinematic_toggle(kinematic: &mut Kinematic, _activate: &Activate, cmd: &mut CommandBuffer, entity: &Entity)
{

    if kinematic.time >= 1000
    {

        kinematic.start(-1);

    }
    else if kinematic.time <= 0
    {

        kinematic.start(1);

    }

    cmd.remove_component::<Activate>(*entity);

}

#[system(for_each)]
fn kinematic_stop(kinematic: &mut Kinematic, _watcher: &Watcher)
{

    kinematic.stop();

}

#[system(for_each)]
fn kinematic_consume_stop(kinematic: &mut Kinematic, _watcher: &ConsumeWatcher)
{

    kinematic.stop();

}

#[system(for_each)]
#[write_component(DynamicBody)]
fn kinematic_static_move(kinematic: &mut Kinematic, static_body: &mut StaticBody, world: &mut SubWorld, #[resource] time: &Timestep)
{

    let old_x = static_body.body.x;
    let old_y = static_body.body.y;

    let new_pos = kinematic.update(time.step);

    let velocity = (new_pos.0 - old_x, new_pos.1 - old_y);

    let mut query = <&mut DynamicBody>::query();

    for body in query.iter_mut(world)
    {

        //Move along with the platform as necessary
        if Kinematic::should_move(&static_body.body, &mut body.body)
        {

            body.body.translate(velocity);
			body.temp_velocity.add(velocity);

        }
        else
        {

            static_body.body.x = new_pos.0;
            static_body.body.y = new_pos.1;

            let correction = Rect::collides(&body.body, &static_body.body, (-velocity.0, -velocity.1));
            body.body.translate(correction); 

            static_body.body.x = old_x;
            static_body.body.y = old_y;

        }

    }

    static_body.body.x = new_pos.0;
    static_body.body.y = new_pos.1;

}

#[system(for_each)]
#[write_component(DynamicBody)]
#[read_component(InteractsWithOneWay)]
fn kinematic_oneway_move(kinematic: &mut Kinematic, oneway_body: &mut OneWayBody, world: &mut SubWorld, #[resource] time: &Timestep)
{

    let old_x = oneway_body.body.x;
    let old_y = oneway_body.body.y;

    let new_pos = kinematic.update(time.step);

    let velocity = (new_pos.0 - old_x, new_pos.1 - old_y);

    let mut query = <(&mut DynamicBody, &InteractsWithOneWay)>::query();

    for(body, _) in query.iter_mut(world)
    {

        //Move along with the platform as necessary
        if Kinematic::should_move_oneway(&oneway_body.body, &mut body.body)
        {

            body.body.translate(velocity);
			body.temp_velocity.add(velocity);

        }
        else
        {

            oneway_body.body.x = new_pos.0;
            oneway_body.body.y = new_pos.1;

            let correction = Rect::collides(&body.body, &oneway_body.body, (-velocity.0, -velocity.1)); 

            if correction.1 < 0.0 && body.body.bottom() + velocity.1 <= oneway_body.body.y
            {

                body.body.y += correction.1;

            }

            oneway_body.body.x = old_x;
            oneway_body.body.y = old_y;

        }

    }

    oneway_body.body.x = new_pos.0;
    oneway_body.body.y = new_pos.1;

}
#[system(for_each)]
fn velocity(dynamic_body: &mut DynamicBody, velocity: &Velocity)
{

    dynamic_body.body.translate((velocity.x, velocity.y));

}

#[system(for_each)]
fn facing(dynamic_body: &mut DynamicBody, velocity: &Velocity)
{

    if velocity.x < -FLOATING_POINT_ERROR
    {

        dynamic_body.left = true;

    }
    else if velocity.x > FLOATING_POINT_ERROR
    {

        dynamic_body.left = false;

    }

}

#[system(for_each)]
fn gravity(velocity: &mut Velocity, _g: &HasGravity, #[resource] gravity: &Gravity)
{

    velocity.y = (velocity.y + gravity.force).min(gravity.max);

}

#[system(for_each)]
fn reset_oneway(reset: &mut ResetOneWayInteraction, cmd: &mut CommandBuffer, entity: &Entity, #[resource] time: &Timestep)
{

    reset.time += time.step;

    if reset.time >= reset.duration
    {

		cmd.remove_component::<ResetOneWayInteraction>(*entity);
		cmd.add_component(*entity, InteractsWithOneWay {});

    }

}

#[system(for_each)]
fn top_collision(body: &mut DynamicBody, #[resource] time: &Timestep)
{

    body.top_collision += time.step;

}

#[system(for_each)]
#[read_component(StaticBody)]
fn static_collision(dynamic_body: &mut DynamicBody, velocity: &mut Velocity, world: &mut SubWorld)
{

    let mut query = <&StaticBody>::query();

    let mut top = false;

	let mut collision_velocity = (velocity.x + dynamic_body.temp_velocity.x, velocity.y); //Currently unclear why adding temp_velocity.y breaks everything, but it does. Will revisit if necessary

    for body in query.iter(world)
    {

        let correction = Rect::collides(&dynamic_body.body, &body.body, collision_velocity);  

        dynamic_body.body.translate(correction);
        velocity.add(correction);
		collision_velocity.0 += correction.0;
		collision_velocity.1 += correction.1;

        if correction.1 != 0.0
        {

            velocity.y = 0.0;
			collision_velocity.1 = 0.0;

        }

        if correction.1 < 0.0
        {

            top = true;

        }

    }

    if top
    {

        dynamic_body.top_collision = 0;

    }

}

#[system(for_each)]
#[read_component(OneWayBody)]
fn oneway_collision(dynamic_body: &mut DynamicBody, velocity: &mut Velocity, _interacts: &InteractsWithOneWay, world: &mut SubWorld)
{

    let mut query = <&OneWayBody>::query();

    let mut top = false;

	let mut collision_velocity = (velocity.x + dynamic_body.temp_velocity.x, velocity.y + dynamic_body.temp_velocity.y);

    for body in query.iter(world)
    {

        let correction = Rect::collides(&dynamic_body.body, &body.body, collision_velocity);

        if correction.1 < 0.0 && dynamic_body.body.bottom() - collision_velocity.1 <= body.body.y
        {

            dynamic_body.body.y += correction.1;
            velocity.y = 0.0;
			collision_velocity.1 = 0.0;

            top = true;

        }

    }

    if top
    {

        dynamic_body.top_collision = 0;

    }

}

#[system(for_each)]
#[read_component(StaticBody)]
#[read_component(OneWayBody)]
fn request_size_change(dynamic_body: &mut DynamicBody, request: &RequestSizeChange, world: &mut SubWorld, cmd: &mut CommandBuffer, entity: &Entity)
{

	let new_rect = Rect { x: dynamic_body.body.x, y: dynamic_body.body.y + (dynamic_body.body.height - request.height), width: request.width, height: request.height };

	let mut allowed = true;

	let mut query = <&StaticBody>::query();

	for body in query.iter(world)
	{

		if Rect::intersects(&new_rect, &body.body)
		{

			allowed = false;
			break;

		}

	}

	if allowed
	{

		dynamic_body.body = Rect { x: new_rect.x, y: new_rect.y, width: new_rect.width, height: new_rect.height };

		cmd.add_component(*entity, RequestSizeChangeSuccess {});

	}
	else
	{

		cmd.add_component(*entity, RequestSizeChangeFailure {});

	}

	cmd.remove_component::<RequestSizeChange>(*entity);

}

#[system(for_each)]
fn resize_failure(_fail: &RequestSizeChangeFailure, cmd: &mut CommandBuffer, entity: &Entity)
{

	cmd.remove_component::<RequestSizeChangeFailure>(*entity);

}

#[system(for_each)]
fn resize_success(_success: &RequestSizeChangeSuccess, cmd: &mut CommandBuffer, entity: &Entity)
{

	cmd.remove_component::<RequestSizeChangeSuccess>(*entity);

}

pub fn schedule_early_systems(schedule: &mut Builder)
{

    schedule.add_system(kinematic_toggle_system());
    schedule.add_system(kinematic_stop_system());
    schedule.add_system(kinematic_consume_stop_system());

	schedule.add_system(reset_temp_velocity_system());
	schedule.add_system(reset_oneway_system());
	schedule.add_system(top_collision_system());

}

pub fn schedule_physics_systems(schedule: &mut Builder)
{

	schedule.add_system(kinematic_static_move_system());
    schedule.add_system(kinematic_oneway_move_system());

    schedule.add_system(gravity_system());

    schedule.add_system(velocity_system());
    schedule.add_system(facing_system());

    schedule.add_system(static_collision_system());
    schedule.add_system(oneway_collision_system());

	schedule.add_system(request_size_change_system());

}

pub fn schedule_cleanup_systems(schedule: &mut Builder)
{

	schedule_request_systems(schedule);

}

fn schedule_request_systems(schedule: &mut Builder)
{

	schedule.add_system(resize_failure_system());
	schedule.add_system(resize_success_system());

}


