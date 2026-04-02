use std::ops::Index;

use crate::input::Input;

use super::Hitbox;

#[derive(Default)]
enum Direction {
    Left,
    #[default]
    Right,
}

#[derive(Default)]
pub struct Player {
    hitbox: Hitbox,
    state: PlayerState,
    direction: Direction,
}

#[derive(Default)]
enum PlayerState {
    #[default]
    Idle,
    Jump(u16),
    Fall(u16),
    Dash(u16),
    SnapDash(u16, i32),
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        let width = 3;
        let height = 3;
        Self {
            hitbox: Hitbox::new_extend_upwards(x, y, width, height),
            direction: Direction::Right,
            state: PlayerState::Idle,
        }
    }

    pub fn hitbox(&self) -> &Hitbox {
        &self.hitbox
    }

    pub fn tick(&mut self, input: Option<Input>, level: &[Hitbox]) {
        // reset player position when they exit screen
        if !matches!(self.hitbox.x, -90..90) || !matches!(self.hitbox.y, -20..50) {
            self.hitbox.x = 0;
            self.hitbox.y = 0;
        }

        match self.state {
            PlayerState::Idle => {
                if !self.hitbox.touching_below(level).is_some() {
                    self.state = PlayerState::Fall(0);
                }
                if let Some(input) = input {
                    match input {
                        Input::Up => {
                            self.state = PlayerState::Jump(0);
                        }
                        Input::Left => {
                            self.direction = Direction::Left;
                            if let Some(b) = self.hitbox.touching_below(level) {
                                let dx = b.x - self.hitbox.x;
                                self.state = PlayerState::SnapDash(0, dx);
                            }
                        }
                        Input::Right => {
                            self.direction = Direction::Right;
                            if let Some(b) = self.hitbox.touching_below(level) {
                                let dx = b.x + b.width - self.hitbox.x - self.hitbox.width;
                                self.state = PlayerState::SnapDash(0, dx);
                            }
                        }
                        Input::SmallLeft => {
                            self.hitbox.move_x(-1, level);
                        }
                        Input::SmallRight => {
                            self.hitbox.move_x(1, level);
                        }
                        _ => {}
                    };
                }
            }

            PlayerState::Jump(ref mut f) => {
                *f += 1;
                let dy = match f {
                    1 => 3,
                    2 => 2,
                    3 => 2,
                    4 => 1,
                    5 => 1,
                    _ => 0,
                };
                if self.hitbox.move_y(dy, level) != dy || *f > 8 {
                    self.state = PlayerState::Fall(0)
                };
                if let Some(input) = input {
                    match input {
                        Input::Down => {
                            self.state = PlayerState::Fall(0);
                        }
                        Input::Left => {
                            self.direction = Direction::Left;
                            self.state = PlayerState::Dash(0);
                        }
                        Input::Right => {
                            self.direction = Direction::Right;
                            self.state = PlayerState::Dash(0);
                        }
                        _ => {}
                    };
                }
            }

            PlayerState::Fall(ref mut f) => {
                *f += 1;
                let dy = match *f {
                    1..3 => -1,
                    3..5 => -2,
                    _ => -3,
                };

                if self.hitbox.move_y(dy, level) != dy {
                    self.state = PlayerState::Idle;
                }
                if let Some(input) = input {
                    match input {
                        Input::Left => {
                            self.direction = Direction::Left;
                            self.state = PlayerState::Dash(0);
                        }
                        Input::Right => {
                            self.direction = Direction::Right;
                            self.state = PlayerState::Dash(0);
                        }
                        _ => {}
                    };
                }
            }

            PlayerState::Dash(ref mut f) => {
                *f += 1;

                let dx = *[5, 5, 2, 1].get(*f as usize - 1).unwrap_or(&0);

                let dx = match self.direction {
                    Direction::Left => -dx,
                    Direction::Right => dx,
                };
                let hit_wall = self.hitbox.move_x(dx, level) != dx;

                if *f > 2
                    && let Some(input) = input
                {
                    match input {
                        Input::Left => {
                            self.direction = Direction::Left;
                            self.state = PlayerState::Dash(0);
                            return;
                        }
                        Input::Right => {
                            self.direction = Direction::Right;
                            self.state = PlayerState::Dash(0);
                            return;
                        }
                        Input::Up if self.hitbox.touching_below(level).is_some() => {
                            self.state = PlayerState::Jump(0);
                            return;
                        }
                        _ => {}
                    };
                }
                if hit_wall {
                    if self.hitbox.touching_below(level).is_some() {
                        self.state = PlayerState::Idle;
                    } else {
                        self.state = PlayerState::Fall(0);
                    }
                    return;
                }
                if *f > 4 {
                    self.state = PlayerState::Fall(0);
                    return;
                }
            }

            PlayerState::SnapDash(ref mut f, ref mut x) => {
                *f += 1;

                let animation = {
                    let abs_x = x.unsigned_abs();
                    match abs_x {
                        0 => vec![],
                        1 => vec![1],
                        2 => vec![1, 1],
                        3 => vec![2, 1],
                        4 => vec![2, 1, 1],
                        5 => vec![2, 2, 1],
                        6 => vec![3, 2, 1],
                        7 => vec![4, 2, 1],
                        8 => vec![5, 2, 1],
                        9 => vec![5, 2, 1, 1],
                        10 => vec![5, 2, 2, 1],
                        11 => vec![5, 3, 2, 1],
                        12 => vec![5, 4, 2, 1],
                        13.. => vec![5, 5, 2, 1],
                    }
                };

                // let dx = *[5, 5, 2, 1].get(*f as usize - 1).unwrap_or(&0).min(x) * x.signum();
                // *x -= dx;

                let dx = *animation.get(*f as usize - 1).unwrap_or(&0) * x.signum();

                // let dx = match self.direction {
                //     Direction::Left => -dx,
                //     Direction::Right => dx,
                // };

                let hit_wall = self.hitbox.move_x(dx, level) != dx;

                if *f > 1
                    && let Some(input) = input
                {
                    match input {
                        Input::Left => {
                            self.direction = Direction::Left;
                            if let Some(b) = self.hitbox.touching_below(level) {
                                let dx = b.x - self.hitbox.x;
                                self.state = PlayerState::SnapDash(0, dx);
                                return;
                            }
                        }
                        Input::Right => {
                            self.direction = Direction::Right;
                            if let Some(b) = self.hitbox.touching_below(level) {
                                let dx = b.x + b.width - self.hitbox.x - self.hitbox.width;
                                self.state = PlayerState::SnapDash(0, dx);
                                return;
                            }
                        }
                        Input::Up if self.hitbox.touching_below(level).is_some() => {
                            self.state = PlayerState::Jump(0);
                            return;
                        }
                        _ => {}
                    };
                }
                if hit_wall || *f > 4 {
                    if self.hitbox.touching_below(level).is_some() {
                        self.state = PlayerState::Idle;
                    } else {
                        self.state = PlayerState::Fall(0);
                    }
                    return;
                }
            }
        }
    }
}
