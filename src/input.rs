use ratatui::crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug)]
pub enum Input {
    Left,
    Right,
    Up,
    Down,
    SmallLeft,
    SmallRight,
    SmallUp,
    SmallDown,
    Space,
    Quit,
}

pub fn get_input(key_code: KeyCode, key_modifiers: KeyModifiers) -> Option<Input> {
    let shift = matches!(key_modifiers, KeyModifiers::SHIFT);
    let ctrl = matches!(key_modifiers, KeyModifiers::CONTROL);

    Some(match key_code {
        KeyCode::Char('q') => Input::Quit,
        KeyCode::Char('x') => Input::SmallLeft,
        KeyCode::Char('d') => Input::SmallRight,
        KeyCode::Char('r') => Input::Left,
        KeyCode::Char('s') => Input::Down,
        KeyCode::Char('t') => Input::Right,
        KeyCode::Char('f') => Input::Up,
        KeyCode::Char(' ') => Input::Space,
        _ => return None,
    })
}
