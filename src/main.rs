use bevy::{
    prelude::*,
};
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Hash, PartialEq, Eq)]
pub enum EnemyVariant {
    Zombie,
    Skeleton,
    Slime,
    Bird,
    Pirate,
}

pub struct SkillSet {
    xp: f32,
    damage: f32,
    health: f32,
    speed: f32,
}

lazy_static! {
    static ref ENEMIES: HashMap<EnemyVariant, SkillSet> = {
        let mut map = HashMap::new();
        map.insert(EnemyVariant::Zombie, SkillSet {
            xp: 10.0,
            damage: 50.0,
            health: 100.0,
            speed: 10.0,
        });
        map.insert(EnemyVariant::Skeleton, SkillSet {
            xp: 15.0,
            damage: 25.0,
            health: 75.0,
            speed: 30.0,
        });
        map.insert(EnemyVariant::Slime, SkillSet {
            xp: 20.0,
            damage: 45.0,
            health: 75.0,
            speed: 20.0,
        });
        map.insert(EnemyVariant::Bird, SkillSet {
            xp: 35.0,
            damage: 10.0,
            health: 25.0,
            speed: 50.0,
        });
        map.insert(EnemyVariant::Pirate, SkillSet {
            xp: 50.0,
            damage: 75.0,
            health: 250.0,
            speed: 8.0,
        });
        map
    };
}

pub trait Entity {
    fn take_damage(damage: f32);
}

pub struct Player {
    skills: SkillSet,
}

impl Player {
    pub fn new(&mut self, damage: f32, health: f32, speed: f32) -> Self {
        Player {
            skills: SkillSet {
                xp: 0.0,
                damage,
                health,
                speed,
            },
        }
    }
}

pub struct Enemy {
    variant: EnemyVariant,
    skills: SkillSet,
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "The Crypt".into(),
                        resolution: (512.0, 512.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .build()
        )
        .add_systems(Startup, setup_camera)
        .run()
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}
