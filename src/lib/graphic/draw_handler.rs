
use crate::entity::Entity;
use crate::graphic::video::Canvas; 

pub fn handler (drawables : &Vec<&mut dyn Entity>,canvas :&mut Canvas ) -> Result<(),String>{
    for player in drawables{
        player.draw(canvas)?;
    }
    Ok(())
}
