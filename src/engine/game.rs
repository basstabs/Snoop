use sfml::window::Event;
use sfml::graphics::{Color, RenderTarget, RenderWindow};

pub struct Timestep
{

    pub step: i32

}

pub trait State
{

    fn initialize(&mut self);

    //Returns false if the state has stopped running
    //True normally
    fn update(&mut self, timestep: i32) -> bool;

    fn render(&mut self, window: &mut RenderWindow, time: f32);

    //Whether or not the state should be updated
    fn active(&self) -> bool;

}

pub struct Game
{

    //Game data
    states: Vec<Box<dyn State>>,

    //Backend data
    window: RenderWindow

}

impl Game
{

    pub fn new(window: RenderWindow) -> Game
    {

        return Game { states: Vec::new(), window: window };

    }

    pub fn push_state(&mut self, mut state: Box<dyn State>)
    {

        state.initialize();

        self.states.push(state);

    }

    //Returns true if the application should close
    //False normally
    pub fn process(&mut self) -> bool
    {

        while let Some(e) = self.window.poll_event()
        {

            match e
            {

                Event::Closed => { return true; },
                _ => {}


            };

        }

        return false;

    }

    //Returns false if the game has stopped running
    //True normally
    pub fn update(&mut self, timestep: i32) -> bool
    {

        let mut running = false;

        for state in self.states.iter_mut().filter(|s| { return s.active(); })
        {

            running = running || state.update(timestep);

        }

        return running;

    }

    pub fn render(&mut self, time: f32)
    {

        self.window.clear(Color::BLACK);

        for state in self.states.iter_mut()
        {

            state.render(&mut self.window, time);

        }

        self.window.display();

    }

}
