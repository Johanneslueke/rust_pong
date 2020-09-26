use rust_pong::physic::position::Position;
use rust_pong::physic::bounding_box::BoundingBox;
use rust_pong::entity::Ball;
use rust_pong::entity::Player;
use rust_pong::entity::Entity;
use rust_pong::command::Command;
use rust_pong::graphic::video::VideoContext;
use rust_pong::graphic::input_handler;
use rust_pong::graphic::draw_handler;
use rust_pong::graphic::update_handler;
use rust_pong::mut_shared_collection::MutSharedCollection;


use std::iter::FromIterator;


fn main() -> Result<(),String>{

    let mut window : VideoContext = VideoContext::new()?;

    window.create_window(500,500)?;
   
    

    let mut actions : Vec<&dyn Command> = Vec::new();
    let mut drawables : Vec<&mut dyn Entity> = Vec::new();
    let mut stuff : Vec<& dyn Entity> = Vec::new();

    let t : MutSharedCollection<& dyn Entity> = MutSharedCollection::from_iter(stuff.into_iter());

    let mut player = Player{
        bounding_box : sdl2::rect::Rect::new(250, 250, 100, 100)
    };

    let mut enemy = Ball{
        bounding_box : BoundingBox{
            width : 25,
            height : 25,
            pos : Position{
                x : 25,
                y : 25
            }
        },
        velocity : Position{
            x : 2,
            y : 1
        }
    };

    drawables.push(
        &mut player
    );

    drawables.push(
        &mut enemy
    );

    while input_handler::handler(
        window.access_event_pump(),
        &mut actions
    )?{

        update_handler::handler(&mut drawables,&mut stuff)?;
        window.clear_screen();

            draw_handler::handler(
                &drawables,
                window.access_canvas()
            )?;

        window.update_screen();

        

    }

    Ok(())
}
