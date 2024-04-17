use bevy::{
    prelude::*,
    input::keyboard::KeyboardInput,
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

impl SkillSet {
    pub fn get_xp(&self) -> f32 {
        self.xp.clone()
    }
    
    pub fn set_xp(&mut self, value: f32) {
        self.xp = value;
    }

    pub fn get_damage(&self) -> f32 {
        self.damage.clone()
    }
    
    pub fn set_damage(&mut self, value: f32) {
        self.damage = value;
    }

    pub fn get_health(&self) -> f32 {
        self.health.clone()
    }
    
    pub fn set_health(&mut self, value: f32) {
        self.health = value;
    }

    pub fn get_speed(&self) -> f32 {
        self.speed.clone()
    }
    
    pub fn set_speed(&mut self, value: f32) {
        self.speed = value;
    }
}

impl Clone for SkillSet {
    fn clone(&self) -> Self {
        SkillSet {
            xp: self.get_xp(),
            damage: self.get_damage(),
            health: self.get_health(),
            speed: self.get_speed(),
        }
    }
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
    fn take_damage(&mut self, damage: f32);
    fn get_skillset(&self) -> SkillSet;
}

#[derive(Component)]
pub struct Player {
    skillset: SkillSet,
}

impl Player {
    pub fn new(damage: f32, health: f32, speed: f32) -> Self {
        Player {
            skillset: SkillSet {
                xp: 0.0,
                damage,
                health,
                speed,
            },
        }
    }

    
}

impl Entity for Player {
    fn take_damage(&mut self, damage: f32) {
        self.get_skillset().set_damage(self.get_skillset().get_health() - damage);
    }
    
    fn get_skillset(&self) -> SkillSet {
        self.skillset.clone()
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
        .add_systems(Startup, setup_player)
        .run()
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                rect: Some(Rect {
                    min: Vec2::new(32.0, 16.0),
                    max: Vec2::new(40.0, 24.0),
                }),
                ..default()
            },
            texture: asset_server.load("textures/texture_atlas.png"),
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(50.0),
        },
        Player::new(50.0, 100.0, 100.0),
    ));
}

fn move_player(
    mut commands: Commands,
    mut players: Query<&mut Player>,
    mut keyboard_event_reader: EventReader<KeyboardInput>,
    time: Res<Time>,
) {
    use bevy::input::ButtonState;

    for event in keyboard_event_reader.read() {
        let pressed_key: Option<KeyCode> = match event.state {
            ButtonState::Pressed => Some(event.key_code),
            _ => None,
        };

        if pressed_key.is_some() {
            
        }
    }
}
