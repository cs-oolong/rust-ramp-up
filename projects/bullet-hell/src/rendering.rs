//! Coverage-exclusion reason: this module involves rendering things in the terminal, it's difficult to unit test this, and can be verified by running the game itself.
//! If you touch this file, verify the comment above is still true and
//! the file should remain excluded from coverage.
#![warn(you_are_editing_an_coverage_excluded_file)] // ← unknown attribute
// TODO: this always warns, right way would be to warn only when the file is actually changed, on a CI tool

use crate::game::{MAP_HEIGHT, MAP_WIDTH, Player, Projectile};
use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::{self, Write};

pub fn setup_terminal() -> io::Result<()> {
    crossterm::terminal::enable_raw_mode()?;
    execute!(io::stdout(), cursor::Hide, Clear(ClearType::All))?;
    Ok(())
}

pub fn restore_terminal() -> io::Result<()> {
    execute!(io::stdout(), cursor::Show)?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

pub fn draw_game(player: &Player, projectiles: &[Projectile]) -> io::Result<()> {
    let mut stdout = io::stdout();

    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    // Draw borders
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            if x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_HEIGHT - 1 {
                execute!(stdout, MoveTo(x, y), Print("."))?;
            }
        }
    }

    // Draw player
    execute!(stdout, SetForegroundColor(Color::Blue))?;
    execute!(stdout, MoveTo(player.x, player.y), Print("♥"))?;
    execute!(stdout, SetForegroundColor(Color::White))?;

    // Draw projectiles
    for p in projectiles {
        if p.active {
            execute!(stdout, MoveTo(p.x, p.y), Print("|"))?;
        }
    }

    // Draw UI
    execute!(
        stdout,
        MoveTo(2, MAP_HEIGHT),
        Print(format!("HP: {}/{}", player.hp, player.max_hp))
    )?;

    let bar_len = 10;
    let filled = ((player.hp as usize * bar_len) / player.max_hp as usize).min(bar_len);
    let bar = "█".repeat(filled) + &"░".repeat(bar_len - filled);
    execute!(stdout, MoveTo(2, MAP_HEIGHT + 1), Print(bar))?;

    stdout.flush()?;
    Ok(())
}
