use std::ops::{Add, Sub};

use serde::Deserialize;

pub const FLOATING_POINT_ERROR: f32 = 0.0001;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct Point
{

    pub x: f32,
    pub y: f32

}

impl Point
{

	pub fn dot(&self, other: Point) -> f32
	{

		return self.x * other.x + self.y * other.y;

	}

	pub fn scale(&self, s: f32) -> Point
	{

		return Point { x: self.x * s, y: self.y * s };

	}

	//Assumes the angle from lower to upper is less than pi. Swap lower and upper and negate it or larger angles
	pub fn ray_between(&self, lower: &Point, upper: &Point) -> bool
	{

		//Dot product of upper rotated ccw by pi/2
		let upper_component = self.y * upper.x - self.x * upper.y;

		if upper_component > FLOATING_POINT_ERROR
		{

			return false;

		}

		//Dot product of lower rotated cw by pi/2
		let lower_component = self.x * lower.y - self.y * lower.x;

		if lower_component > FLOATING_POINT_ERROR
		{

			return false;

		}

		return true;

	}

	//Works so long as all represented angles are between from and from+pi
	pub fn sort_from_angle(rays: &mut Vec<Point>, from: Point)
	{

		rays.sort_unstable_by(|a, b|
		{

			//We want to order by angle from lower, which is the same as reverse ordering by normalized projections along lower
			//We do some algebra to avoid computing square roots for the normalization, i.e. a dot L/|a|>b dot L/|b| if and only if
			// a dot L*|a dot L|*|b|^2 > b dot L * |b dot L| * |a|^2
			let a_dot_f = from.dot(*a);
			let lhs = a_dot_f.abs() * a_dot_f * (b.x * b.x + b.y * b.y);

			let b_dot_f = from.dot(*b);
			let rhs = b_dot_f.abs() * b_dot_f * (a.x * a.x + a.y * a.y);

			return rhs.partial_cmp(&lhs).unwrap();

		});

	}

}

impl Add for Point
{

	type Output = Point;

	fn add(self, other: Point) -> Point
	{

		return Point { x: self.x + other.x, y: self.y + other.y };

	}

}

impl Sub for Point
{

	type Output = Point;

	fn sub(self, other: Point) -> Point
	{

		return Point { x: self.x - other.x, y: self.y - other.y };

	}

}

pub struct Segment
{

	pub start: Point,
	pub end: Point

}

impl Segment
{

	pub fn new(start: Point, end: Point) -> Segment
	{

		if start.x == end.x && start.y == end.y
		{

			panic!("Cannot create line segment between point and itself.");

		}

		return Segment { start, end };

	}

	pub fn raycast(&self, location: Point, ray: Point) -> Option<f32>
	{

		//Ensure the ray can be raycast
		if ray.x == 0.0 && ray.y == 0.0
		{

			panic!("Cannot raycast the zero vector");

		}

		let rise = self.end.y - self.start.y;
		let run = self.end.x - self.start.x;

		let denominator = rise * ray.x - run * ray.y;
		if denominator.abs() < FLOATING_POINT_ERROR //The ray and the segment are parallel, so there is no intersection to find
		{

			return None;

		}

		let segment_param = (location.y * ray.x + self.start.x * ray.y - location.x * ray.y - self.start.y * ray.x) / denominator;
		if segment_param < -FLOATING_POINT_ERROR || segment_param > 1.0 + FLOATING_POINT_ERROR //The lines intersect outside the segment, so there is no intersection
		{

			return None;

		}

		let ray_param;
		if ray.x == 0.0
		{

			ray_param = (self.start.y - location.y + rise * segment_param) / ray.y;

		}
		else
		{

			ray_param = (self.start.x - location.x + run * segment_param) / ray.x;

		}

		if ray_param < -FLOATING_POINT_ERROR //The opposite of the ray intersects the segment, not the ray itself
		{

			return None;

		}

		return Some(ray_param);

	}

}

#[derive(Deserialize)]
pub struct Rect
{

    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32

}

impl Rect
{

    pub fn translate(&mut self, (vx, vy): (f32, f32))
    {

        self.x += vx;
        self.y += vy;

    }

    pub fn right(&self) -> f32
    {

        return self.x + self.width;

    }

    pub fn bottom(&self) -> f32
    {

        return self.y + self.height;

    }

	pub fn to_polygon(&self) -> Polygon
	{

		let mut vertices: Vec<Point> = Vec::new();

		vertices.push(Point { x: self.x, y: self.y });
		vertices.push(Point { x: self.width, y: 0.0 });
		vertices.push(Point { x: 0.0, y: self.height });
		vertices.push(Point { x: -self.width, y: 0.0 });
		vertices.push(Point { x: 0.0, y: -self.height });

		return Polygon { vertices };

	}

