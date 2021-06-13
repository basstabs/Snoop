use sfml::SfBox;
use sfml::graphics::{Color, RenderTarget, RenderWindow, View};
use sfml::system::Vector2f;
use sfml::window::Key;

use legion::*;
use legion::systems::Builder;
use legion::storage::ComponentTypeId;

use super::engine::alarm;
use alarm::{Cone, Observer, Walls};

use super::engine::camera;
use camera::{Camera, Target};

use super::engine::codes;
use codes::{Codes};

use super::engine::game::{State, Timestep};

use super::engine::draw::{Draw, Stroke};

use super::engine::input::Input;

use super::engine::physics;
use physics::{DynamicBody, Gravity, HasGravity, InteractsWithOneWay, Kinematic, OneWayBody, StaticBody, Velocity};

use super::engine::sprites;
use sprites::{Sheets, SpriteSheet};

use super::engine::space::{Point, Rect};

mod level;
mod collisionmap;
mod player;

use player::{Player, InputCommand};

pub struct Snoop
{

    world: World,
    schedule: Schedule,
    resources: Resources,
    view: SfBox<View>,
    debug: bool

}

impl Snoop
{

    pub fn new(step: i32) -> Snoop
    {

        let mut world = World::default();

        let mut draw = Draw::new();
        let mut sheets = Sheets::new();

        let timestep = (step as f32) / 1000.0;

        let mut resources = Resources::default();
        resources.insert(Timestep { step });
        resources.insert(Gravity { force: 20.0 * timestep, max: 1000.0 * timestep});
		resources.insert(Walls::new());
        resources.insert(Codes::new());

        camera::register_camera_resources(&mut resources, 400.0, 400.0);

        world.push(
        (

            SpriteSheet::from_files(&mut draw, &mut sheets, "Character", "./assets/images/", "./assets/data/atlases/", "Character", "./assets/data/sheets/"),
            Player::new(200.0, 8.0, 50.0),
            HasGravity {},
            InteractsWithOneWay {},
            Velocity::new(0.0, 0.0),
            DynamicBody::new(50.0, 50.0, 15.0, 50.0),
            Target {}

        ));

		world.push(
		(

			Observer::new(Point { x: 600.0, y: 200.0 }, Point { x: 0.0, y: 0.0 }, Point { x: -3.0, y: 1.0 }, Point { x: -1.0, y: 1.0 } , 1),
			Cone { field: Vec::new() }

		));

        level::load_level(&mut world, &mut resources, "test", "./assets/data/levels/");

        resources.insert(draw);
        resources.insert(sheets);

        let mut schedule_builder = Schedule::builder();
     
        Snoop::schedule_early_systems(&mut schedule_builder);
        Snoop::schedule_physics_systems(&mut schedule_builder);
        Snoop::schedule_render_systems(&mut schedule_builder);
		Snoop::schedule_cleanup_systems(&mut schedule_builder);

        let schedule = schedule_builder.build();

        return Snoop { world, schedule, resources, view: View::new(Vector2f::new(0.0, 0.0), Vector2f::new(400.0, 400.0)), debug: true };

    }

    fn schedule_early_systems(schedule: &mut Builder)
    {

		player::schedule_early_systems(schedule);

		physics::schedule_early_systems(schedule);

    }

    fn schedule_physics_systems(schedule: &mut Builder)
    {

		physics::schedule_physics_systems(schedule);
		
		alarm::schedule_alarm_systems(schedule);
        codes::schedule_watcher_systems(schedule);

    }

    fn schedule_render_systems(schedule: &mut Builder)
    {

        player::schedule_animation_systems(schedule);

        schedule.add_system(sprites::update_spritesheets_system());

        camera::schedule_camera_systems(schedule);

    }

	fn schedule_cleanup_systems(schedule: &mut Builder)
	{

		physics::schedule_cleanup_systems(schedule);

	}

