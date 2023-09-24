use ggez::GameResult;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::graphics::Image;

use crate::bullet::Bullet;

pub struct Player {
    x: f32,
    y: f32,
    speed: f32,
    image: Image,
    scale_factor: f32,
}

impl Player{

    pub fn new(x:f32, y:f32, speed:f32, ctx: &mut ggez::Context) -> GameResult<Player>{
        let image = Image::new(ctx, "/assets/player.png")?;
        let scale_factor = 0.1;
        Ok(Player{x,y,speed,image, scale_factor})
    }

    pub fn update(&mut self, ctx: &mut ggez::Context){
        let screen_width = ggez::graphics::drawable_size(ctx).0;
        let scaled_image_width = self.image.width() as f32 * self.scale_factor;

        if keyboard::is_key_pressed(ctx, KeyCode::Left) {
            self.x -= self.speed;
        }
        
        if keyboard::is_key_pressed(ctx, KeyCode::Right) {
            self.x += self.speed;
        }

        // Screen limits
        if self.x < 0.0 {
            self.x = 0.0;
        } else if self.x + scaled_image_width > screen_width {
            self.x = screen_width - scaled_image_width;
        }

    }
    
    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let scale_factor = 0.1; // Scale image
        ggez::graphics::draw(
            ctx, 
            &self.image, 
            ggez::graphics::DrawParam::new()
                .dest(mint::Point2 { x: self.x, y: self.y })
                .scale(mint::Vector2 { x: scale_factor, y: scale_factor })
        )?;
        Ok(())
    }

    pub fn shoot(&self) -> Bullet {
        Bullet::new(
            self.x + 22.0, // Little correction :)
             self.y, 5.0)
    }

}