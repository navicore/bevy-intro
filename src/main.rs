use bevy::prelude::*;

fn setup() {
    // setup code here
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

#[cfg(test)]
mod tests {
    // Import the necessary items for testing
    use super::*;

    #[test]
    fn test_setup() {
        // Ensure setup function runs without errors
        setup();
    }
}