    pub fn intersects(r1: &Rect, r2: &Rect) -> bool
    {

        if r1.x >= r2.right()
        {

            return false;

        }

        if r1.right() <= r2.x
        {

            return false;

        }

        if r1.y >= r2.bottom()
        {

            return false;

        }

        if r1.bottom() <= r2.y
        {

            return false;

        }

        return true; 

    }

    //r1 moves using velocity (vx, vy), r2 is stationary
    pub fn collides(r1: &Rect, r2: &Rect, (vx, vy): (f32, f32)) -> (f32, f32)
    {

        let mut correction = (0.0, 0.0);

        if r1.x >= r2.right()
        {

            return correction;

        }

        if r1.right() <= r2.x
        {

            return correction;

        }

        if r1.y >= r2.bottom()
        {

            return correction;

        }

        if r1.bottom() <= r2.y
        {

            return correction;

        }

        //There was a collision, so we need to fix it
        //Set the x position for a rightward-moving r1 to match the left edge of r2
        if vx >= 0.0
        {

            correction.0 = r2.x - r1.right();

        }
        else //Set the x position for a leftward-moving r1 to match the right edge of r2
        {

            correction.0 = r2.right() - r1.x;

        }

        //Set the y position for a downward-moving r1 to match the top edge of r2
        if vy >= 0.0
        {

            correction.1 = r2.y - r1.bottom();

        }
        else //Set the y position for an upward moving r1 to match the bottom edge of r2
        {

            correction.1 = r2.bottom() - r1.y;

        }

        //Determine which correction to use: We want to use the smaller of the two correction
        //directions, unless the other is zero.
        if correction.0.abs() >= correction.1.abs() && correction.1 != 0.0 && !(correction.1 > 0.0 && vy >= -FLOATING_POINT_ERROR)
        {

            correction.0 = 0.0;

        }
        else
        {

            correction.1 = 0.0;

        }

        return correction;

    }

}

#[derive(Debug)]
pub struct Triangle
{

	pub vertices: [Point; 3]

}

impl Triangle
{

	pub fn new(a: Point, b: Point, c: Point) -> Triangle
	{

		return Triangle { vertices: [a, b, c] };

	}

	fn to_polygon(&self) -> Polygon
	{

		let mut vertices: Vec<Point> = Vec::new();

		vertices.push(self.vertices[0]);
		vertices.push(self.vertices[1] - self.vertices[0]);
		vertices.push(self.vertices[2] - self.vertices[1]);
		vertices.push(self.vertices[0] - self.vertices[2]);

		return Polygon { vertices };

	}

	pub fn intersects_rectangle(&self, rectangle: &Rect) -> bool
	{

		let triangle = self.to_polygon();
		let rect = rectangle.to_polygon();

		return Polygon::sat(&triangle, &rect);

	}
	
}

pub struct Polygon
{

	//First vertex in world coordinates, then vectors to get to next vertex. n+1 vectors for an n-gon
	pub vertices: Vec<Point>

}

impl Polygon
{

	//Project the polygon along a vector and return its (min, max)
	pub fn project(&self, vector: Point) -> (f32, f32)
	{

		let mut vertex = self.vertices[0];

		let mut min = vector.dot(vertex);
		let mut max = min;
		
		for i in 1..(self.vertices.len() - 1)
		{

			vertex = vertex + self.vertices[i];

			let projection = vector.dot(vertex);

			if projection < min
			{

				min = projection;

			}
			else if projection > max
			{

				max = projection;

			}

		}

		return (min, max);

	}

	//Returns true if the two convex polygons are intersecting. Does NOT check that they are convex, nor that the sum from 0..len of vertices ends at vertices[0]
	pub fn sat(p1: &Polygon, p2: &Polygon) -> bool
	{

		let projection_test = |v: Point| -> bool
		{

			let normal = Point { x: -v.y, y: v.x };

			let (min1, max1) = p1.project(normal);
			let (min2, max2) = p2.project(normal);

			return min1 >= max2 || min2 >= max1

		};

		//Check p1's normals
		for i in 1..p1.vertices.len()
		{

			if projection_test(p1.vertices[i])
			{

				return false;

			}		

		}

		//Check p2's normals
		for i in 1..p2.vertices.len()
		{

			if projection_test(p2.vertices[i])
			{

				return false;

			}

		}

		return true;

	}

}

#[cfg(test)]
mod tests
{

	use super::*;

	#[test]
	fn angle_sort()
	{

		let mut rays = vec![Point { x: 1.0, y: 1.0 }, Point { x: 0.0, y: 1.0 }, Point { x: 2.0, y: 4.0 }, Point { x: -1.0, y: 1.0 }, Point { x: 1.0, y: 0.2 } ];
		Point::sort_from_angle(&mut rays, Point { x: 1.0, y: 0.0 });

		assert_eq!(rays, vec![Point { x: 1.0, y: 0.2 }, Point { x: 1.0, y: 1.0 }, Point { x: 2.0, y: 4.0 }, Point { x: 0.0, y: 1.0 }, Point { x: -1.0, y: 1.0 }]);

	}

