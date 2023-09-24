use ggez::event::{self, EventHandler, KeyCode};
use ggez::graphics::{self, Color, Text};
use ggez::input::keyboard;
use ggez::{Context, GameResult};
use ggez::timer;
use rand::Rng;

// Modules for different elements
mod player;
use player::Player;

mod bullet;
use bullet::Bullet;

mod enemy;
use enemy::Enemy;

const DESIRED_FPS: u32 = 60;

// Function to spawn grid of enemies
fn spawn_enemies(ctx: &mut Context, num_x: i32, num_y: i32) -> GameResult<Vec<Enemy>> {
    let (screen_width, _screen_height) = graphics::drawable_size(ctx);
    let mut enemies = Vec::new();

    // Calculate real space (0.8 = scale_factor)
    let enemy_width = (Enemy::new(0.0, 0.0, 0.0, 2.0,ctx)?.image.width() as f32 * 0.08) as f32;
    let gap_x = (screen_width - enemy_width * num_x as f32) / (num_x + 1) as f32;
    let gap_y = 50.0; // Puedes ajustar este valor si quieres cambiar el espaciado vertical

    let mut rng = rand::thread_rng();
    let horizontal_speed = if rng.gen_bool(0.5) { 2.0 } else { -2.0 }; // rng for first way to move

    for row in 0..num_x { // Row 
        for column in 0..num_y { // Columns
            let x = (row as f32 + 1.0) * gap_x + row as f32 * enemy_width;
            let y = -1.0 * gap_y + column as f32 * gap_y;

            let enemy = Enemy::new(x, y, 1.5, horizontal_speed, ctx)?;
            enemies.push(enemy);
        }
    }

    #[cfg(debug_assertions)]
    println!("Generated {} enemies", enemies.len());

    Ok(enemies)
}


// Game states
enum GameState {
    Playing,
    Paused,
    GameOver,
}

// Game struct
struct Game {
    player: Player,
    bullets: Vec<Bullet>,
    enemys: Vec<Enemy>,
    background: ggez::graphics::Image,
    round: i32,
    score: i32,
    last_shot_time: std::time::Duration,
    state: GameState,
    lifes: i32,
}

// Game implement 
impl Game {
    // Function to reset the game
    fn reset_game(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.enemys.clear();
        self.bullets.clear();
        self.round = 0;
        self.lifes = 3;
        self.score = 0;
        self.player = new_player(ctx)?;
        self.state = GameState::Playing;

        Ok(())
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // Limited to 60 fps
        while !timer::check_update_time(ctx, DESIRED_FPS){
            return  Ok(());
        }

        // Debug buttons
        #[cfg(debug_assertions)]{
            // Show enemies len
            if keyboard::is_key_pressed(ctx, KeyCode::D) {
                println!("Actually {}",self.enemys.len());
            }
            // Create new enemies grid
            if keyboard::is_key_pressed(ctx, KeyCode::O) {
                self.enemys = spawn_enemies(ctx, 6, 2)?;
            }
        }

