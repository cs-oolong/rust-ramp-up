//! Coverage-exclusion reason: this module involves rendering things in the terminal, it's difficult to unit test this, and can be verified by running the game itself.
//! If you touch this file, verify the comment above is still true and
//! the file should remain excluded from coverage.
#![warn(you_are_editing_an_coverage_excluded_file)] // ← unknown attribute
// TODO: this always warns, right way would be to warn only when the file is actually changed, on a CI tool

mod data;
mod game;
mod input;
mod player;
mod projectile;
mod rendering;

use std::io;

fn main() -> io::Result<()> {
    let mut player = player::create_player();
    let blueprints = data::load_blueprints("assets/projectiles.ron");
    let mut projectiles = projectile::create_projectiles_from_blueprints(blueprints);

    rendering::setup_terminal()?;

    let mut running = true;
    while running {
        // Handle input
        match input::handle_input()? {
            input::InputCommand::Quit => running = false,
            input::InputCommand::MoveUp => player::move_player(&mut player, 0, -1),
            input::InputCommand::MoveDown => player::move_player(&mut player, 0, 1),
            input::InputCommand::MoveLeft => player::move_player(&mut player, -1, 0),
            input::InputCommand::MoveRight => player::move_player(&mut player, 1, 0),
            input::InputCommand::None => {}
        }

        // Update game state
        for projectile in &mut projectiles {
            if projectile::check_collision(&player, projectile) {
                player::damage_player(&mut player, 1);
                projectile.active = false;
            }
        }

        for projectile in &mut projectiles {
            projectile::update_projectile(projectile);
        }

        // Render
        rendering::draw_game(&player, &projectiles)?;

        // Check game over
        if player.hp == 0 {
            running = false;
        }
    }

    rendering::restore_terminal()?;
    Ok(())
}
