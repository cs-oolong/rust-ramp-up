//! Coverage-exclusion reason: this module involves rendering things in the terminal, it's difficult to unit test this, and can be verified by running the game itself.
//! If you touch this file, verify the comment above is still true and
//! the file should remain excluded from coverage.
#![warn(you_are_editing_an_coverage_excluded_file)] // ← unknown attribute
// TODO: this always warns, right way would be to warn only when the file is actually changed, on a CI tool

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
