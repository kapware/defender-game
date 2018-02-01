extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use graphics::clear;
use piston::input::*;
use piston::window::Window;
use rand::Rng;

mod color;
pub mod config;
mod geom;

mod models;
use models::{GameObject};
use models::bullet::Bullet;
use models::enemy::Enemy;
use models::player::Player;

const UNIT_MOVE: f64 = 5.0;

struct GameState {
    debug_mode: bool,
    fire_bullets: bool,
}

pub struct App {
    pub window: config::GraphicsConfig,
    player: Player,
    enemy: Enemy,
    bullets: Vec<Bullet>,

    // Game-wide events
    state: GameState,
}

impl App {
    pub fn new(window: config::GraphicsConfig) -> App {
        let size = window.settings.size();

        let (x, y) = ((size.width / 2) as f64,
                      (size.height / 2) as f64);

        let player = Player::new(x, y, 20.0);

        // Choose a random spot in the window to render the enemy.
        let mut rng = rand::thread_rng();

        let enemy = Enemy::new(
            rng.gen_range(0.0, size.width as f64),
            rng.gen_range(0.0, size.height as f64),
            20.0
        );

        let state = GameState {
            fire_bullets: false,
            debug_mode: false
        };

        return App {
            window,
            player,
            enemy,
            bullets: Vec::new(),
            state
        };
    }

    pub fn input(&mut self, button: &Button, is_press: bool) {
        // Zeroes out movement is a move button is released.
        let multiplier = if is_press { 1.0 } else { 0.0 };

        if let Button::Keyboard(key) = *button {
            match key {
                // TODO: Setup movement as a player state and handle addition
                // in update?
                Key::Up => self.player.move_y = -UNIT_MOVE * multiplier,
                Key::Down => self.player.move_y = UNIT_MOVE * multiplier,
                Key::Left => self.player.move_x = -UNIT_MOVE * multiplier,
                Key::Right => self.player.move_x = UNIT_MOVE * multiplier,
                Key::Space => {
                    // TODO: Setup a cooldown for firing? so we can just hold
                    // down the space button?
                    if is_press {
                        self.state.fire_bullets = true;
                    }
                },
                // Toggle debug mode.
                Key::D => {
                    if is_press {
                        self.state.debug_mode = !self.state.debug_mode;
                        println!("Debug mode: {}", self.state.debug_mode);
                    }
                },
                _ => (),
            }
        }
    }

    // Render stuff on the screen.
    pub fn render(&mut self, args: &RenderArgs) {
        // Grab list of objects to render.
        let bullets = &self.bullets;
        let enemy = &self.enemy;
        let player = &self.player;
        let debug_mode = self.state.debug_mode;
        // Render stuff.
        self.window.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(color::BLACK, gl);

            // Render objects
            for bullet in bullets.iter() {
                bullet.render(&c, gl);
            }
            enemy.render(&c, gl);
            player.render(&c, gl);

            if debug_mode {
                player.render_dbg(&c, gl);
            }
        });
    }

    // Update any animation, etc.
    // dt is the delta since the last update.
    pub fn update(&mut self, args: &UpdateArgs) {
        // Handle game events
        if self.state.fire_bullets {
            self.state.fire_bullets = false;
            self.bullets.push(
                Bullet::new(self.player.pos.x, self.player.pos.y)
            );
        }

        for bullet in self.bullets.iter_mut() {
            // Animate bullet
            bullet.update(args.dt);
            // Did bullet collide with enemy?
            if bullet.collides(&self.enemy) {
                // Destroy bullet
                bullet.ttl = 0.0;
            }
        }
        // Remove bullets that have outlived their TTL
        self.bullets.retain(|bullet| bullet.ttl > 0.0);

        // Update player & enemies
        self.player.update(args.dt);
        self.enemy.update(args.dt);
    }
}