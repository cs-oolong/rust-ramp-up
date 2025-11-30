# 🎮 **Undertale-Inspired Bullet Hell Combat System - Design Document**

## 📋 **Table of Contents**
1. [Overview](#overview)
2. [Core Systems](#core-systems)
3. [Data Structures](#data-structures)
4. [Combat Flow](#combat-flow)
5. [File Structure](#file-structure)
6. [Implementation Details](#implementation-details)
7. [Data Formats](#data-formats)
8. [UI/UX Design](#ui-ux-design)
9. [Testing Strategy](#testing-strategy)

---

## 🔍 **Overview**

Transform the current real-time bullet-hell game into a turn-based combat system inspired by Undertale's battle mechanics. The system maintains the core dodging gameplay while adding strategic depth through turn-based combat phases.

### **Key Design Principles**
- **Separation of Concerns**: Keep combat logic separate from rendering/input
- **Data-Driven**: Define enemies and attack patterns in RON files
- **Extensible**: Easy to add new enemies, patterns, and mechanics
- **Simple**: No complex abstractions, just clean functions and data

---

## 🏗️ **Core Systems**

### **1. Combat Phase Management**
```rust
// src/combat/mod.rs
pub enum CombatPhase {
    EnemyTurn,
    PlayerTurn,
    Transition,
    Victory,
    Defeat,
}

pub struct CombatState {
    pub phase: CombatPhase,
    pub current_enemy: Enemy,
    pub player: Player,
    pub active_projectiles: Vec<Projectile>,
    pub current_pattern: Option<AttackPattern>,
}
```

### **2. Attack Pattern System**
```rust
// src/combat/attack_pattern.rs
pub struct AttackPattern {
    pub name: String,
    pub projectiles: Vec<Projectile>,
    pub description: String, // For UI display
}

impl AttackPattern {
    // Returns true if all projectiles in this pattern are inactive
    pub fn is_complete(&self) -> bool {
        self.projectiles.iter().all(|p| !p.active)
    }
    
    // Spawn all projectiles for this pattern
    pub fn spawn_projectiles(&self) -> Vec<Projectile> {
        self.projectiles.iter().map(|p| Projectile {
            x: p.x,
            y: p.y,
            pattern: p.pattern.clone(),
            step: 0,
            active: true,
        }).collect()
    }
}
```

### **3. Enemy System**
```rust
// src/combat/enemy.rs
pub struct Enemy {
    pub name: String,
    pub max_hp: u16,
    pub current_hp: u16,
    pub attack_patterns: Vec<String>, // Pattern names
    pub dialogue: Vec<String>, // Enemy taunts/messages
}

impl Enemy {
    pub fn choose_random_pattern(&self) -> &str {
        &self.attack_patterns[rand::random::<usize>() % self.attack_patterns.len()]
    }
    
    pub fn take_damage(&mut self, damage: u16) {
        self.current_hp = self.current_hp.saturating_sub(damage);
    }
    
    pub fn is_defeated(&self) -> bool {
        self.current_hp == 0
    }
}
```

### **4. Player Combat Actions**
```rust
// src/combat/player_actions.rs
pub struct PlayerCombatStats {
    pub attack_min: u16,
    pub attack_max: u16,
    pub heal_amount: u16,
}

pub enum PlayerAction {
    Fight,
    Heal,
    // Future: Mercy, Items, etc.
}

pub fn calculate_attack_damage(min: u16, max: u16) -> u16 {
    rand::random::<u16>() % (max - min + 1) + min
}

pub fn execute_player_action(
    action: PlayerAction,
    player: &mut Player,
    enemy: &mut Enemy,
    stats: &PlayerCombatStats,
) -> String {
    match action {
        PlayerAction::Fight => {
            let damage = calculate_attack_damage(stats.attack_min, stats.attack_max);
            enemy.take_damage(damage);
            format!("You dealt {} damage!", damage)
        }
        PlayerAction::Heal => {
            let old_hp = player.hp;
            player.hp = (player.hp + stats.heal_amount).min(player.max_hp);
            let healed = player.hp - old_hp;
            format!("You healed {} HP!", healed)
        }
    }
}
```

---

## 🔄 **Combat Flow**

### **Main Combat Loop**
```rust
// In main.rs or combat/mod.rs
pub fn run_combat_turn(
    combat_state: &mut CombatState,
    player_action: Option<PlayerAction>,
) -> Result<String, CombatError> {
    match combat_state.phase {
        CombatPhase::EnemyTurn => {
            handle_enemy_turn(combat_state)
        }
        CombatPhase::PlayerTurn => {
            if let Some(action) = player_action {
                handle_player_turn(combat_state, action)
            } else {
                Ok("Choose your action".to_string())
            }
        }
        CombatPhase::Transition => {
            check_pattern_completion(combat_state)
        }
        _ => Ok("Combat ended".to_string()),
    }
}
```

### **Enemy Turn Flow**
```rust
fn handle_enemy_turn(state: &mut CombatState) -> Result<String, CombatError> {
    // 1. Choose random attack pattern
    let pattern_name = state.current_enemy.choose_random_pattern();
    
    // 2. Load pattern from data
    let pattern = load_attack_pattern(pattern_name)?;
    
    // 3. Spawn projectiles
    state.active_projectiles = pattern.spawn_projectiles();
    state.current_pattern = Some(pattern);
    
    // 4. Transition to dodging phase
    state.phase = CombatPhase::Transition;
    
    Ok(format!("{} uses {}!", state.current_enemy.name, pattern_name))
}
```

### **Pattern Completion Detection**
```rust
fn check_pattern_completion(state: &mut CombatState) -> Result<String, CombatError> {
    if let Some(ref pattern) = state.current_pattern {
        if pattern.is_complete() && state.active_projectiles.iter().all(|p| !p.active) {
            state.phase = CombatPhase::PlayerTurn;
            state.active_projectiles.clear();
            return Ok("Your turn! Choose FIGHT or HEAL".to_string());
        }
    }
    Ok("Keep dodging!".to_string())
}
```

---

## 📁 **File Structure**

```
src/
├── main.rs                    # Main game loop, now combat-focused
├── game.rs                    # Core types (keep existing)
├── player.rs                  # Player movement (keep existing)
├── projectile.rs              # Projectile updates (keep existing)
├── rendering.rs               # Enhanced with combat UI
├── input.rs                   # Enhanced with combat menu input
├── data.rs                    # Enhanced with enemy/pattern loading
└── combat/
    ├── mod.rs                 # Combat state management
    ├── enemy.rs               # Enemy logic
    ├── attack_pattern.rs      # Attack pattern system
    └── player_actions.rs      # Fight/Heal actions

assets/
├── projectiles.ron           # Keep existing for basic patterns
├── enemies.ron               # New: Enemy definitions
└── attack_patterns.ron       # New: Named attack patterns
```

---

## 💾 **Data Formats**

### **Enemies (assets/enemies.ron)**
```ron
[
    (
        name: "Training Dummy",
        max_hp: 25,
        current_hp: 25,
        attack_patterns: ["BasicWave", "CircleBurst", "HorizontalRain"],
        dialogue: [
            "The dummy stares blankly...",
            "Training begins!",
            "You're doing great!",
        ],
    ),
    (
        name: "Shadow Spirit",
        max_hp: 45,
        current_hp: 45,
        attack_patterns: ["Spiral", "CrossFire", "DiamondStorm", "TeleportBurst"],
        dialogue: [
            "A chill runs down your spine...",
            "The shadows writhe angrily!",
            "You feel your soul trembling...",
        ],
    ),
    (
        name: "Final Boss: Determination",
        max_hp: 80,
        current_hp: 80,
        attack_patterns: ["OmegaFloweyCircle", "SoulSpears", "RealityBreak", "MercyPlea"],
        dialogue: [
            "You feel the weight of your choices...",
            "This is your final test!",
            "Show me your determination!",
        ],
    ),
]
```

### **Attack Patterns (assets/attack_patterns.ron)**
```ron
{
    "BasicWave": (
        description: "A simple wave of projectiles",
        projectiles: [
            (x: 0, y: 10, pattern: [(1, 0)]),
            (x: 0, y: 12, pattern: [(1, 0)]),
            (x: 0, y: 8, pattern: [(1, 0)]),
        ],
    ),
    
    "CircleBurst": (
        description: "Projectiles burst outward in a circle",
        projectiles: [
            (x: 20, y: 10, pattern: [(1, 0), (0, 1), (-1, 0), (0, -1)]),
            (x: 20, y: 10, pattern: [(1, 1), (-1, 1), (-1, -1), (1, -1)]),
            (x: 20, y: 10, pattern: [(2, 0), (0, 2), (-2, 0), (0, -2)]),
        ],
    ),
    
    "Spiral": (
        description: "A spiraling pattern of projectiles",
        projectiles: [
            (x: 10, y: 10, pattern: [(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)]),
            (x: 30, y: 10, pattern: [(-1, 0), (-1, -1), (0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1)]),
        ],
    ),
    
    "CrossFire": (
        description: "Projectiles from all directions",
        projectiles: [
            (x: 20, y: 0, pattern: [(0, 1)]),
            (x: 20, y: 19, pattern: [(0, -1)]),
            (x: 0, y: 10, pattern: [(1, 0)]),
            (x: 39, y: 10, pattern: [(-1, 0)]),
        ],
    ),
}
```

---

## 🎨 **UI/UX Design**

### **Combat Screen Layout**
```
┌────────────────────────────────────────┐
│ ENEMY TURN                             │  <- Phase indicator
│ Shadow Spirit HP: [████████░░░░░░░░]  │  <- Enemy HP bar
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                  ♥                     │  <- Player (dodging area)
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│ Keep dodging!                          │  <- Status messages
│                                        │
│ HP: 4/5 [█████░░░░░]                   │  <- Player HP
└────────────────────────────────────────┘
```

### **Player Turn Menu**
```
┌────────────────────────────────────────┐
│ YOUR TURN                              │
│ Shadow Spirit HP: [████████░░░░░░░░]  │
│                                        │
│ What will you do?                      │
│                                        │
│  > FIGHT      HEAL                     │  <- Menu options
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│                                        │
│ HP: 4/5 [█████░░░░░]                   │
└────────────────────────────────────────┘
```

### **Enhanced Rendering Module**
```rust
// src/rendering.rs additions
pub fn draw_combat_ui(
    enemy: &Enemy,
    phase: &CombatPhase,
    message: &str,
    show_menu: bool,
) -> io::Result<()> {
    // Draw phase indicator
    let phase_text = match phase {
        CombatPhase::EnemyTurn => "ENEMY TURN",
        CombatPhase::PlayerTurn => "YOUR TURN",
        CombatPhase::Transition => "DODGE!",
        _ => "",
    };
    
    // Draw enemy HP bar
    let enemy_hp_bar = create_hp_bar(enemy.current_hp, enemy.max_hp, 20);
    execute!(stdout, MoveTo(2, 1), Print(format!("{} HP: {}", enemy.name, enemy_hp_bar)))?;
    
    // Draw status message
    execute!(stdout, MoveTo(2, 17), Print(message))?;
    
    // Draw player action menu if it's player turn
    if show_menu {
        draw_player_menu()?;
    }
    
    Ok(())
}

fn draw_player_menu() -> io::Result<()> {
    execute!(stdout, MoveTo(10, 10), Print("What will you do?"))?;
    execute!(stdout, MoveTo(10, 12), Print("> FIGHT      HEAL"))?;
    Ok(())
}
```

---

## 🧪 **Testing Strategy**

### **Unit Tests**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_pattern_completion() {
        let mut pattern = AttackPattern {
            name: "Test".to_string(),
            projectiles: vec![
                Projectile { active: false, ..Default::default() },
                Projectile { active: false, ..Default::default() },
            ],
            description: "Test".to_string(),
        };
        assert!(pattern.is_complete());
        
        pattern.projectiles[0].active = true;
        assert!(!pattern.is_complete());
    }

    #[test]
    fn test_damage_calculation() {
        let damage = calculate_attack_damage(5, 10);
        assert!(damage >= 5 && damage <= 10);
    }

    #[test]
    fn test_enemy_pattern_selection() {
        let enemy = Enemy {
            name: "Test".to_string(),
            max_hp: 10,
            current_hp: 10,
            attack_patterns: vec!["pattern1".to_string(), "pattern2".to_string()],
            dialogue: vec![],
        };
        
        let selected = enemy.choose_random_pattern();
        assert!(selected == "pattern1" || selected == "pattern2");
    }
}
```

### **Integration Tests**
```rust
#[test]
fn test_full_combat_flow() {
    // Setup combat state
    let mut combat = CombatState::new(
        Enemy::new("Test Enemy", 20, vec!["BasicWave"]),
        Player::new(),
    );
    
    // Test enemy turn
    let result = combat.start_enemy_turn().unwrap();
    assert!(result.contains("uses"));
    assert_eq!(combat.phase, CombatPhase::Transition);
    
    // Simulate dodging completion
    combat.active_projectiles.clear();
    let result = combat.check_pattern_completion().unwrap();
    assert_eq!(combat.phase, CombatPhase::PlayerTurn);
    assert!(result.contains("Your turn"));
}
```

---

## 🚀 **Implementation Roadmap**

### **Phase 1: Core Combat System**
1. Create combat module structure
2. Implement `AttackPattern` and pattern completion detection
3. Implement `Enemy` structure with pattern selection
4. Create combat state management
5. Update main game loop to use combat phases

### **Phase 2: Data Integration**
1. Create `enemies.ron` and `attack_patterns.ron` files
2. Implement data loading functions
3. Test with basic enemy (Training Dummy)

### **Phase 3: Player Actions**
1. Implement player action system (Fight/Heal)
2. Add damage calculation and healing
3. Create player turn menu system

### **Phase 4: UI Enhancement**
1. Update rendering for combat UI
2. Add phase indicators and menus
3. Implement enemy HP bars
4. Add combat messages/log

### **Phase 5: Polish & Balance**
1. Add more attack patterns
2. Add more enemy types
3. Balance damage numbers
4. Add victory/defeat screens
5. Add enemy dialogue system

---

## 🎯 **Success Criteria**

- ✅ Combat feels turn-based and strategic
- ✅ Each enemy has distinct attack patterns
- ✅ Player has meaningful choices (Fight vs Heal)
- ✅ Dodging phase is engaging and fair
- ✅ UI clearly shows combat state
- ✅ System is extensible for new enemies/patterns
- ✅ No complex abstractions - keeps the simple, clean style

---

## 💡 **Future Enhancements** (Optional)

- **Mercy System**: Spare enemies like in Undertale
- **Timed Hits**: Press button at right time for bonus damage
- **Items**: Consumables that heal more or boost attack
- **Multiple Enemies**: Fight groups of enemies
- **Boss Phases**: Enemies that change patterns at low HP
- **Player Progression**: Level up, increase stats
- **Special Attacks**: Pattern-specific player abilities

---

This design maintains the simplicity of your current codebase while adding substantial gameplay depth. The turn-based structure creates natural tension/release cycles, and the pattern system makes each enemy feel unique. The data-driven approach means you can add new content just by editing RON files!

Happy coding! 🎮✨