    fn debug_render(&mut self, window: &mut RenderWindow, time: f32)
    {

        let draw = self.resources.get::<Draw>().unwrap();

        let outline = Color::rgba(255, 255, 255, 200);

        let mut static_query = <&StaticBody>::query().filter(component::<Kinematic>() | !component::<Kinematic>());
        for chunk in static_query.iter_chunks(&mut self.world)
        {

            let mut fill = Color::rgba(255, 0, 0, 100);

            if chunk.archetype().layout().component_types().contains(&ComponentTypeId::of::<Kinematic>())
            {

                fill = Color::rgba(255, 0, 255, 100);

            }

            for body in chunk
            { 

                let rect = draw.create_rect(&Stroke::new(outline, fill, 1.0), &body.body);

                window.draw(&rect);

            }

        }

        let mut oneway_query = <&OneWayBody>::query().filter(component::<Kinematic>() | !component::<Kinematic>());
        for chunk in oneway_query.iter_chunks(&mut self.world)
        {

            let mut fill = Color::rgba(0, 255, 0, 100);

            if chunk.archetype().layout().component_types().contains(&ComponentTypeId::of::<Kinematic>())
            {

                fill = Color::rgba(0, 255, 255, 100);

            }

            for body in chunk
            { 

                let rect = draw.create_rect(&Stroke::new(outline, fill, 1.0), &body.body);

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

                let rect = draw.create_rect(&Stroke::new(outline, fill, 1.0), &Rect { x: body.x() + time * velocity.x, y: body.y() + time * velocity.y, width: body.body.width, height: body.body.height });
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

    fn handle_input(&mut self, input: &Input)
    {

        self.resources.remove::<InputCommand>();

        let command = InputCommand { left: input.contains(Key::A), right: input.contains(Key::D), up: input.contains(Key::W), down: input.contains(Key::S), jump: input.contains(Key::SPACE) }; 

        self.resources.insert(command);

    }

    fn update(&mut self, _timestep: i32) -> bool
    {

        self.schedule.execute(&mut self.world, &mut self.resources);

        return true

    }

    fn render(&mut self, window: &mut RenderWindow, time: f32)
    {

        //Normal render block, subscoped so that immutable borrows do not interfere with
        //mutable borrow required by debug render block
        {

            let camera = self.resources.get::<Camera>().unwrap();
            self.view.set_center(Vector2f::new(camera.x, camera.y));

            let draw = self.resources.get::<Draw>().unwrap();
            let sheets = self.resources.get::<Sheets>().unwrap();

            let mut sprite_query = <(&SpriteSheet, &DynamicBody, &Velocity)>::query().filter(component::<Target>() | !component::<Target>());
            for chunk in sprite_query.iter_chunks(&self.world)
            {

                let target = chunk.archetype().layout().component_types().contains(&ComponentTypeId::of::<Target>());

                for (sheet, body, velocity) in chunk
                {

                    let rect = sheet.get_src(&draw, sheet.get_sheet(&sheets));
                    let offset = sheet.get_offset(sheet.get_sheet(&sheets));

                    let x;

                    if body.left
                    {

                        x = body.right() - offset.x + (time * velocity.x);

                    }
                    else
                    {

                        x = body.x() + offset.x + (time * velocity.x);

                    }

                    let y = body.y() + offset.y + time * velocity.y;

                    let sprite = draw.create_sprite(sheet.texture, rect, &Rect { x: x, y: y, width: rect.width, height: rect.height }, body.left);

                    if target
                    {

                        let mut center = Vector2f::new(camera.x + time * velocity.x, camera.y + time * velocity.y);

                        if camera.lock_x
                        {

                            center.x = camera.x;

                        }

                        if camera.lock_y
                        {

                            center.y = camera.y;

                        }

                        self.view.set_center(center);

                    }

                    window.draw(&sprite);

                }

            }

			let cone_stroke = Stroke::new(Color::rgba(255, 255, 255, 0), Color::rgba(255, 255, 255, 50), 0.0);
			let mut cone_query = <&Cone>::query();
			for cone in cone_query.iter(&self.world)
			{

				for triangle in cone.field.iter()
				{

					let tri = draw.create_triangle(&cone_stroke, &triangle);
					
					window.draw(&tri);

				}

			}

            window.set_view(&self.view);

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