	#[test]
	fn ray_between()
	{

		let ray1 = Point { x: 2.5, y: 0.0 };
		let ray2 = Point { x: 0.0, y: 1.0 };
		let ray3 = Point { x: -1.0, y: 2.0 };
		let ray4 = Point { x: -1.0, y: -1.1 };
		let ray5 = Point { x: 3.7, y: -2.0 };
		let ray6 = Point { x: -2.0, y: 0.0 };
		let ray7 = Point { x: 0.0, y: -30.0 };
		let ray8 = Point { x: 10.0, y: 1.0 };

		assert!(ray8.ray_between(&ray1, &ray2));
		assert!(ray6.ray_between(&ray3, &ray4));
		assert!(ray5.ray_between(&ray7, &ray1));
		assert!(ray4.ray_between(&ray3, &ray5));

		assert!(!ray3.ray_between(&ray1, &ray2));
		assert!(!ray1.ray_between(&ray3, &ray4));
		assert!(!ray2.ray_between(&ray7, &ray1));
		assert!(!ray8.ray_between(&ray3, &ray5));

	}

	#[test]
	fn rectangle_intersection()
	{

		let rect1 = Rect { x: 0.0, y: 0.0, width: 50.0, height: 100.0 };
		let rect2 = Rect { x: 10.0, y: 10.0, width: 800.0, height: 400.0 };
		let rect3 = Rect { x: 200.0, y: 200.0, width: 100.0, height: 100.0 };

		assert!(Rect::intersects(&rect1, &rect2));
		assert!(Rect::intersects(&rect2, &rect3));
		assert!(!Rect::intersects(&rect1, &rect3));

		let poly1 = rect1.to_polygon();
		let poly2 = rect2.to_polygon();
		let poly3 = rect3.to_polygon();

		assert!(Polygon::sat(&poly1, &poly2));
		assert!(Polygon::sat(&poly2, &poly3));
		assert!(!Polygon::sat(&poly1, &poly3));

	}

	#[test]
	fn rect_triangle_intersection()
	{

		let rect1 = Rect { x: 0.0, y: 0.0, width: 100.0, height: 150.0 };
		let rect2 = Rect { x: 13.0, y: 56.0, width: 25.0, height: 25.0 };

		let triangle1 = Triangle { vertices: [ Point { x: 5.0, y: 5.0}, Point { x: 0.0, y: -5.0 }, Point { x: -5.0, y: 5.0} ] };
		let triangle2 = Triangle { vertices: [ Point { x: 20.0, y: 60.0}, Point { x: 100.0, y: 100.0}, Point { x: 0.0, y: 8.0} ] };

		assert!(triangle1.intersects_rectangle(&rect1));
		assert!(!triangle1.intersects_rectangle(&rect2));
		assert!(triangle2.intersects_rectangle(&rect1));
		assert!(triangle2.intersects_rectangle(&rect2));

	}

	#[test]
	fn raycast()
	{

		let segment1 = Segment::new(Point { x: 10.0, y: 0.0 }, Point { x: 0.0, y: 0.0 });
		let segment2 = Segment::new(Point { x: 0.0, y: 0.0 }, Point { x: 0.0, y: 10.0 });
		let segment3 = Segment::new(Point { x: 1.0, y: 9.0 }, Point { x: 2.0, y: 5.0});

		let cast1 = segment1.raycast(Point { x: 1.0, y: -1.0 }, Point { x: 0.0, y: 1.0 }).unwrap();
		let cast2 = segment2.raycast(Point { x: 3.0, y: 4.0 }, Point { x: -30.0, y: 0.0 }).unwrap();
		let cast3 = segment3.raycast(Point { x: 0.0, y: 9.0 }, Point { x: 1.0, y: -1.0 }).unwrap();

		assert_eq!(1.0, cast1);
		assert_eq!(1.0 / 10.0, cast2);
		assert_eq!(4.0 / 3.0, cast3);

		let cast4 = segment1.raycast(Point { x: 2.0, y: 0.0 }, Point { x: 1.0, y: 0.0 });
		let cast5 = segment2.raycast(Point { x: 1.0, y: 11.0 }, Point { x: -1.0, y: 0.0 });
		let cast6 = segment3.raycast(Point { x: 0.0, y: 9.0 }, Point { x: -1.0, y: 1.0 });

		assert!(cast4.is_none());
		assert!(cast5.is_none());
		assert!(cast6.is_none());

	}

}
