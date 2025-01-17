use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

use super::components::*;
use super::resources::*;
use super::{ NUMBER_OF_ENEMIES, ENEMY_SIZE, ENEMY_SPEED };

pub fn spawn_enemies(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let random_x = random::<f32>() * window.width() - window.width() / 2.0;
        let random_y = random::<f32>() * window.height() - window.height() / 2.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn despawn_enemies(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in enemy_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn update_enemy_direction(
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0; // 32.0
    let x_min = 0.0 - window.width() / 2.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size - window.width() / 2.0;
    let y_min = 0.0 - window.height() / 2.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size - window.height() / 2.0;

    for (transform, mut enemy) in enemy_query.iter_mut() {
        let mut direction_changed: bool = false;

        let trans = transform.translation;

        if trans.x < x_min || trans.x > x_max {
            enemy.direction.x *= -1.0;
            direction_changed = true;
        }
        if trans.y < y_min || trans.y > y_max {
            enemy.direction.y *= -1.0;
            direction_changed = true;
        }

        if direction_changed {
            let sfx1 = asset_server.load("audio/wallHit1.ogg");
            let sfx2 = asset_server.load("audio/wallHit2.ogg");
            let sfx = if random::<f32>() > 0.5 { sfx1 } else { sfx2 };

            audio.play(sfx);
        }
    }
}

pub fn confine_enemy_movement(
    mut enemy_query: Query<&mut Transform, With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    let window = window_query.get_single().unwrap();

    let half_enemy_size = ENEMY_SIZE / 2.0; // 32.0
    let x_min = 0.0 - window.width() / 2.0 + half_enemy_size;
    let x_max = window.width() - half_enemy_size - window.width() / 2.0;
    let y_min = 0.0 - window.height() / 2.0 + half_enemy_size;
    let y_max = window.height() - half_enemy_size - window.height() / 2.0;

    for mut transform in enemy_query.iter_mut() {
        let mut trans = transform.translation;

        if trans.x < x_min {
            trans.x = x_min;
        } else if trans.x > x_max {
            trans.x = x_max;
        }

        if trans.y < y_min {
            trans.y = y_min;
        } else if trans.y > y_max {
            trans.y = y_max;
        }

        transform.translation = trans;
    }
}

pub fn spawn_enemies_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: ResMut<EnemySpawnTimer>
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width() - window.width() / 2.0;
        let random_y = random::<f32>() * window.height() - window.height() / 2.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("sprites/ball_red_large.png"),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

pub fn tick_enemy_spawn_timer(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}
