// To run this code, you will need the 'crossterm' dependency.
// Add the following to your Cargo.toml file:
// [dependencies]
// crossterm = "0.27"

use crossterm::{
    cursor::{self, MoveTo},
    event::{self, Event, KeyCode, KeyEventKind},
    style::Print,
    terminal::{self, enable_raw_mode, disable_raw_mode, Clear, ClearType},
    execute,
};
use std::{io::{self, Write}, time::Duration};

// --- Game Constants ---
const MAP_WIDTH: u16 = 40;
const MAP_HEIGHT: u16 = 20;

// --- Game State Structure ---
struct Player {
    x: u16,
    y: u16,
}

// --- Core Game Functions ---

/// Initializes the terminal, enabling raw mode for immediate keypress handling.
fn setup_terminal() -> io::Result<()> {
    // Enable raw mode, which lets us read keys without waiting for Enter
    // and prevents input from being echoed to the screen.
    enable_raw_mode()?;
    
    // Hide the terminal cursor and clear the screen
    execute!(io::stdout(), cursor::Hide, Clear(ClearType::All))?;
    Ok(())
}

/// Restores the terminal to its normal state.
fn cleanup_terminal() -> io::Result<()> {
    // Show the cursor
    execute!(io::stdout(), cursor::Show)?;
    
    // Restore the terminal mode
    disable_raw_mode()?;
    Ok(())
}

/// Clears the screen and draws the game state.
fn draw_game(player: &Player, stdout: &mut io::Stdout) -> io::Result<()> {
    // 1. Clear the entire screen
    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0))?;

    // 2. Draw the border
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            // Check if we are on an edge row or column
            let is_border = x == 0 || x == MAP_WIDTH - 1 || y == 0 || y == MAP_HEIGHT - 1;

            if is_border {
                // Move cursor to position (x, y) and print a dot
                execute!(stdout, MoveTo(x, y), Print("."))?;
            }
        }
    }

    // 3. Draw the player
    // Move cursor to player position and print the player character
    execute!(stdout, MoveTo(player.x, player.y), Print("@"))?;

    // Flush the buffer to ensure everything is drawn immediately
    stdout.flush()?;
    Ok(())
}

/// Handles player movement based on the key pressed.
fn handle_input(player: &mut Player) -> io::Result<bool> {
    // Check if an event is available without blocking for 100 milliseconds
    if event::poll(Duration::from_millis(100))? {
        let event = event::read()?;
        
        // We only care about key presses (when the key is pressed down)
        if let Event::Key(key_event) = event {
            if key_event.kind == KeyEventKind::Press {
                match key_event.code {
                    // Exit the game on 'q' or ESC
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(false), 

                    // Movement: saturating_sub ensures we don't wrap around below 0
                    KeyCode::Up => player.y = player.y.saturating_sub(1),
                    KeyCode::Down => player.y += 1,
                    KeyCode::Left => player.x = player.x.saturating_sub(1),
                    KeyCode::Right => player.x += 1,

                    _ => {} // Ignore other keys
                }

                // Clamp player position to boundary (1 to MAP_WIDTH/HEIGHT - 2)
                // This prevents the player from moving onto the border lines (0 and MAX-1)
                player.x = player.x.clamp(1, MAP_WIDTH - 2);
                player.y = player.y.clamp(1, MAP_HEIGHT - 2);
            }
        }
    }
    Ok(true) // Continue the loop
}


// --- Main Execution ---

fn main() -> io::Result<()> {
    // Initialize standard output handle
    let mut stdout = io::stdout();
    
    // Initial player position (center of the map, accounting for borders)
    let mut player = Player {
        x: MAP_WIDTH / 2,
        y: MAP_HEIGHT / 2,
    };
    
    // 1. Set up the terminal environment
    setup_terminal()?;

    // 2. Main Game Loop
    let mut running = true;
    while running {
        // A. Draw the current game state
        draw_game(&player, &mut stdout)?;

        // B. Handle user input
        match handle_input(&mut player) {
            Ok(keep_running) => running = keep_running,
            Err(e) => {
                // Handle potential I/O errors and exit gracefully
                eprintln!("An error occurred: {}", e);
                break;
            }
        }
    }

    // 3. Cleanup terminal before exiting
    cleanup_terminal()?;

    // Final message printed after cleanup
    println!("Game Over! Thanks for playing.");
    Ok(())
}