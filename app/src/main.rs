use sfml::graphics::RenderWindow;
use sfml::window::Style;

use engine::game::Game;

mod snoop;
use snoop::Snoop;

fn main()
{

    let window = RenderWindow::new((800, 600), "Snoop", Style::DEFAULT, &Default::default());

    let mut game = Game::new(window);
    game.push_state(Box::new(Snoop::new(16)));

    engine::gameloop::rungame(16, &mut game);

}
