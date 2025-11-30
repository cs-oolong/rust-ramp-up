use crate::game::{Projectile, Player, MAP_WIDTH, MAP_HEIGHT};

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

pub fn create_projectiles_from_blueprints(blueprints: Vec<crate::game::ProtoProjectile>) -> Vec<Projectile> {
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