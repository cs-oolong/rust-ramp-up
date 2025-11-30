use crate::game::{MAP_HEIGHT, MAP_WIDTH, Player};

pub fn create_player() -> Player {
    Player {
        x: MAP_WIDTH / 2,
        y: MAP_HEIGHT / 2,
        hp: 5,
        max_hp: 5,
    }
}

pub fn move_player(player: &mut Player, dx: i16, dy: i16) {
    let new_x = (player.x as i32 + dx as i32).clamp(1, (MAP_WIDTH - 2) as i32) as u16;
    let new_y = (player.y as i32 + dy as i32).clamp(1, (MAP_HEIGHT - 2) as i32) as u16;
    player.x = new_x;
    player.y = new_y;
}

pub fn damage_player(player: &mut Player, amount: u16) {
    player.hp = player.hp.saturating_sub(amount);
}

mod tests {
    use super::*;

    #[test]
    fn damage_player_updates_hp() {
        let mut player = create_player();
        damage_player(&mut player, 2);
        let expected = Player {
            x: 20,
            y: 10,
            hp: 3,
            max_hp: 5,
        };
        assert_eq!(player, expected);
    }

    #[test]
    fn damage_player_hp_never_goes_negative() {
        let mut player = create_player();
        damage_player(&mut player, 10);
        let expected = Player {
            x: 20,
            y: 10,
            hp: 0,
            max_hp: 5,
        };
        assert_eq!(player, expected);
    }

    #[test]
    fn create_player_creates_default_player() {
        let player = create_player();
        let expected = Player {
            x: 20,
            y: 10,
            hp: 5,
            max_hp: 5,
        };
        assert_eq!(player, expected);
    }

    #[test]
    fn move_player_upper_bound() {
        let mut player = create_player();
        move_player(&mut player, 60, 60);
        assert!(player.x == 40 - 2 && player.y == 20 - 2);
    }

    #[test]
    fn move_player_lower_bound() {
        let mut player = create_player();
        move_player(&mut player, -60, -60);
        println!("x is {}", player.x);
        println!("y is {}", player.y);
        assert!(player.x == 1 && player.y == 1);
    }

    #[test]
    fn move_player_within_bounds() {
        let mut player = create_player();
        move_player(&mut player, 1, -1);
        assert!(player.x == 21 && player.y == 9);
    }
}
