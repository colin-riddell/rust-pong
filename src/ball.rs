
use coffee::graphics::{Color, Shape, Mesh, Rectangle, Frame};


pub struct Ball { // a square ball - cos pong
    x: f32,
    y: f32,
    width: f32, 
    height: f32,
    colour: Color,
}

impl Ball {
    pub fn new(x: f32, y: f32, width: f32, height: f32, colour: Color) -> Ball{
        return Ball{x:x, y:y, height:height, width:width, colour:colour}
    }

    pub fn draw(&mut self, frame: &mut Frame){

        let mut mesh = Mesh::new();
            mesh.fill(
                Shape::Rectangle(Rectangle {
                    x: self.x,
                    y: self.y,
                    width: self.width,
                    height: self.height,
                }),
                self.colour
            );
        mesh.draw(&mut frame.as_target());

        // self.mesh.draw(&mut frame.as_target());
    }

    pub fn move_right(&mut self){
        self.x = self.x + 1.0;
    }

    pub fn move_left(&mut self){
        self.x = self.x - 1.0;
    }

    pub fn print_x_pos(&mut self){
        println!("x pos is {}", self.x);
    }

    pub fn get_x(&mut self) -> f32{
        return self.x;
    }

}

