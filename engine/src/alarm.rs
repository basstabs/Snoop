use legion::*;
use legion::world::SubWorld;
use legion::systems::Builder;

use super::codes::Codes;
use super::space::{Point, Segment, Triangle};
use super::physics::{DynamicBody, StaticBody, OneWayBody};

pub struct Observer
{

	location: Point,
	offset: Point,
	upper: Point,
	lower: Point,
    code: u128

}

pub struct Cone
{

	pub field: Vec<Triangle>

}

pub struct Suspicious
{

    pub code: u128

}

impl Observer
{

	pub fn new(location: Point, offset: Point, upper: Point, lower: Point, code: u128) -> Observer
	{

		return Observer { location: location + offset, offset: offset, upper: upper, lower: lower, code: code };

	}

	pub fn shift(&mut self, location: Point)
	{

		self.location = location + self.offset;

	}

}

pub struct Walls
{

	segments: Vec<Segment>

}

impl Walls
{

	pub fn new() -> Walls
	{

		return Walls { segments: Vec::new() };

	}

}
#[system]
#[read_component(StaticBody)]
#[read_component(OneWayBody)]
fn update_wall_segments(world: &mut SubWorld, #[resource] walls: &mut Walls)
{

	walls.segments.clear();

	let mut static_query = <&StaticBody>::query();

	for body in static_query.iter(world)
	{

		let rect = &body.body;

		walls.segments.push(Segment::new(Point { x: rect.x, y: rect.y }, Point { x: rect.right(), y: rect.y }));
		walls.segments.push(Segment::new(Point { x: rect.right(), y: rect.y }, Point { x: rect.right(), y: rect.bottom() }));
		walls.segments.push(Segment::new(Point { x: rect.right(), y: rect.bottom() }, Point { x: rect.x, y: rect.bottom() }));
		walls.segments.push(Segment::new(Point { x: rect.x, y: rect.bottom() }, Point { x: rect.x, y: rect.y }));

	}

	let mut oneway_query = <&OneWayBody>::query();

	for body in oneway_query.iter(world)
	{

		let rect = &body.body;

		walls.segments.push(Segment::new(Point { x: rect.x, y: rect.y }, Point { x: rect.right(), y: rect.y }));
		walls.segments.push(Segment::new(Point { x: rect.right(), y: rect.y }, Point { x: rect.right(), y: rect.bottom() }));
		walls.segments.push(Segment::new(Point { x: rect.right(), y: rect.bottom() }, Point { x: rect.x, y: rect.bottom() }));
		walls.segments.push(Segment::new(Point { x: rect.x, y: rect.bottom() }, Point { x: rect.x, y: rect.y }));

	}

}

#[system(for_each)]
fn line_of_sight(observer: &mut Observer, cone: &mut Cone, #[resource] walls: &Walls)
{

	let mut rays: Vec<Point> = Vec::new();
	rays.push(observer.lower);
	rays.push(observer.upper);

	let bound_check = |ray: Point| -> bool
	{

		return ray.ray_between(&observer.lower, &observer.upper);
		
	};

	//Collect the rays we need to project
	for segment in walls.segments.iter()
	{

		//We only cast a ray to the start of the segment because each corner of each box will be the start of one of the segments			
		let ray = Point { x: segment.start.x - observer.location.x, y: segment.start.y - observer.location.y };

		if bound_check(ray)
		{

			rays.push(ray);

		}

	}

	//Sort the rays from lower to upper
	Point::sort_from_angle(&mut rays, observer.lower);
	
	//Actually create the triangles
	cone.field.clear();

	for i in 0..rays.len()-1
	{

		let mut shortest_current = 0.0;
		let mut shortest_next = 0.0;

		for segment in walls.segments.iter()
		{

			let cast_current = segment.raycast(observer.location, rays[i]);
			let cast_next = segment.raycast(observer.location, rays[i + 1]);

			if cast_current.is_some() && cast_next.is_some() && (shortest_current == 0.0 || cast_current.unwrap() < shortest_current)
			{

				shortest_current = cast_current.unwrap();
				shortest_next = cast_next.unwrap();

			}

		}

		cone.field.push(Triangle::new(observer.location, observer.location + rays[i].scale(shortest_current), observer.location + rays[i + 1].scale(shortest_next)));

	}

}

#[system(for_each)]
#[read_component(DynamicBody)]
#[read_component(Suspicious)]
fn visual_alarm(observer: &Observer, cone: &Cone, world: &mut SubWorld, #[resource] codes: &mut Codes)
{

    if !codes.contains(observer.code)
    {

	    let mut query = <(&DynamicBody, &Suspicious)>::query();

	    for (body, suspicious) in query.iter(world)
	    {

            if Codes::codes_interact(suspicious.code, observer.code)
            {

		        for triangle in cone.field.iter()
		        {

			        if triangle.intersects_rectangle(&body.body)
			        {

                        codes.insert(observer.code);
                        break;

			        }						

                }
            
		    }

	    }

    }

}

pub fn schedule_alarm_systems(schedule: &mut Builder)
{

	schedule.add_system(update_wall_segments_system());
	schedule.add_system(line_of_sight_system());
	schedule.add_system(visual_alarm_system());

}
