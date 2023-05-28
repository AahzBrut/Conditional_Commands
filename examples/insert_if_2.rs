use bevy::prelude::*;
use conditional_commands::ConditionalInsertBundleExt;

#[derive(Component)]
struct Name(String);

fn lazy(mut commands: Commands) {
    commands.spawn_empty().insert_if(true, || {
        println!("condition = true, so this closure is computed and inserts a Name component.");
        Name("Ben".to_string())
    });
    commands.spawn_empty().insert_if(false, || {
        println!(
            "condition = false, so this closure is not computed, 
                        the Name component is not inserted 
                        and this message is not displayed."
        );
        Name("Jane".to_string())
    });
}

fn main() {
    App::new().add_system(lazy).run();
}
