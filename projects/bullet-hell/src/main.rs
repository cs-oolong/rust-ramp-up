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
    max_hp: u16,
    hp: u16,
}

struct Projectile {
    x: u16,
    y: u16,
    pattern: Vec<(i8, i8)>,
    step: usize,
    active: bool,
}

impl Projectile {
    fn update(&mut self) {
        if !self.active { return; }
        let (dx, dy) = self.pattern[self.step];
        self.x = (self.x as i16 + dx as i16).clamp(1, MAP_WIDTH as i16 - 2) as u16;
        self.y = (self.y as i16 + dy as i16).clamp(1, MAP_HEIGHT as i16 - 2) as u16;
        self.step = (self.step + 1) % self.pattern.len();
    }
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

fn draw_game(player: &Player, projectile: &Projectile, stdout: &mut io::Stdout) -> io::Result<()> {
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

    if projectile.active {
        execute!(stdout, MoveTo(projectile.x, projectile.y), Print("|"))?;
    }
    
    execute!(stdout, MoveTo(2, MAP_HEIGHT), Print(format!("HP: {}/{}", player.hp, player.max_hp)))?;
    let bar_len = 10;
    let filled = ((player.hp as usize * bar_len) / player.max_hp as usize).min(bar_len);
    let bar = format!("█").repeat(filled) + &"░".repeat(bar_len - filled);
    execute!(stdout, MoveTo(2, MAP_HEIGHT + 1), Print(bar))?;

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
        hp: 4,
        max_hp: 5,
    };

    let mut projectile = Projectile {
        x: 1,
        y:  MAP_HEIGHT / 2,
        pattern: vec![(1,0)],
        step: 0,
        active: true,
    };
    
    setup_terminal()?;

    let mut running = true;
    while running {
        if projectile.active && (player.x, player.y) == (projectile.x, projectile.y) {
            player.hp = player.hp.saturating_sub(1);
            projectile.active = false;
        }

        projectile.update();
        draw_game(&player, &projectile, &mut stdout)?;

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