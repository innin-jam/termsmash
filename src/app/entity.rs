use crate::input::Input;

use super::Hitbox;

enum Direction {
    Left,
    Right,
}

pub struct Player {
    hitbox: Hitbox,
    state: PlayerState,
    dash_count: DashCount,
    direction: Direction,
}

struct DashCount(u16);

impl DashCount {
    pub fn restore(&mut self) {
        self.0 = 2;
    }

    pub fn use_dash(&mut self) -> bool {
        self.0.checked_sub(1).is_some_and(|x| {
            self.0 = x;
            true
        })
    }
}

pub enum PlayerState {
    Idle,
    Jump(u16),
    Fall(u16),
    Dash(u16),
    SnapDash(u16, i32),
    JumpAttack(u16),
    Crouch(u16),
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        let width = 3;
        let height = 3;
        Self {
            hitbox: Hitbox::new_extend_upwards(x, y, width, height),
            direction: Direction::Right,
            dash_count: DashCount(0),
            state: PlayerState::Idle,
        }
    }

    pub fn hitbox(&self) -> &Hitbox {
        &self.hitbox
    }

    pub fn tick(&mut self, input: Option<Input>, level: &[Hitbox]) {
        // Reset player position when they exit screen
        if !matches!(self.hitbox.x, -90..90) || !matches!(self.hitbox.y, -20..50) {
            self.hitbox.x = 0;
            self.hitbox.y = 0;
        }

        match self.state {
            PlayerState::Idle => {
                let Some(ground) = self.hitbox.touching_below(level) else {
                    self.state = PlayerState::Fall(0);
                    return;
                };

                self.dash_count.restore();

                if let Some(input) = input {
                    match input {
                        Input::Up => {
                            self.state = PlayerState::Jump(0);
                        }
                        Input::Left => {
                            self.direction = Direction::Left;
                            let dx = ground.x - self.hitbox.x;
                            self.state = PlayerState::SnapDash(0, dx);
                        }
                        Input::Right => {
                            self.direction = Direction::Right;
                            let dx = ground.x + ground.width - self.hitbox.x - self.hitbox.width;
                            self.state = PlayerState::SnapDash(0, dx);
                        }
                        Input::Down => {
                            self.state = PlayerState::Crouch(0);
                        }
                        _ => {}
                    };
                }
            }

            PlayerState::Jump(ref mut f) => {
                *f += 1;

                // Move player; start falling if hit ceiling or animation over
                let should_fall = [3, 2, 2, 1, 1, 0, 0, 0]
                    .get(*f as usize - 1)
                    .copied()
                    .is_none_or(|vy| self.hitbox.move_y(vy, level) != vy);

                if should_fall {
                    self.state = PlayerState::Fall(0)
                }

                if let Some(input) = input {
                    match input {
                        Input::Down => {
                            self.state = PlayerState::Fall(0);
                        }
                        Input::Left if self.dash_count.use_dash() => {
                            self.direction = Direction::Left;
                            self.state = PlayerState::Dash(0);
                        }
                        Input::Right if self.dash_count.use_dash() => {
                            self.direction = Direction::Right;
                            self.state = PlayerState::Dash(0);
                        }
                        _ => {}
                    };
                }
            }

            PlayerState::Fall(ref mut f) => {
                *f += 1;
                let vy = [-1, -1, -2, -2].get(*f as usize - 1).copied().unwrap_or(-3);

                if self.hitbox.move_y(vy, level) != vy {
                    self.state = PlayerState::Idle;
                }

                if let Some(input) = input {
                    match input {
                        Input::Left if self.dash_count.use_dash() => {
                            self.direction = Direction::Left;
                            self.state = PlayerState::Dash(0);
                        }
                        Input::Right if self.dash_count.use_dash() => {
                            self.direction = Direction::Right;
                            self.state = PlayerState::Dash(0);
                        }
                        _ => {}
                    };
                }
            }

            PlayerState::Dash(ref mut f) => {
                *f += 1;

                let vx = [5, 5, 2, 1].get(*f as usize - 1).copied().unwrap_or(0);

                let dx = match self.direction {
                    Direction::Left => -vx,
                    Direction::Right => vx,
                };
                let hit_wall = self.hitbox.move_x(dx, level) != dx;

                if *f > 2
                    && let Some(input) = input
                {
                    match input {
                        Input::Left if self.dash_count.use_dash() => {
                            self.direction = Direction::Left;
                            self.state = PlayerState::Dash(0);
                            return;
                        }
                        Input::Right if self.dash_count.use_dash() => {
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

                let should_end_dash = animation
                    .get(*f as usize - 1)
                    .copied()
                    .is_none_or(|vx| self.hitbox.move_x(vx * x.signum(), level) != vx * x.signum());

                let Some(ground) = self.hitbox.touching_below(level) else {
                    self.state = PlayerState::Fall(0);
                    return;
                };

                if let Some(input) = input {
                    match input {
                        Input::Left => {
                            self.direction = Direction::Left;
                            let dx = ground.x - self.hitbox.x;
                            self.state = PlayerState::SnapDash(0, dx);
                            return;
                        }
                        Input::Right => {
                            self.direction = Direction::Right;
                            let dx = ground.x + ground.width - self.hitbox.x - self.hitbox.width;
                            self.state = PlayerState::SnapDash(0, dx);
                            return;
                        }
                        Input::Up => {
                            self.state = PlayerState::Jump(0);
                            return;
                        }
                        _ => {}
                    };
                }

                if should_end_dash {
                    self.state = PlayerState::Idle;
                }
            }

            PlayerState::Crouch(ref mut f) => {
                *f += 1;

                if self.hitbox.touching_below(level).is_none() {
                    self.state = PlayerState::Fall(0);
                    return;
                };

                if let Some(input) = input {
                    match input {
                        Input::Left => {
                            self.direction = Direction::Left;
                            self.state = PlayerState::JumpAttack(0);
                            return;
                        }
                        Input::Right => {
                            self.direction = Direction::Right;
                            self.state = PlayerState::JumpAttack(0);
                            return;
                        }
                        _ => {}
                    };
                }

                if *f > 5 {
                    self.state = PlayerState::Idle;
                }
            }

            PlayerState::JumpAttack(ref mut f) => {
                *f += 1;

                // Move player; start falling if hit ceiling or animation over
                let animation = [3, 2, 2, 1, 1, 0, 0, 0, -1, -1, -2, -2];
                let vy = animation.get(*f as usize - 1).copied().unwrap_or(-3);
                let vx = match self.direction {
                    Direction::Left => -2,
                    Direction::Right => 2,
                };

                if vy < 0 {
                    if self.hitbox.move_y(vy, level) != vy {
                        self.state = PlayerState::Idle;
                        return;
                    }
                } else {
                    self.hitbox.move_y(vy, level);
                }
                self.hitbox.move_x(vx, level);

                if let Some(input) = input {
                    match input {
                        Input::Down => {
                            self.state = PlayerState::Fall(0);
                        }
                        Input::Left if self.dash_count.use_dash() => {
                            self.direction = Direction::Left;
                            self.state = PlayerState::Dash(0);
                        }
                        Input::Right if self.dash_count.use_dash() => {
                            self.direction = Direction::Right;
                            self.state = PlayerState::Dash(0);
                        }
                        _ => {}
                    };
                }
            }
        }
    }

    pub fn state(&self) -> &PlayerState {
        &self.state
    }
}