        match self.state {
            GameState::Playing => {
                
                // Press 'P' for pause
                if keyboard::is_key_pressed(ctx, KeyCode::P) {
                    self.state = GameState::Paused;
                }

                // Update player
                self.player.update(ctx);

                // Shoot logic + cooldowns   
                let current_time = ggez::timer::time_since_start(ctx);
                let shot_cooldown = std::time::Duration::from_millis(300); // 300ms as cooldown
                
                if keyboard::is_key_pressed(ctx, KeyCode::Space) && current_time - self.last_shot_time > shot_cooldown
                {
                    let bullet = self.player.shoot();
                    self.bullets.push(bullet);
                    self.last_shot_time = current_time;
                }

                // Bullet logic
                for bullet in &mut self.bullets {
                    // Check if still active 
                    if bullet.is_active {
                        bullet.update();

                        // Check for enemys collision
                        for enemy in &mut self.enemys {
                            if bullet.collides_with(enemy) {
                                bullet.is_active = false;
                                enemy.is_dead = true;
                                self.score += 10;
                                break;
                            }
                        }
                    }
                }

                // Enemy logic
                for enemy in &mut self.enemys {
                    if !enemy.is_dead{

                        if enemy.y >= (ggez::graphics::drawable_size(ctx).1)
                        {
                            enemy.is_dead = true; 
                            println!("-1 life less to {}",self.lifes);
                            self.lifes -= 1;
                            
                            // If more than 3 touch, change state to GameOver
                            if self.lifes < 1 {
                                self.state = GameState::GameOver
                            }
                        }
                        
                        enemy.update(ctx);
                    }
                    // If touch bottom screen

                }

                // Cleanup inactive bullets and enemies
                self.bullets.retain(|bullet| bullet.is_active);
                self.enemys.retain(|enemy| !enemy.is_dead);


                // No more enemys logic
                if self.enemys.len() == 0 {
                    
                    #[cfg(debug_assertions)]
                    println!("Enemys down");

                    self.round += 1; // Increase round number
                    self.lifes = 3; // Reset lifes
                    
                    let increased_speed = 0.2 + 0.1 * self.round as f32; // Increase speed
                    self.bullets = Vec::new(); // deleting all bullets
                    
                    // Setting the number of the next wave. Max: 9 * 3
                    let mut column = 3 * self.round;
                    let mut row = 1 * self.round;
                    if self.round > 3 {
                        column = 9;
                        row = 3;
                    }
                    #[cfg(debug_assertions)]
                    {
                    println!("New enemy grid. columns {} & rows {} ", column,row);
                    }
                    // Setting new enemies (with new speed)
                    self.enemys = spawn_enemies(ctx, column, row)?;
                    for enemy in &mut self.enemys {
                        enemy.speed = increased_speed;
                    }
                }
            }
            GameState::Paused => {
                // Resume Game
                if keyboard::is_key_pressed(ctx, KeyCode::P) {
                    self.state = GameState::Playing;
                }
            }
            GameState::GameOver => {
                // Reset game on 'R' press
                if keyboard::is_key_pressed(ctx, KeyCode::R) {
                    self.reset_game(ctx)?;
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        match self.state {
            GameState::Playing => {
                // Clear screen + background
                ggez::graphics::clear(ctx, Color::new(255.0, 255.0, 255.0, 1.0));
                ggez::graphics::draw(ctx, &self.background, ggez::graphics::DrawParam::new())?;

                // Draw bullets
                for bullet in &mut self.bullets {
                    bullet.draw(ctx)?;
                }

                // Draw enemies
                for enemy in &mut self.enemys {
                    enemy.draw(ctx)?;
                }

                // Score text
                let score_text = Text::new(format!("Puntuación: {}", self.score));
                graphics::draw(
                    ctx,
                    &score_text,
                    (mint::Point2 { x: 10.0, y: 10.0 }, Color::WHITE),
                )?;

                // Round text
                let round_text = Text::new(format!("Ronda: {}", self.round));
                graphics::draw(
                    ctx,
                    &round_text,
                    (mint::Point2 { x: 10.0, y: 30.0 }, Color::WHITE),
                )?;

                // Lifes text
                let life_text = Text::new(format!("Vidas: {}", self.lifes));
                graphics::draw(
                    ctx,
                    &life_text,
                    (mint::Point2 { x: 10.0, y: 50.0 }, Color::WHITE),
                )?;

                // Draw player
                self.player.draw(ctx)?;
            }
            GameState::Paused => {
                let (screen_width, screen_height) = ggez::graphics::drawable_size(&ctx);

                // Setting middle text
                let paused_text = Text::new("PAUSED");
                graphics::draw(
                    ctx,
                    &paused_text,
                    (
                        mint::Point2 {
                            x: screen_width / 2.0,
                            y: screen_height / 2.0,
                        },
                        Color::WHITE,
                    ),
                )?;
            }
            GameState::GameOver => {
                let (screen_width, screen_height) = ggez::graphics::drawable_size(&ctx);

                // Setting middle text
                let gameover = Text::new("GAME OVER");
                graphics::draw(
                    ctx,
                    &gameover,
                    (
                        mint::Point2 {
                            x: screen_width / 2.0,
                            y: screen_height / 2.0,
                        },
                        Color::WHITE,
                    ),
                )?;

                // Setting score text
                let score_text = Text::new(format!("Score: {}", self.score));
                graphics::draw(
                    ctx,
                    &score_text,
                    (
                        mint::Point2 {
                            x: screen_width / 2.0,
                            y: (screen_height / 2.0) + 50.0,
                        },
                        Color::WHITE,
                    ),
                )?;
            }
        }

        return ggez::graphics::present(ctx);
    }
}

// Create new player instance
fn new_player(ctx: &mut Context) -> Result<Player, ggez::GameError> {
    let (screen_width, screen_height) = ggez::graphics::drawable_size(&ctx);

    let player_width = 50.0;
    let player_height = 50.0;
    let player_speed = 15.0;

    let player_x = (screen_width - player_width) / 2.0;
    let player_y = screen_height - player_height - 50.0;

    Player::new(player_x, player_y, player_speed, ctx)
}

fn main() -> GameResult<()> {
    // Main game setup
    let cb = ggez::ContextBuilder::new("Space invaders", "Mestre")
        .add_resource_path(std::path::Path::new("resources/"));
    let (mut ctx, events_loop) = cb.build()?;

    let player = new_player(&mut ctx)?;
    let bullets: Vec<Bullet> = Vec::new();
    let enemys: Vec<Enemy> = Vec::new();
    let background = ggez::graphics::Image::new(&mut ctx, "/assets/background.png")?;
    let last_shot_time = ggez::timer::time_since_start(&ctx);

    let game = Game {
        player,
        bullets,
        enemys,
        background,
        round: 0,
        score: 0,
        lifes: 3,
        last_shot_time,
        state: GameState::Playing,
    };
    event::run(ctx, events_loop, game)
}
