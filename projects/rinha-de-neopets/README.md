## Commands
```
cargo llvm-cov --html --open
```

## Checkpoint

Today I implemented the battle events, the neopets and some functions.
What is still missing:
- Maybe split battle.rs, it's getting big, perhaps split RNG related stuff somewhere else.
- Actually change the state of the fighters. Update the HP when damage is taken, or when heal is applied.
- Stop battle if a fighter reaches zero HP before the maximum number of turns.
- Generate pairs of Neopets, so that we have a "live battles" list.
- Let the user pick which battle to watch.


- The actual betting, odds, cash out, balance, profit etc. system.

## Features to be added
- `main.rs` should become a CLI that allows the user to:
    - Create a fighter interactively via command-line, leading the fighter to be registered at `assets/neopets.json` after proper validation.
    - Create a battle by informing the names of the fighters and watch it happen live, or save it for later at `assets/battles.json`. Battles should have a unique ID/timestamp, to distinguish between two battles that are different but involve the same fighters.
    - Create N random battles between the available fighters.
    - List the names of all fighters.
    - List fighter information verbosely - besides the name, also see the stats and spells.
    - List the available battles.
    - Trigger a battle so that it happens live.
    - Place bets on a battle => complex, requires deriving betting system, architecture must be discussed!

## TODOs & Technical Debt

### 1. Runtime Validation for Programmatically Created Neopets

**Problem:** Currently, Neopet validation only occurs during JSON deserialization via `TryFrom` traits. However, all struct fields are `pub`, allowing direct construction that bypasses validation:

```rust
// This creates an invalid Neopet but compiles successfully!
let invalid_neopet = Neopet {
    name: "HackedPet".to_string(),
    health: 100,
    heal_delta: 10,
    base_attack: 5,
    base_defense: 3,
    spells: vec![spell1, spell2],  // 2 spells
    behavior: Behavior {
        attack_chance: 0.5,
        spell_chances: vec![0.1],  // Only 1 spell chance!
        heal_chance: 0.5,
    },
};
```

**Impact:**
- Invalid probabilities (don't sum to 1.0)
- Mismatched spell counts vs spell_chances length
- Battle system will behave unpredictably
- Betting odds calculations will be incorrect

**Solution Options:**
   
2. **Private Fields + Constructor**
   - Make all fields private
   - Force all creation through `Neopet::new()` that validates
   - Breaks tests that construct Neopets directly
   
3. **Builder Pattern (Recommended Long-term)**
   - Create `NeopetBuilder` with fluent API
   - `build()` method performs validation
   - Most ergonomic for both production and tests

**Recommended Approach:**
1. Refactor to Builder pattern (Option 3) once the API stabilizes
2. Update all tests to use builder instead of direct construction

**Files to Modify:**
- `src/neopets.rs`: Add `validate()` method
- `src/battle.rs`: Call `validate()` before starting battle
- Tests: Either use builder or call `validate().unwrap()` after construction

### 2. Runtime Validation for Programmatically Created BattleEvents

Same as the programmatically created neopets issue, but for BattleEvents. For example, a Roll can't be positive crit and negative crit at the same time.

## Extra - Only if there is time

### 1. ASCII art & animations for the battle display

---
