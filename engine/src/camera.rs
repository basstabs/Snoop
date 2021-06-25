use legion::*;
use legion::systems::Builder;

use super::physics::DynamicBody;

pub struct WorldSize
{

    pub width: f32,
    pub height: f32

}

pub struct ViewSize
{

    pub width: f32,
    pub height: f32
    
}

pub struct Camera
{

    pub x: f32,
    pub y: f32,
    pub lock_x: bool,
    pub lock_y: bool

}

pub struct Target {}

#[system(for_each)]
fn world_camera(body: &DynamicBody, _target: &Target, #[resource] camera: &mut Camera)
{

    camera.x = body.x() + body.width() * 0.5;
    camera.y = body.y() + body.height() * 0.5;

}

#[system]
fn camera_bound(#[resource] camera: &mut Camera, #[resource] size: &WorldSize, #[resource] view: &mut ViewSize)
{

    camera.lock_x = false;
    camera.lock_y = false;

    if size.width > view.width
    {

        if camera.x < view.width * 0.5
        {

            camera.x = view.width * 0.5;
            camera.lock_x = true;

        }
        else if camera.x > size.width - view.width * 0.5
        {

            camera.x = size.width - view.width * 0.5;
            camera.lock_x = true;

        }

    }
    else
    {

        camera.x = size.width * 0.5;
        camera.lock_x = true;

    }

    if size.height > view.height
    {
        
        if camera.y < view.height * 0.5
        {

            camera.y = view.height * 0.5;
            camera.lock_y = true;

        }
        else if camera.y > size.height - view.height * 0.5
        {

            camera.y = size.height - view.height * 0.5;
            camera.lock_y = true;

        }

    }
    else
    {

        camera.y = size.height * 0.5;
        camera.lock_y = true;

    }

}

pub fn register_camera_resources(resources: &mut Resources, width: f32, height: f32)
{

    resources.insert(ViewSize { width, height });
    resources.insert(Camera { x: 0.0, y: 0.0, lock_x: false, lock_y: false });

}

pub fn schedule_camera_systems(schedule: &mut Builder)
{  

    schedule.add_system(world_camera_system()); 
    schedule.add_system(camera_bound_system());

}
