use legion::*;
use legion::world::SubWorld;
use legion::systems::CommandBuffer;

use parametrizer::Parametrizer;

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

pub struct Kinematic
{

    time: i32,
    x_param: Parametrizer<f32>,
    y_param: Parametrizer<f32>

}

impl Kinematic
{

    pub fn new(x_param: Parametrizer<f32>, y_param: Parametrizer<f32>) -> Kinematic
    {

        return Kinematic { time: 0, x_param: x_param, y_param: y_param };

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

        self.time += step;

        let t = self.time as f32 / 1000.0;

        let x = self.x_param.evaluate(t);
        let y = self.y_param.evaluate(t);

        return (x, y);

    }

}

pub struct DynamicBody
{

    pub body: Rect,
    pub left: bool

}

impl DynamicBody
{

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> DynamicBody
    {

        return DynamicBody { body: Rect { x, y, width, height }, left: false };

    }

    pub fn x(&self) -> f32
    {

        return self.body.x;

    }

    pub fn y(&self) -> f32
    {

        return self.body.y;

    }

    pub fn right(&self) -> f32
    {

        return self.body.right();

    }

}

pub struct TopCollision {}

pub struct Gravity
{

    pub force: f32,
    pub max: f32

}

pub struct HasGravity {}

impl TopCollision
{

    fn add(entity: &Entity, cmd: &mut CommandBuffer)
    {

        cmd.add_component(*entity, TopCollision {});

    }

    fn remove(entity: &Entity, cmd: &mut CommandBuffer)
    {

        cmd.remove_component::<TopCollision>(*entity);

    }

}

#[system(for_each)]
#[write_component(DynamicBody)]
pub fn kinematic_static_move(kinematic: &mut Kinematic, static_body: &mut StaticBody, world: &mut SubWorld, #[resource] time: &Timestep)
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
pub fn kinematic_oneway_move(kinematic: &mut Kinematic, oneway_body: &mut OneWayBody, world: &mut SubWorld, #[resource] time: &Timestep)
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
pub fn velocity(dynamic_body: &mut DynamicBody, velocity: &Velocity)
{

    dynamic_body.body.translate((velocity.x, velocity.y));

}

#[system(for_each)]
pub fn facing(dynamic_body: &mut DynamicBody, velocity: &Velocity)
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
pub fn gravity(velocity: &mut Velocity, _g: &HasGravity, #[resource] gravity: &Gravity)
{

    velocity.y = (velocity.y + gravity.force).min(gravity.max);

}

#[system(for_each)]
pub fn top_collision(_top: &TopCollision, cmd: &mut CommandBuffer, entity: &Entity)
{

    TopCollision::remove(entity, cmd);

}

#[system(for_each)]
#[read_component(StaticBody)]
pub fn static_collision(dynamic_body: &mut DynamicBody, velocity: &mut Velocity, world: &mut SubWorld, cmd: &mut CommandBuffer, entity: &Entity)
{

    let mut query = <&StaticBody>::query();

    let mut top = false;

    for body in query.iter(world)
    {

        let correction = Rect::collides(&dynamic_body.body, &body.body, (velocity.x, velocity.y));  

        dynamic_body.body.translate(correction);
        velocity.add(correction);

        if correction.1 != 0.0
        {

            velocity.y = 0.0;

        }

        if correction.1 < 0.0
        {

            top = true;

        }

    }

    if top
    {

        TopCollision::add(entity, cmd);

    }

}

#[system(for_each)]
#[read_component(OneWayBody)]
pub fn oneway_collision(dynamic_body: &mut DynamicBody, velocity: &mut Velocity, _interacts: &InteractsWithOneWay, world: &mut SubWorld, cmd: &mut CommandBuffer, entity: &Entity)
{

    let mut query = <&OneWayBody>::query();

    let mut top = false;

    for body in query.iter(world)
    {

        let correction = Rect::collides(&dynamic_body.body, &body.body, (velocity.x, velocity.y));

        if correction.1 < 0.0 && dynamic_body.body.bottom() - velocity.y <= body.body.y
        {

            dynamic_body.body.y += correction.1;
            velocity.y = 0.0;

            top = true;

        }

    }

    if top
    {

        TopCollision::add(entity, cmd);

    }

}
