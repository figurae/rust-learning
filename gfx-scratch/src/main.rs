use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(add_people)
        .add_system(list_names)
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands
        .spawn()
        .insert(Person)
        .insert(Name("First Name".to_string()));
    commands
        .spawn()
        .insert(Person)
        .insert(Name("Second Name".to_string()));
}

fn list_names(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("This Is {}", name.0);
    }
}
