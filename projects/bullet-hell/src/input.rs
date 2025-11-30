use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

pub enum InputCommand {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
    None,
}

pub fn handle_input() -> std::io::Result<InputCommand> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key_event) = event::read()? {
            if key_event.kind == KeyEventKind::Press {
                return Ok(match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => InputCommand::Quit,
                    KeyCode::Up => InputCommand::MoveUp,
                    KeyCode::Down => InputCommand::MoveDown,
                    KeyCode::Left => InputCommand::MoveLeft,
                    KeyCode::Right => InputCommand::MoveRight,
                    _ => InputCommand::None,
                });
            }
        }
    }
    Ok(InputCommand::None)
}
