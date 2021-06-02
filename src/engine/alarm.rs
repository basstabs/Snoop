use legion::*;
use legion::world::SubWorld;
use legion::systems::CommandBuffer;

use super::space::{Point, Triangle};
use super::physics::{DynamicBody, StaticBody, OneWayBody};

pub struct Observer
{

	location: Point,
	offset: Point,
	upper: Point,
	lower: Point,
	moved: bool

}

pub struct Cone
{

	field: Vec<Triangle>

}

pub struct Suspicious {}

impl Observer
{

	pub fn new(location: Point, offset: Point, upper: Point, lower: Point) -> Observer
	{

		return Observer { location: location + offset, offset: offset, upper: upper, lower: lower, moved: true };

	}

	pub fn shift(&mut self, location: Point)
	{

		self.location = location + self.offset;

		self.moved = true;

	}

}

#[system(for_each)]
#[read_component(StaticBody)]
#[read_component(OneWayBody)]
fn line_of_sight(observer: &mut Observer, cone: &mut Cone, world: &mut SubWorld)
{

	if observer.moved
	{

		let rays: Vec<Point> = Vec::new();

		

		observer.moved = false;

	}

}

#[system(for_each)]
#[read_component(DynamicBody)]
#[read_component(Suspicious)]
fn visual_alarm(cone: &Cone, world: &mut SubWorld, cmd: &mut CommandBuffer)
{

	let mut query = <(&DynamicBody, &Suspicious)>::query();

	for (body, _) in query.iter(world)
	{

		for triangle in cone.field.iter()
		{

			if triangle.intersects_rectangle(&body.body)
			{

				

			}						

		}

	}

}
