use crate::enemy::Enemy;
use ggez::graphics::Color;

// Bullet structure
pub struct Bullet {
    pub x: f32,        
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub is_active: bool, // If its false, i had to destroy it
}

impl Bullet {
    // Contructor
    pub fn new(x: f32, y: f32, speed: f32) -> Bullet {
        let width = 5.0;
        let height = 10.0;
        Bullet { x, y,width,height, speed, is_active: true }
    }

    // Update bullet position
    pub fn update(&mut self) {
        if self.is_active {
            self.y -= self.speed;
            // Si el disparo sale de la pantalla, lo desactivamos
            if self.y < 0.0 {
                self.is_active = false;
            }
        }
    }

    // Draw the bullet 
    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        let red = Color::new(1.0, 0.0, 0.0, 1.0);
        if self.is_active {
            let rect = ggez::graphics::Mesh::new_rectangle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                ggez::graphics::Rect::new(self.x, self.y, self.width, self.height),
                red
            )?;
            ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;
        }
        Ok(())
    }

    // Check if collides with an enemy
    pub fn collides_with(&self, enemy: &Enemy) -> bool {
        // Define the right and bottom boundaries of the bullet
        let bullet_right = self.x + self.width;
        let bullet_bottom = self.y + self.height;
        
        // Define the right and bottom boundaries of the enemy.
        let enemy_right = enemy.x + enemy.image.width() as f32 * enemy.scale_factor;
        let enemy_bottom = enemy.y + enemy.image.height() as f32 * enemy.scale_factor;

        // Check collision comparing the boundaries
        if self.x < enemy_right && bullet_right > enemy.x && self.y < enemy_bottom && bullet_bottom > enemy.y {
            #[cfg(debug_assertions)]{
                println!("Shoot reached");
            }
            return true;
        }

        false
    }
}