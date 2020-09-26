use crate::entity::Entity; 
use crate::command::Command; 

pub fn handler (
    actors : &mut Vec< &mut dyn Entity >,
    actionlist : &mut Vec<&dyn Entity>) -> Result<(),String>{


    for current in 0..actors.len(){

        /*for action in actionlist{
            actors[current].do_action(&action);
        }*/

        actors[current].update();

        

        /*for other in 0..actors.len() { 
            if current == other {
                continue;
            }
            match & actors[current].clone(){
                GameObjects::Player(player)  => match &mut actors[other]{

                    GameObjects::Player(other_player) => {
                        if player.body.collision(other_player.body){
                            println!("Collision Player + Player");
                        }
                    }
                    GameObjects::Ball(ball) =>{
                        if player.body.collision(ball.body){
                            println!("Collision Player + Ball");
                            ball.invert();
                        }
                    }
                }
                _ => {}
            }
        }*/
    }
     
    Ok(())
}