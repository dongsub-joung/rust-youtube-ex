use bevy::prelude::*;

fn main() {
    App::new()
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update,(update_position, print_position))
        .add_plugins(DefaultPlugins)
        .run();
}

pub fn spawn_spaceship(mut commands: Commands){
    commands.spawn((Position{ x: 0.0, y: 0.0}, Velocity { x: 1.0, y: 1.0 }));
}

#[derive(Component, Debug)]
pub struct Velocity{
    x: f32,
    y: f32,
}
#[derive(Component, Debug)]
pub struct Position{
    x: f32,
    y: f32,
}

fn update_position(mut query: Query<(&Velocity, &mut Position)>){
    for (velocity, mut position) in  query.iter_mut(){
        position.x+= velocity.x;
        position.y+= velocity.y;        
    }
}


fn print_position(query: Query<(Entity, &Position)>){
    for (entity ,position) in  query.iter(){
        info!("Entity {:?} is at postion {:?}", entity, position);
    }
}
