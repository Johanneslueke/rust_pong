


pub trait Command{
    fn execute(&self);
}

pub struct NOPCommand;

impl Command for NOPCommand{
    fn execute(&self){

    }
}