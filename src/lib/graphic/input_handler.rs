use crate::command::NOPCommand;
use crate::command::Command;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;


pub fn handler (events :&mut sdl2::EventPump, actions : &mut Vec<&dyn Command>)-> Result<bool,String>{
    for event in events.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Ok(false);
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                actions.push(&NOPCommand{});
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                actions.push(&NOPCommand{});
            }
            _ => {}
        }
    }
    Ok(true)
}