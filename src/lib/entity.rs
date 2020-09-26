
use crate::physic::position::Position;
use crate::physic::bounding_box::BoundingBox;
use sdl2::pixels::Color;
use crate::graphic::video::Canvas;
use crate::graphic::video::Drawable;
use crate::graphic::video::Updateable;

pub trait Entity : Drawable+Updateable{
    fn get_bounding_box(&self) -> sdl2::rect::Rect;
    fn move_entity(&mut self, delta : sdl2::rect::Rect) -> (); 

    fn world_bounderies(&mut self) -> sdl2::rect::Rect {
        sdl2::rect::Rect::new(0, 0, 500, 500)
    }
}


pub struct Player{

   pub bounding_box : sdl2::rect::Rect

}

impl Updateable for Player{
    fn update(&mut self){
        
    }
}

impl Drawable for Player{
    fn draw(&self, canvas : &mut Canvas) -> Result<(),String>
    {
        canvas.set_draw_color(Color::RGB(0xff,0xff,0xff));
        canvas.fill_rect(self.bounding_box)?;
        Ok(())
    }
}

impl Entity for Player{

    fn get_bounding_box(&self) ->  sdl2::rect::Rect { todo!() }
    fn move_entity(&mut self, _:   sdl2::rect::Rect) { todo!() }

    

}

pub struct Ball{

    pub bounding_box : BoundingBox,
    pub velocity : Position
 }
 
 impl Updateable for Ball{
     fn update(&mut self){
        self.bounding_box.pos.x += self.velocity.x * 1;
        self.bounding_box.pos.y += self.velocity.y * 1;

        if self.bounding_box.pos.x < 0{
            self.bounding_box.pos.x = 0;
            self.velocity.x *= -1;
        }

        if self.bounding_box.pos.y < 0{
            self.bounding_box.pos.y = 0;
            self.velocity.y *= -1;
        }

        if self.bounding_box.pos.x + (self.bounding_box.width as i32) > self.world_bounderies().width() as i32{
            self.bounding_box.pos.x = self.world_bounderies().width() as i32 - self.bounding_box.width as i32;
            self.velocity.x *= -1;
        }

        if self.bounding_box.pos.y + (self.bounding_box.height as i32) > self.world_bounderies().height() as i32{
            self.bounding_box.pos.y = self.world_bounderies().height() as i32 - self.bounding_box.height as i32;
            self.velocity.y *= -1;
        }
     }
 }
 
 use sdl2::gfx::primitives::DrawRenderer;
 impl Drawable for Ball{
     fn draw(&self, canvas : &mut Canvas) -> Result<(),String>
     {
         canvas.set_draw_color(Color::RGB(0xff,0xff,0xff));
         canvas.filled_circle(
             self.bounding_box.pos.x as i16,
             self.bounding_box.pos.y as i16,
             self.bounding_box.width as i16,
             Color::RGB(0xff,0xff,0xff)
            )?;
         Ok(())
     }
 }
 
 impl Entity for Ball{
 
     fn get_bounding_box(&self) ->  sdl2::rect::Rect { todo!() }
     fn move_entity(&mut self, _:   sdl2::rect::Rect) { todo!() }
 
 }


