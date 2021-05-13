use sfml::graphics::{Color, RenderTarget, RenderWindow};

use legion::*;
use legion::systems::Builder;
use legion::storage::ComponentTypeId;

use super::engine::game::{State, Timestep};
use super::engine::draw::{Draw, Stroke};
use super::engine::physics;
use super::engine::physics::{DynamicBody, Gravity, HasGravity, Kinematic, OneWayBody, StaticBody, Velocity};
use super::engine::sprites;
use sprites::{Sheets, SpriteSheet};
use super::engine::space::{Point, Rect};

mod collisionmap;

pub struct Snoop
{

    world: World,
    schedule: Schedule,
    resources: Resources,
    debug: bool

}

impl Snoop
{

    pub fn new(step: i32) -> Snoop
    {

        let mut world = World::default();

        let mut draw = Draw::new();
        let mut sheets = Sheets::new();;

        let timestep = (step as f32) / 1000.0;

        world.push(
        (

            SpriteSheet::from_files(&mut draw, &mut sheets, "Character", "./assets/images/", "./assets/data/atlases/", "Character", "./assets/data/sheets/"),
            HasGravity {},
            Velocity::new(0.0, 0.0),
            DynamicBody::new(10.0, 10.0, 54.0, 54.0)

        ));

        world.push(
        (

            StaticBody::new(0.0, 500.0, 500.0, 25.0),

        ));

        let mut resources = Resources::default();
        resources.insert(draw);
        resources.insert(sheets);
        resources.insert(Timestep { step });
        resources.insert(Gravity { force: 20.0 * timestep, max: 1000.0 * timestep});

        let mut schedule_builder = Schedule::builder();
     
        Snoop::schedule_early_systems(&mut schedule_builder);
        Snoop::schedule_physics_systems(&mut schedule_builder);

        schedule_builder.add_system(sprites::update_spritesheets_system());

        let schedule = schedule_builder.build();

        return Snoop { world, schedule, resources, debug: true };

    }

    fn schedule_early_systems(schedule: &mut Builder)
    {

        schedule.add_system(physics::top_collision_system());

    }

    fn schedule_physics_systems(schedule: &mut Builder)
    {

        schedule.add_system(physics::gravity_system());

        schedule.add_system(physics::velocity_system());

        schedule.add_system(physics::static_collision_system());
        schedule.add_system(physics::oneway_collision_system());

    }

    fn debug_render(&mut self, window: &mut RenderWindow, time: f32)
    {

        let draw = self.resources.get::<Draw>().unwrap();

        let outline = Color::rgba(255, 255, 255, 200);

        let mut static_query = <(&StaticBody)>::query().filter(component::<Kinematic>() | !component::<Kinematic>());
        for chunk in static_query.iter_chunks(&mut self.world)
        {

            let mut fill = Color::rgba(255, 0, 0, 100);

            if chunk.archetype().layout().component_types().contains(&ComponentTypeId::of::<Kinematic>())
            {

                fill = Color::rgba(255, 0, 255, 100);

            }

            for body in chunk
            { 

                let rect = draw.create_rect(Stroke::new(outline, fill, 1.0), &body.body);
                window.draw(&rect);

            }

        }

        let mut oneway_query = <(&OneWayBody)>::query().filter(component::<Kinematic>() | !component::<Kinematic>());
        for chunk in oneway_query.iter_chunks(&mut self.world)
        {

            let mut fill = Color::rgba(0, 255, 0, 100);

            if chunk.archetype().layout().component_types().contains(&ComponentTypeId::of::<Kinematic>())
            {

                fill = Color::rgba(0, 255, 255, 100);

            }

            for body in chunk
            { 

                let rect = draw.create_rect(Stroke::new(outline, fill, 1.0), &body.body);
                window.draw(&rect);

            }

        }

        let mut dynamic_query = <(&DynamicBody, &Velocity)>::query().filter(component::<HasGravity>() | !component::<HasGravity>());
        for chunk in dynamic_query.iter_chunks(&mut self.world)
        {

            let mut fill = Color::rgba(0, 0, 255, 100);

            if chunk.archetype().layout().component_types().contains(&ComponentTypeId::of::<HasGravity>())
            {

                fill = Color::rgba(128, 0, 255, 100);

            }

            for (body, velocity) in chunk
            {

                let rect = draw.create_rect(Stroke::new(outline, fill, 1.0), &Rect { x: body.x() + time * velocity.x, y: body.y() + time * velocity.y, width: body.body.width, height: body.body.height });
                window.draw(&rect);

            }

        }

    }

}

impl State for Snoop
{

    fn initialize(&mut self)
    {

    }

    fn update(&mut self, timestep: i32) -> bool
    {

        self.schedule.execute(&mut self.world, &mut self.resources);

        return true

    }

    fn render(&mut self, window: &mut RenderWindow, time: f32)
    {

        //Normal render block, subscoped so that immutable borrows do not interfere with
        //mutable borrow required by debug render block
        {

            let draw = self.resources.get::<Draw>().unwrap();
            let sheets = self.resources.get::<Sheets>().unwrap();

            let mut query = <(&SpriteSheet, &DynamicBody, &Velocity)>::query();
            for (sheet, body, velocity) in query.iter(&mut self.world)
            {

                let rect = sheet.get_src(&draw, sheet.get_sheet(&sheets));
                let sprite = draw.create_sprite(sheet.texture, rect, &Rect { x: body.x() + time * velocity.x, y: body.y() + time * velocity.y, width: rect.width, height: rect.height });

                window.draw(&sprite);

            }

        }

        if self.debug
        {

            self.debug_render(window, time);

        }

    } 
    
    fn active(&self) -> bool
    {

        return true;

    }

}
