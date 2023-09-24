use ggez::{graphics::Image, GameResult, Context};

// Enemy struct
pub struct Enemy {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    horizontal_speed: f32,
    pub image: Image,
    pub is_dead: bool,
    pub scale_factor: f32,
}

impl Enemy {

    // Contructor
    pub fn new(x: f32, y: f32, speed: f32,horizontal_speed: f32, ctx: &mut ggez::Context) -> GameResult<Enemy> {
        let image = Image::new(ctx, "/assets/enemy.png")?;
        
        let scale_factor = 0.08;
        Ok(Enemy { x, y, speed,horizontal_speed, image, is_dead:false, scale_factor })
    }

    // Update enemy
    pub fn update(&mut self, ctx: &mut Context) {
        // Screen dimensions
        let (screen_width, _screen_height) = ggez::graphics::drawable_size(ctx);

        self.y += self.speed;  // Move to bottom

        // Update horizontal movement
        self.x += self.horizontal_speed;

        // If touch left or right reverse its horizontal speed
        if self.x < 0.0 || self.x + (self.image.width() as f32 * self.scale_factor) > screen_width {
            self.horizontal_speed = -self.horizontal_speed;
        }


    }

    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        ggez::graphics::draw(
            ctx, 
            &self.image, 
            ggez::graphics::DrawParam::new()
                .dest(mint::Point2 { x: self.x, y: self.y+35.0 }) // The +35 is to adjust image with real hitbox
                .scale(mint::Vector2 { x: self.scale_factor, y: -self.scale_factor })  // Nota el -scale_factor en y
        )?;
        Ok(())
    }
    
}
