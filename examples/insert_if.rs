#![allow(
clippy::needless_pass_by_value,
)]
use bevy::prelude::*;
use conditional_commands::ConditionalInsertBundleExt;

#[derive(Component)]
struct A;

#[derive(Component)]
struct B;

fn spawn_entities(mut commands: Commands) {
    for i in 0..10 {
        commands.spawn_empty().insert(A).insert_if(i < 3, || B);
    }
}

fn report(query_without: Query<&A, Without<B>>, query_with: Query<&A, With<B>>) {
    println!(
        "{} entities with component A and without B.",
        query_without.iter().count()
    );
    println!(
        "{} entities with both component A and B.",
        query_with.iter().count()
    );
}

fn main() {
    App::new()
        .add_startup_system(spawn_entities)
        .add_system(report)
        .run();
}
