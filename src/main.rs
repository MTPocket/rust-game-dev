extern crate piston_window;
extern crate cgmath;
extern crate rustfest_game_assets;


use piston_window::*;
use cgmath::*;
use rustfest_game_assets::*;
//use std::vec;
//use std::option;
use std::cmp;

struct Player {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    rotation: Rad<f64>,
    cooldown: f64,
    energylevel: f64,
}

struct Asteroid {
    position: Point2<f64>,
    velocity: Vector2<f64>,
    rotation: Rad<f64>,
}

struct Bullet {
    position: Point2<f64>,
    velocity: Vector2<f64>,
}

#[derive(Default)]
struct ControllerState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    shootybang: bool,
}

struct GameState {
    players: Vec<Player>,
    asteroids: Vec<Asteroid>,
    bullets: Vec<Bullet>,
}

impl Player {
    fn shoot<'a>(&mut self, energy: f64) -> Result<f64, &'a str>{
        if self.cooldown <= 0. {
//            let e = cmp::min(energy, self.energylevel);
            let e;
            if energy.gt(&self.energylevel){
                e = self.energylevel;
            }
            else {
                e = energy;
            }
            self.energylevel -= e;
            return Ok(e);
        }
        else {
            return Err("You're not ready to shoot again yet!");
        }
    }
}

fn spawn_bullet(mut s: GameState, position: Point2<f64>, velocity: Vector2<f64>) {
    s.bullets.push(Bullet{
        position: position,
        velocity: velocity,
    })
}


fn main() {

    let mut window: PistonWindow = WindowSettings::new("Big budget triple A masterpiece", [720, 720])
        .exit_on_esc(true)
        .vsync(true)
        .decorated(true)
        .build()
        .expect("OpenGL can't be instantiated");

    let mut controller = ControllerState::default();
    let mut gamestate = GameState {
        asteroids: Vec::new(),
        bullets: Vec::new(),
        players: Vec::new()
    };
    let mut player = Player {
        position: Point2 {x: 0., y: 0.},
        velocity: Vector2::zero(),
        rotation: Rad(0.),
        cooldown: 0.,
        energylevel: 100.,
    };
//    let &mut player_ref = player;

    while let Some(event) = window.next() {
        event.update(|&UpdateArgs{ dt }| { // lambdas are called closures in rust.
            let acceleration_factor = 5.0;
            let direction = Basis2::from_angle(player.rotation)
                .rotate_vector(Vector2::unit_y());

            if controller.up {
                player.velocity += acceleration_factor * direction * dt;
            } else if controller.down {
                player.velocity -= 3. * player.velocity * dt;
                if player.velocity.magnitude() < 0.05 {
                    player.velocity -= player.velocity;
                }
//                player.velocity -= acceleration_factor * direction * dt;
            }

            if controller.left {
                player.rotation += Rad(3.)*dt;
            } else if controller.right {
                player.rotation -= Rad(3.)*dt;
            }

            if controller.shootybang {
                let r = player.shoot(200.);
                match r {
                    Ok(_f64) => {
                        player.cooldown = 10.;
                        println!("Yay");

                    }
                    Err(_str) => {println!("Boo {}", player.cooldown)}
                }
            }

            player.position += player.velocity * dt;
            player.energylevel += 100. * dt;
            player.cooldown -= 100. * dt;
            if player.cooldown < 0. {
                player.cooldown = 0.;
            }

            for bullet in gamestate.bullets.iter_mut() {
                bullet.position += bullet.velocity * dt;
            }
        });

        event.button(|ButtonArgs{ button, state, ..}| {
            use Button::*;

            match button{
                Keyboard(Key::W) => {
                    controller.up = state == ButtonState::Press;
                }
                Keyboard(Key::S) => {
                    controller.down = state == ButtonState::Press;
                }
                Keyboard(Key::A) => {
                    controller.left = state == ButtonState::Press;
                }
                Keyboard(Key::D) => {
                    controller.right = state == ButtonState::Press;
                }
                Keyboard(Key::Space) => {
                    controller.shootybang = state == ButtonState::Press;
                }
                _ => {}
            }
        });

        window.draw_2d(&event, |_, graphics| {
            clear([0.2,0.2,0.2,1.], graphics);
            polygon(
                [1., 0.5, 0.5, 1.],
                PLAYER,
                math::identity()
                    .scale(0.1, 0.1)
                    .trans(player.position.x, player.position.y)
                    .rot_rad(player.rotation.0),
                graphics,
            );

        });
    }

}
