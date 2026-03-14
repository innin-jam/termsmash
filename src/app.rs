use crate::input::Input;
pub use hitbox::Hitbox;

mod hitbox;

pub struct App {
    player: Player,
    level: Vec<Hitbox>,
    should_quit: bool,
}

#[derive(Default)]
enum Direction {
    Left,
    #[default]
    Right,
}

#[derive(Default)]
struct Player {
    hitbox: Hitbox,
    state: PlayerState,
    direction: Direction,
}

#[derive(Default)]
enum PlayerState {
    #[default]
    Idle,
    Walk,
    Jump(u16),
    Fall(u16),
    JumpForwards(u16),
    FallForwards(u16),
    Dash(u16),
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        let width = 3;
        let height = 3;
        Self {
            hitbox: Hitbox::new_extend_upwards(x, y, width, height),
            direction: Direction::Right,
            state: PlayerState::Idle,
        }
    }

    fn tick(&mut self, input: Option<Input>, level: &[Hitbox]) {
        fn allow_dash(self_: &mut Player, input: Input) {
            match input {
                Input::Left => {
                    self_.direction = Direction::Left;
                    self_.state = PlayerState::Dash(0);
                }
                Input::Right => {
                    self_.direction = Direction::Right;
                    self_.state = PlayerState::Dash(0);
                }
                _ => {}
            }
        }

        match self.state {
            PlayerState::Idle => {
                if let Some(input) = input {
                    match input {
                        Input::Left => {
                            self.direction = Direction::Left;
                            self.state = PlayerState::Walk
                        }
                        Input::Right => {
                            self.direction = Direction::Right;
                            self.state = PlayerState::Walk
                        }
                        Input::Jump => self.state = PlayerState::Jump(0),
                        _ => {}
                    }
                }
            }

            PlayerState::Walk => {
                let dx = match self.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
                if self.hitbox.move_x(dx, level) != dx {
                    self.state = PlayerState::Idle
                };
                if !self.hitbox.touching_below(level) {
                    self.state = PlayerState::FallForwards(0);
                    return;
                }
                if let Some(input) = input {
                    match input {
                        Input::Left if matches!(self.direction, Direction::Right) => {
                            self.direction = Direction::Left;
                        }
                        Input::Right if matches!(self.direction, Direction::Left) => {
                            self.direction = Direction::Right;
                        }
                        Input::Jump => self.state = PlayerState::JumpForwards(0),
                        Input::Down => self.state = PlayerState::Idle,
                        _ => {}
                    }
                }
            }

            PlayerState::Jump(ref mut f) => {
                *f += 1;
                let dy = match f {
                    1 => 2,
                    2 => 2,
                    3 => 1,
                    4 => 1,
                    5 => 1,
                    6 => 1,
                    _ => 0,
                };
                if self.hitbox.move_y(dy, level) != dy || *f > 8 {
                    self.state = PlayerState::Fall(0)
                };
                if let Some(input) = input {
                    if matches!(input, Input::Down) {
                        self.state = PlayerState::Fall(0);
                        return;
                    }
                    allow_dash(self, input);
                }
            }

            PlayerState::Fall(ref mut f) => {
                *f += 1;
                let dy = match *f {
                    1..=3 => -1,
                    _ => -2,
                };
                if self.hitbox.move_y(dy, level) != dy {
                    self.state = PlayerState::Idle;
                }
                if let Some(input) = input {
                    allow_dash(self, input);
                }
            }

            PlayerState::JumpForwards(ref mut f) => {
                *f += 1;
                let dx = match self.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
                let dy = match f {
                    1 => 2,
                    2 => 2,
                    3 => 1,
                    4 => 1,
                    5 => 1,
                    6 => 1,
                    _ => 0,
                };

                match (
                    self.hitbox.move_x(dx, level) != dx,
                    self.hitbox.move_y(dy, level) != dy,
                ) {
                    (true, true) => self.state = PlayerState::Fall(0),
                    _ if *f > 8 => self.state = PlayerState::FallForwards(0),
                    (false, true) => self.state = PlayerState::FallForwards(0),
                    (true, false) => self.state = PlayerState::Jump(*f),
                    (false, false) => {}
                }
                if let Some(input) = input {
                    if matches!(input, Input::Down) {
                        self.state = PlayerState::Fall(0);
                        return;
                    }
                    allow_dash(self, input);
                }
            }

            PlayerState::FallForwards(ref mut f) => {
                *f += 1;
                let dx = match self.direction {
                    Direction::Left => -1,
                    Direction::Right => 1,
                };
                let dy = match *f {
                    1..=3 => -1,
                    _ => -2,
                };
                match (
                    self.hitbox.move_x(dx, level) != dx,
                    self.hitbox.move_y(dy, level) != dy,
                ) {
                    (true, false) => self.state = PlayerState::Fall(*f),
                    (false, true) => self.state = PlayerState::Walk,
                    (true, true) => self.state = PlayerState::Idle,
                    _ => {}
                }

                if let Some(input) = input {
                    if matches!(input, Input::Down) {
                        self.state = PlayerState::Fall(0);
                        return;
                    }
                    allow_dash(self, input);
                }
            }

            PlayerState::Dash(ref mut f) => {
                *f += 1;
                if *f > 5 {
                    self.state = PlayerState::FallForwards(0);
                }
                let dx = match self.direction {
                    Direction::Left => -3,
                    Direction::Right => 3,
                };

                if self.hitbox.move_x(dx, level) != dx {
                    self.state = PlayerState::Fall(0);
                }
                if let Some(input) = input {
                    if matches!(input, Input::Down) {
                        self.state = PlayerState::Fall(0);
                        return;
                    }
                }
            }
        }
    }
}

impl App {
    pub fn new() -> Self {
        App {
            player: Player::new(0, 0),
            level: vec![
                Hitbox {
                    x: -22,
                    y: -10,
                    width: 60,
                    height: 10,
                },
                Hitbox {
                    x: -34,
                    y: -10,
                    width: 10,
                    height: 10,
                },
                Hitbox {
                    x: -45,
                    y: -9,
                    width: 10,
                    height: 10,
                },
                Hitbox {
                    x: -37,
                    y: 4,
                    width: 8,
                    height: 3,
                },
                Hitbox {
                    x: -25,
                    y: -2,
                    width: 30,
                    height: 3,
                },
                Hitbox {
                    x: -10,
                    y: 6,
                    width: 20,
                    height: 3,
                },
                Hitbox {
                    x: 25,
                    y: -10,
                    width: 50,
                    height: 10,
                },
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
        &self.player.hitbox
    }

    pub fn level(&self) -> &Vec<Hitbox> {
        &self.level
    }
}
