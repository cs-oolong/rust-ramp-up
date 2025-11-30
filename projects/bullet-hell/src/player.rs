use crate::game::{Player, MAP_WIDTH, MAP_HEIGHT};

pub fn create_player() -> Player {
    Player {
        x: MAP_WIDTH / 2,
        y: MAP_HEIGHT / 2,
        hp: 4,
        max_hp: 5,
    }
}

pub fn move_player(player: &mut Player, dx: i16, dy: i16) {
    let new_x = (player.x as i16 + dx) as u16;
    let new_y = (player.y as i16 + dy) as u16;
    
    player.x = new_x.clamp(1, MAP_WIDTH - 2);
    player.y = new_y.clamp(1, MAP_HEIGHT - 2);
}

pub fn damage_player(player: &mut Player, amount: u16) {
    player.hp = player.hp.saturating_sub(amount);
}