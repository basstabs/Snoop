use legion::*;
use legion::world::SubWorld;
use legion::systems::CommandBuffer;

use super::space::Rect;

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

pub struct Kinematic
{

}

pub struct DynamicBody
{

    pub body: Rect

}

impl DynamicBody
{

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> DynamicBody
    {

        return DynamicBody { body: Rect { x, y, width, height } };

    }

    pub fn x(&self) -> f32
    {

        return self.body.x;

    }

    pub fn y(&self) -> f32
    {

        return self.body.y;

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
pub fn velocity(dynamic_body: &mut DynamicBody, velocity: &Velocity)
{

    dynamic_body.body.translate((velocity.x, velocity.y));

}

#[system(for_each)]
pub fn gravity(velocity: &mut Velocity, _g: &HasGravity, #[resource] gravity: &Gravity)
{

    velocity.y = (velocity.y + gravity.force).min(gravity.max);

}

#[system(for_each)]
pub fn top_collision(top: &TopCollision, cmd: &mut CommandBuffer, entity: &Entity)
{

    TopCollision::remove(entity, cmd);

}

#[system(for_each)]
#[read_component(StaticBody)]
pub fn static_collision(dynamic_body: &mut DynamicBody, velocity: &mut Velocity, world: &mut SubWorld, cmd: &mut CommandBuffer, entity: &Entity)
{

    let mut query = <&StaticBody>::query();

    let mut top = false;

    for (body) in query.iter(world)
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
pub fn oneway_collision(dynamic_body: &mut DynamicBody, velocity: &mut Velocity, world: &mut SubWorld, cmd: &mut CommandBuffer, entity: &Entity)
{

    let mut query = <&OneWayBody>::query();

    let mut top = false;

    for (body) in query.iter(world)
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
