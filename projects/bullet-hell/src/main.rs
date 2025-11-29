use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode, KeyEventKind},
    style::{Print, SetForegroundColor, Color, ResetColor},
    terminal::{self, enable_raw_mode, disable_raw_mode, Clear, ClearType},
    execute,
};
use std::{io::{self, Write}, time::Duration};

const MAP_WIDTH: u16 = 40;
const MAP_HEIGHT: u16 = 20;

struct Player {
    x: u16,
    y: u16,
}

fn setup_terminal() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), cursor::Hide, Clear(ClearType::All))?;
    Ok(())
}

fn restore_terminal() -> io::Result<()> {
    execute!(io::stdout(), cursor::Show)?;
    disable_raw_mode()?;
    Ok(())
}

fn draw_game(player: &Player, stdout: &mut io::Stdout) -> io::Result<()> {
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            let is_border = x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_HEIGHT - 1;

            if is_border {
                execute!(stdout, MoveTo(x, y), Print("."))?;
            }
        }
    }
    execute!(stdout, SetForegroundColor(Color::Blue))?;
    execute!(stdout, MoveTo(player.x, player.y), Print("♥"))?;
    execute!(stdout, SetForegroundColor(Color::White))?;

    stdout.flush()?;
    Ok(())
}

fn handle_input(player: &mut Player) -> io::Result<bool> {
    if event::poll(Duration::from_millis(100))? {
        let event = event::read()?;
        
        if let Event::Key(key_event) = event {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(false), 

                    KeyCode::Up => player.y = player.y.saturating_sub(1),
                    KeyCode::Down => player.y += 1,
                    KeyCode::Left => player.x = player.x.saturating_sub(1),
                    KeyCode::Right => player.x += 1,

                    _ => {}
                }
                player.x = player.x.clamp(1, MAP_WIDTH - 2);
                player.y = player.y.clamp(1, MAP_HEIGHT - 2);
            }
        }
    }
    Ok(true)
}


fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    
    let mut player = Player {
        x: MAP_WIDTH / 2,
        y: MAP_HEIGHT / 2,
    };
    
    setup_terminal()?;

    let mut running = true;
    while running {
        draw_game(&player, &mut stdout)?;

        match handle_input(&mut player) {
            Ok(keep_running) => running = keep_running,
            Err(e) => {
                eprintln!("An error occurred: {}", e);
                break;
            }
        }
    }
    restore_terminal()?;
    Ok(())
}