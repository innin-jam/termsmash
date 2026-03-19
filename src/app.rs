use crate::app::entity::Player;
use crate::input::Input;
pub use hitbox::Hitbox;

mod hitbox;

mod entity;

pub struct App {
    player: Player,
    level: Vec<Hitbox>,
    should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            player: Player::new(0, 0),
            level: vec![
                // --- Boundary walls ---
                Hitbox {
                    x: -90,
                    y: -20,
                    width: 4,
                    height: 40,
                }, // left wall
                Hitbox {
                    x: 86,
                    y: -20,
                    width: 4,
                    height: 40,
                }, // right wall
                // --- Floor (3 slabs with 2 holes) ---
                // Hole 1: x=-50..-35 (15 units wide)
                // Hole 2: x=-7..28   (35 units wide)
                Hitbox {
                    x: -84,
                    y: -17,
                    width: 40,
                    height: 3,
                }, // floor A (left)
                Hitbox {
                    x: -35,
                    y: -17,
                    width: 50,
                    height: 3,
                }, // floor B (center)
                Hitbox {
                    x: 28,
                    y: -17,
                    width: 58,
                    height: 3,
                }, // floor C (right)
                // --- Platforms (ascending left to right) ---
                Hitbox {
                    x: -80,
                    y: -8,
                    width: 18,
                    height: 3,
                }, // P1 — low left
                Hitbox {
                    x: -55,
                    y: 0,
                    width: 20,
                    height: 3,
                }, // P2 — mid left
                Hitbox {
                    x: -20,
                    y: 8,
                    width: 22,
                    height: 3,
                }, // P3 — upper center-left
                Hitbox {
                    x: 10,
                    y: 14,
                    width: 20,
                    height: 3,
                }, // P4 — near the top, center-right
                Hitbox {
                    x: 40,
                    y: 5,
                    width: 18,
                    height: 3,
                }, // P5 — mid right
                Hitbox {
                    x: 62,
                    y: -5,
                    width: 18,
                    height: 3,
                }, // P6 — low right
            ],
            should_quit: false,
        }
    }

    pub fn tick(&mut self, input: Option<Input>) {
        if let Some(ref input) = input {
            if matches!(input, Input::Quit) {
                self.should_quit = true;
            }
        }

        self.player.tick(input, &self.level);
    }

    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    pub fn player_hitbox(&self) -> &Hitbox {
        &self.player.hitbox()
    }

    pub fn level(&self) -> &Vec<Hitbox> {
        &self.level
    }
}
