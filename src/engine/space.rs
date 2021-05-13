use serde::Deserialize;

const FLOATING_POINT_ERROR: f32 = 0.0001;

#[derive(Deserialize)]
pub struct Point
{

    pub x: f32,
    pub y: f32

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

    pub fn intersects(r1: &Rect, r2: &Rect) -> bool
    {

        if r1.x >= r2.x + r2.width
        {

            return false;

        }

        if r1.x + r1.width <= r2.x
        {

            return false;

        }

        if r1.y >= r2.y + r2.height
        {

            return false;

        }

        if r1.y + r1.height <= r2.y
        {

            return false;

        }

        return true; 

    }

    //r1 moves using velocity (vx, vy), r2 is stationary
    pub fn collides(r1: &Rect, r2: &Rect, (vx, vy): (f32, f32)) -> (f32, f32)
    {

        let mut correction = (0.0, 0.0);

        if r1.x >= r2.x + r2.width
        {

            return correction;

        }

        if r1.x + r1.width <= r2.x
        {

            return correction;

        }

        if r1.y >= r2.y + r2.height
        {

            return correction;

        }

        if r1.y + r1.height <= r2.y
        {

            return correction;

        }

        //There was a collision, so we need to fix it
        //Set the x position for a rightward-moving r1 to match the left edge of r2
        if vx >= 0.0
        {

            correction.0 = r2.x - r1.x - r1.width;

        }
        else //Set the x position for a leftward-moving r1 to match the right edge of r2
        {

            correction.0 = r2.x + r2.width - r1.x;

        }

        //Set the y position for a downward-moving r1 to match the top edge of r2
        if vy >= 0.0
        {

            correction.1 = r2.y - r1.y - r1.height;

        }
        else //Set the y position for an upward moving r1 to match the bottom edge of r2
        {

            correction.1 = r2.y + r2.height - r1.y;

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
