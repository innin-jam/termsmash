use ratatui::crossterm::event::{KeyCode, KeyModifiers};

#[derive(Debug)]
pub enum Input {
    Left,
    Right,
    Up,
    Down,
    Space,
    Quit,
}

pub fn get_input(key_code: KeyCode, key_modifiers: KeyModifiers) -> Option<Input> {
    let _shift = matches!(key_modifiers, KeyModifiers::SHIFT);
    let _ctrl = matches!(key_modifiers, KeyModifiers::CONTROL);

    Some(match key_code {
        KeyCode::Char('q') => Input::Quit,
        KeyCode::Char('a') => Input::Left,
        KeyCode::Char('s') => Input::Down,
        KeyCode::Char('d') => Input::Right,
        KeyCode::Char('w') => Input::Up,
        // Keybinds for my weird keyboard
        //KeyCode::Char('r') => Input::Left,
        //KeyCode::Char('s') => Input::Down,
        //KeyCode::Char('t') => Input::Right,
        //KeyCode::Char('f') => Input::Up,
        KeyCode::Char(' ') => Input::Space,
        _ => return None,
    })
}
