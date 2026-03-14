use ratatui::crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug)]
pub enum Input {
    Left,
    Right,
    Up,
    Down,
    Jump,
    Quit,
}

pub fn get_input(key_code: KeyCode, key_modifiers: KeyModifiers) -> Option<Input> {
    Some(match key_code {
        KeyCode::Char('q') => Input::Quit,
        KeyCode::Char('r') => Input::Left,
        KeyCode::Char('s') => Input::Down,
        KeyCode::Char('t') => Input::Right,
        KeyCode::Char('f') => Input::Up,
        KeyCode::Char(' ') => Input::Jump,
        _ => return None,
    })
}
