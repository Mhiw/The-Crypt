use bevy::{
    prelude::*,
    window::PrimaryWindow,
};
use std::collections::HashMap;
use lazy_static::lazy_static;

#[derive(Clone, Hash, PartialEq, Eq)]
pub enum EnemyVariant {
    Zombie,
    Skeleton,
    Slime,
    Bird,
    Pirate,
}

#[derive(Resource)]
pub struct EnemySpawnTimer(Timer);

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

lazy_static! {
    static ref KEYS: HashMap<&'static str, KeyCode> = {
        let mut map = HashMap::new();
        map.insert("Up", KeyCode::ArrowUp);
        map.insert("Down", KeyCode::ArrowDown);
        map.insert("Left", KeyCode::ArrowLeft);
        map.insert("Right", KeyCode::ArrowRight);
        map
    };
}

pub trait Entity {
    fn take_damage(&mut self, damage: f32);
    fn get_skillset(&self) -> SkillSet;
}

#[derive(Component)]
pub struct Bow {
    
}

#[derive(Component)]
pub struct MainCamera;

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

#[derive(Component)]
pub struct Enemy {
    variant: EnemyVariant,
    skillset: SkillSet,
}

impl Enemy {
    pub fn new(variant: EnemyVariant) -> Self {
        Enemy {
            variant: variant.clone(),
            skillset: ENEMIES.get(&variant.clone()).unwrap().clone(),
        }
    }

    pub fn get_variant(&self) -> EnemyVariant {
        self.variant.clone()
    }
}

impl Entity for Enemy {
    fn take_damage(&mut self, damage: f32) {
        self.get_skillset().set_damage(self.get_skillset().get_health() - damage);
    }
    
    fn get_skillset(&self) -> SkillSet {
        self.skillset.clone()
    }
}

#[derive(Debug)]
pub struct Collision {
    point: Vec2,
}

impl Collision {
    pub fn new(point: Vec2) -> Self {
        Collision {
            point,
        }
    }

    pub fn get_point(&self) -> Vec2 {
        self.point.clone()
    }
}

#[derive(Component)]
pub struct Collider {
    position: Vec2,
    dimensions: Vec2,
}

impl Collider {
    pub fn new(position: Vec2, dimensions: Vec2) -> Self {
        Collider {
            position,
            dimensions,
        }
    }

    pub fn get_dimensions(&self) -> Vec2 {
        self.dimensions.clone()
    }

    pub fn get_position(&self) -> Vec2 {
        self.position.clone()
    }

    pub fn set_position(&mut self, position: Vec2) {
        self.position = position.clone()
    }

    pub fn collide(&self, other: &Collider) -> (bool, Option<Collision>) {
        if self.get_position().x < other.get_position().x + other.get_dimensions().x &&
            self.get_position().x + self.get_dimensions().x > other.get_position().x &&
            self.get_position().y < other.get_position().y + other.get_dimensions().y &&
            self.get_position().y + self.get_dimensions().y > other.get_position().y {
            (true, Some(Collision::new(self.get_position() - other.get_position())))
        } else {
            (false, None)
        }
    }
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
        .insert_resource(EnemySpawnTimer(Timer::from_seconds(5.0, TimerMode::Repeating)))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_player)
        .add_systems(Startup, setup_bow)
        .add_systems(Update, move_player)
        .add_systems(Update, spawn_enemy)
        .add_systems(Update, check_player_collisions)
        .add_systems(Update, rotate_bow)
        .add_systems(Update, shoot_bow)
        .run()
}

fn setup_camera(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2dBundle::default(),
        MainCamera,
    ));
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
            transform: Transform {
                translation: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3::splat(5.0),
            },
            texture: asset_server.load("textures/texture_atlas.png"),
            ..default()
        },
        Player::new(50.0, 100.0, 100.0),
        Collider::new(Vec2::ZERO, Vec2::new(20.0, 20.0)),
    ));
}

fn setup_bow(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                rect: Some(Rect {
                    min: Vec2::new(16.0, 0.0),
                    max: Vec2::new(24.0, 8.0),
                }),
                ..default()
            },
            transform: Transform {
                translation: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3::splat(5.0),
            },
            texture: asset_server.load("textures/texture_atlas.png"),
            ..default()
        },
        Bow {  },
    ));
}

fn move_player(
    mut players: Query<(&mut Transform, &Player)>,
    mut player_colliders: Query<(&mut Collider, &Player)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut delta_velocity: Vec3 = Vec3::ZERO;

    if keyboard_input.pressed(*KEYS.get("Up").unwrap()) {
        delta_velocity.y = 1.0;
    }
    if keyboard_input.pressed(*KEYS.get("Down").unwrap()) {
        delta_velocity.y = -1.0;
    }
    if keyboard_input.pressed(*KEYS.get("Left").unwrap()) {
        delta_velocity.x = -1.0;
    }
    if keyboard_input.pressed(*KEYS.get("Right").unwrap()) {
        delta_velocity.x = 1.0;
    }

    for (mut player_transform, player) in players.iter_mut() {
        player_transform.translation += delta_velocity * player.get_skillset().get_speed() * time.delta_seconds();
        for (mut player_collider, _) in player_colliders.iter_mut() {
            player_collider.set_position(Vec2::new(player_transform.translation.x, player_transform.translation.y));
        }
    }
}

fn spawn_enemy(
    mut commands: Commands,
    mut spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    if !spawn_timer.0.tick(time.delta()).just_finished() {
        return
    }

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                rect: Some(Rect {
                    min: Vec2::new(24.0, 32.0),
                    max: Vec2::new(32.0, 40.0),
                }),
                ..default()
            },
            texture: asset_server.load("textures/texture_atlas.png"),
            transform: Transform {
                translation: Vec3::ZERO,
                rotation: Quat::IDENTITY,
                scale: Vec3::splat(5.0),
            },
            ..default()
        },
        Enemy::new(EnemyVariant::Zombie),
        Collider::new(Vec2::ZERO, Vec2::new(20.0, 20.0)),
    ));
}

fn check_player_collisions(
    player_colliders: Query<(&mut Collider, &Player)>,
    other_colliders: Query<&mut Collider, Without<Player>>,
) {
    for other_collider in other_colliders.iter() {
        for (player_collider, _) in player_colliders.iter() {
            println!("Collision: {:#?}", player_collider.collide(other_collider));
        }
    }
}

fn rotate_bow(
    mut bows: Query<&mut Transform, With<Bow>>,
    window: Query<&Window, With<PrimaryWindow>>,
    main_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = main_camera.single();
    let window = window.single();
   
    for mut bow_transform in bows.iter_mut() {
        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate()) {

            let a = world_position.x - bow_transform.translation.x;
            let b = world_position.y - bow_transform.translation.y;
            let angle = a.atan2(b);
            bow_transform.rotation = Quat::from_rotation_z(-angle);
        }
    }
}

fn shoot_bow(
    mut commands: Commands,
    bows: Query<&Transform, With<Bow>>,
    buttons: Res<ButtonInput<MouseButton>>,
    asset_server: Res<AssetServer>,
) {
    if !buttons.pressed(MouseButton::Left) {
        return
    }
    
    for bow_transform in bows.iter() {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    rect: Some(Rect {
                        min: Vec2::new(0.0, 0.0),
                        max: Vec2::new(8.0, 8.0),
                    }),
                    ..default()
                },
                transform: Transform {
                    translation: bow_transform.translation,
                    rotation: bow_transform.rotation,
                    scale: Vec3::splat(5.0),
                },
                texture: asset_server.load("textures/texture_atlas.png"),
                ..default()
            },
        ));
    }
}
