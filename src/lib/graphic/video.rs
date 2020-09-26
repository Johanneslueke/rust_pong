use sdl2::pixels::*;
use std::convert::TryFrom;

pub type Canvas = sdl2::render::Canvas<sdl2::video::Window>;


pub struct VideoContext{
    sdl_context : Option<sdl2::Sdl>,
    video_subsystem : Option<sdl2::VideoSubsystem>,
    pub canvas      : Option<Canvas>,
    pub event_pump  : Option<sdl2::EventPump>
}

pub trait Drawable{
    fn draw(&self, canvas : &mut Canvas) -> Result<(),String>;
}

pub trait Updateable{
    fn update(&mut self);
}


impl VideoContext{

    pub fn new() -> Result<Self,String>{
        let sdl_context =  sdl2::init()?;
        let video_subsystem =  sdl_context.video()?;

        Ok(
            VideoContext{
                sdl_context: Some(sdl_context),
                video_subsystem: Some(video_subsystem),
                canvas : None,
                event_pump : None
            }
        )
    }

    pub fn create_window(&mut self, width:u32, height:u32) -> Result<(),String>
    {
        let window =  self.video_subsystem.as_ref().unwrap().window("rust-sdl2 demo",
        width  , 
        height  
        )
        .position_centered()
        .build().unwrap();

        self.canvas = Some(window.into_canvas().build().unwrap());
        self.event_pump = Some(self.sdl_context.as_ref().unwrap().event_pump()?);
        Ok(())
    }
    
    pub fn set_draw_color(&mut self, rgb:u32) {
        let format = PixelFormat::try_from(PixelFormatEnum::RGBA32).unwrap();
        self.canvas
            .as_mut()
            .unwrap()
            .set_draw_color(
                Color::from_u32(
                    &format,
                    rgb
                    )
            );
    }

    pub fn clear_screen(&mut self) {
        self.set_draw_color(0x00_00_00);
        self.canvas.as_mut().unwrap().clear();
    }

    pub fn update_screen(&mut self){
        self.canvas.as_mut().unwrap().present();
    }

    pub fn access_event_pump(&mut self) -> &mut sdl2::EventPump
    {
        self
        .event_pump
        .as_mut()
        .ok_or_else(|| panic!("no event pump available"))
        .unwrap()
    }

    pub fn access_canvas(&mut self) -> &mut Canvas{
        self
        .canvas
        .as_mut()
        .ok_or_else(|| panic!("no event canvas available"))
        .unwrap()
    }

}
