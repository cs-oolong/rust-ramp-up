use crate::game::{MAP_HEIGHT, MAP_WIDTH, Projectile}

// Possible starting points: anywhere in an edge, because starting in the middle of the grid is unfair, it might be too close to the player
// Patterns are always (x,y) with x between -1,1 and y between -1,1, because larger strides would be too hard too.
// The amount of patterns can vary maybe between 1 to 20?