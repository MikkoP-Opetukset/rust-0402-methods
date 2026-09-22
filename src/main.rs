/// Entry point for Chapter 5.3: Method Syntax
///
/// This file continues the small game-world theme from the structs example.
/// Methods put behavior next to the data type that behavior works with.
fn main() {
    methods_with_an_immutable_receiver();
    methods_with_a_mutable_receiver();
    methods_with_more_parameters();
    associated_functions();
    multiple_impl_blocks();
}

/// A game character, using the same fields as the structs example.
struct Character {
    name: String,
    health: u32,
    level: u32,
    is_npc: bool,
}

/// A position in the game's map grid.
struct MapPosition(i32, i32);

/// An offset in pixels on the player's screen.
struct ScreenOffset(i32, i32);

/// A marker type for one set of game rules.
struct HardcoreMode;

/// # Methods with an Immutable Receiver
/// - An `impl` block defines behavior for a type.
/// - The first parameter, `&self`, borrows the value immutably.
/// - Inside the method, `self` refers to the value before the dot.
/// - Rust automatically borrows a value when calling a method that needs
///   `&self`, so `hero.status()` works without writing `(&hero).status()`.
fn methods_with_an_immutable_receiver() {
    println!("\n{:=>80}", "");
    println!("methods_with_an_immutable_receiver()\n");

    let hero = Character::new_player("Iines", 3);

    // Rust automatically creates an immutable borrow for this method call.
    hero.status();
    println!("Can still use the hero: {}", hero.name);
}

/// # Methods with a Mutable Receiver
/// - `&mut self` borrows the value mutably, so the caller needs a mutable
///   binding.
/// - A mutable method can update the fields of the value it receives.
fn methods_with_a_mutable_receiver() {
    println!("\n{:=>80}", "");
    println!("methods_with_a_mutable_receiver()\n");

    let mut hero = Character::new_player("Iines", 3);
    hero.take_damage(35);
    hero.status();

    // An immutable binding cannot call a method that requires `&mut self`.
    // let hero = Character::new_player("Iines", 3);
    // hero.take_damage(35); // Uncomment to see the error.
}

/// # Methods with More Parameters
/// - A method can take parameters after `self`.
/// - Parameters after `self` work like ordinary function parameters.
/// - Borrowing both positions lets us compare them without transferring
///   ownership of either value.
fn methods_with_more_parameters() {
    println!("\n{:=>80}", "");
    println!("methods_with_more_parameters()\n");

    let hero_position = MapPosition(4, 9);
    let treasure_position = MapPosition(10, 6);

    let steps = hero_position.manhattan_distance_to(&treasure_position);
    println!("The treasure is {steps} map steps away.");
}

/// # Associated Functions
/// - Functions in an `impl` block without a `self` parameter are associated
///   functions, not methods.
/// - They are called with `TypeName::function_name()`.
/// - Associated functions often construct a value, such as
///   `Character::new_player`.
fn associated_functions() {
    println!("\n{:=>80}", "");
    println!("associated_functions()\n");

    let spawn_point = MapPosition::origin();
    let camera_offset = ScreenOffset::centered();
    let rules = HardcoreMode::new();
    let guide = Character::new_npc("Leenu", 100, 3);

    println!(
        "{} starts at map position ({}, {}).",
        guide.name, spawn_point.0, spawn_point.1
    );
    println!("Camera offset: ({}, {}).", camera_offset.0, camera_offset.1);
    println!("Hardcore rules selected: {}", rules.is_enabled());
}

/// # Multiple `impl` Blocks
/// - A type may have more than one `impl` block.
/// - Rust treats all of a type's implementation blocks as one set of methods.
/// - Keeping methods together is often easier to read, but separate blocks are
///   useful when implementations are conditionally compiled or organized by
///   traits.
fn multiple_impl_blocks() {
    println!("\n{:=>80}", "");
    println!("multiple_impl_blocks()\n");

    let mut hero = Character::new_player("Iines", 3);
    hero.take_damage(150);
    hero.status();

    hero.revive();
    hero.status();
}

// Implementing methods for the structs.
impl Character {
    /// Creates a player-controlled character.
    fn new_player(name: &str, level: u32) -> Self {
        Self {
            name: String::from(name),
            health: 100,
            level,
            is_npc: false,
        }
    }

    /// Creates a non-player character from the given game data.
    fn new_npc(name: &str, health: u32, level: u32) -> Self {
        Self {
            name: String::from(name),
            health,
            level,
            is_npc: true,
        }
    }

    /// Prints information without changing the character.
    fn status(&self) {
        let controller = if self.is_npc { "NPC" } else { "player" };
        println!(
            "{} is a level {} {controller} with {} health.",
            self.name, self.level, self.health
        );
    }

    /// Reduces health, stopping at zero instead of underflowing.
    fn take_damage(&mut self, amount: u32) {
        self.health = self.health.saturating_sub(amount);
    }
}

impl Character {
    /// Restores health. This method is in a separate impl block deliberately
    /// to show it is valid. Though in this case it does not make any
    /// particular sense.
    fn revive(&mut self) {
        self.health = 100;
    }
}

impl MapPosition {
    /// Returns the map's starting position.
    fn origin() -> Self {
        Self(0, 0)
    }

    /// Returns the number of horizontal and vertical grid steps between points.
    fn manhattan_distance_to(&self, other: &Self) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl ScreenOffset {
    /// Returns a screen offset that does not move the camera.
    fn centered() -> Self {
        Self(0, 0)
    }
}

impl HardcoreMode {
    /// Creates the marker value for the hardcore rule set.
    fn new() -> Self {
        Self
    }

    /// A method can also be defined for a unit-like struct.
    fn is_enabled(&self) -> bool {
        true
    }
}
