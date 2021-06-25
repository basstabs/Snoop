use sfml::system::Clock;

use super::game::Game;

pub fn rungame(step: i32, game: &mut Game)
{

    //Update using the same step each frame
    let timestep = step as f32;

    //Initialize time variables
    let timer = Clock::start();
    let mut previous = timer.elapsed_time().as_milliseconds();
    let mut lag = 0;

    let mut running = true;

    while running
    {

        //Calculate time values for this loop
        let current = timer.elapsed_time().as_milliseconds();
        let elapsed = current - previous;

        //Time maintenance for the next loop
        previous = current;
        lag += elapsed;

        //Process input, which returns true if the game should quit
        if game.process()
        {

            break;

        }

        //Update the game loop as many times as necessary
        while lag >= step && running
        {

            running = game.update(step);

            lag -= step;

        }

        game.render((lag as f32) / timestep);

    }

}
