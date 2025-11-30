use crate::game::{MAP_HEIGHT, MAP_WIDTH, Player, Projectile};

pub fn update_projectile(projectile: &mut Projectile) {
    if !projectile.active {
        return;
    }
    let (dx, dy) = projectile.pattern[projectile.step];
    projectile.x = (projectile.x as i16 + dx as i16).clamp(1, MAP_WIDTH as i16 - 2) as u16;
    projectile.y = (projectile.y as i16 + dy as i16).clamp(1, MAP_HEIGHT as i16 - 2) as u16;
    projectile.step = (projectile.step + 1) % projectile.pattern.len();
}

pub fn check_collision(player: &Player, projectile: &Projectile) -> bool {
    projectile.active && player.x == projectile.x && player.y == projectile.y
}

pub fn create_projectiles_from_blueprints(
    blueprints: Vec<crate::game::ProtoProjectile>,
) -> Vec<Projectile> {
    blueprints
        .into_iter()
        .map(|p| Projectile {
            x: p.x,
            y: p.y,
            pattern: p.pattern,
            step: 0,
            active: true,
        })
        .collect()
}

mod tests {
    use super::*;

    #[test]
    fn create_projectiles_from_blueprints_sets_step_and_active() {
        let blueprints = vec![
            crate::game::ProtoProjectile {
                x: 1,
                y: 10,
                pattern: vec![(1, 0)],
            },
            crate::game::ProtoProjectile {
                x: 38,
                y: 8,
                pattern: vec![(-1, 0)],
            },
        ];
        let projectiles = create_projectiles_from_blueprints(blueprints);
        for p in projectiles {
            assert_eq!(p.step, 0);
            assert_eq!(p.active, true);
        }
    }

    #[test]
    fn check_collision_projectile_inactive() {
        let player = Player {
            x: 0,
            y: 0,
            hp: 1,
            max_hp: 1,
        };
        let projectile = Projectile {
            x: 0,
            y: 0,
            pattern: vec![(1, 1)],
            step: 0,
            active: false,
        };
        assert!(!check_collision(&player, &projectile));
    }

    #[test]
    fn check_collision_different_x() {
        let player = Player {
            x: 0,
            y: 0,
            hp: 1,
            max_hp: 1,
        };
        let projectile = Projectile {
            x: 3,
            y: 0,
            pattern: vec![(1, 1)],
            step: 0,
            active: true,
        };
        assert!(!check_collision(&player, &projectile));
    }

    #[test]
    fn check_collision_different_y() {
        let player = Player {
            x: 0,
            y: 0,
            hp: 1,
            max_hp: 1,
        };
        let projectile = Projectile {
            x: 0,
            y: 3,
            pattern: vec![(1, 1)],
            step: 0,
            active: true,
        };
        assert!(!check_collision(&player, &projectile));
    }

    #[test]
    fn check_collision_all_hold() {
        let player = Player {
            x: 0,
            y: 0,
            hp: 1,
            max_hp: 1,
        };
        let projectile = Projectile {
            x: 0,
            y: 0,
            pattern: vec![(1, 1)],
            step: 0,
            active: true,
        };
        assert!(check_collision(&player, &projectile));
    }

    #[test]
    fn update_projectile_does_nothing_when_inactive() {
        let mut projectile = Projectile {
            x: 0,
            y: 0,
            pattern: vec![(1, 1), (-1, -1)],
            step: 0,
            active: false,
        };
        let expected = projectile.clone();
        update_projectile(&mut projectile);
        assert_eq!(projectile, expected);
    }

    #[test]
    fn update_projectile_updates_x_y_and_step() {
        let mut projectile = Projectile {
            x: 2,
            y: 2,
            pattern: vec![(1, 1), (-1, -1)],
            step: 0,
            active: true,
        };
        let expected_x_values = [3, 2];
        let expected_y_values = [3, 2];
        let expected_step_values = [1, 0];

        for update in 0..2 {
            update_projectile(&mut projectile);
            assert_eq!(projectile.x, expected_x_values[update]);
            assert_eq!(projectile.y, expected_y_values[update]);
            assert_eq!(projectile.step, expected_step_values[update]);
        }
    }

    #[test]
    fn update_projectile_step_cyclic_mod() {
        let mut projectile = Projectile {
            x: 0,
            y: 0,
            pattern: vec![(1, 1), (0, 1), (1, 0)],
            step: 0,
            active: true,
        };
        let expected_step_values = [1, 2, 0, 1, 2, 0, 1, 2];

        for update in 0..8 {
            update_projectile(&mut projectile);
            assert_eq!(projectile.step, expected_step_values[update])
        }
    }

    #[test]
    fn update_projectile_clamp_x_y_small_case() {
        let mut projectile = Projectile {
            x: 0,
            y: 0,
            pattern: vec![(-10, -10), (1, 1)],
            step: 0,
            active: true,
        };
        let mut expected = projectile.clone();
        expected.x = 1;
        expected.y = 1;
        expected.step = 1;
        update_projectile(&mut projectile);
        assert_eq!(projectile, expected);
    }

    #[test]
    fn update_projectile_clamp_x_y_big_case() {
        let mut projectile = Projectile {
            x: MAP_WIDTH + 10,
            y: MAP_HEIGHT + 10,
            pattern: vec![(1, 1), (-1, -1)],
            step: 0,
            active: true,
        };
        let mut expected = projectile.clone();
        expected.x = MAP_WIDTH - 2;
        expected.y = MAP_HEIGHT - 2;
        expected.step = 1;
        update_projectile(&mut projectile);
        assert_eq!(projectile, expected);
    }
}
