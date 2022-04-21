#![feature(let_chains)]

use bevy::{
    gltf::Gltf,
    input::{keyboard::KeyboardInput, ElementState},
    prelude::*,
};
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0,
        })
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(loading_done)
        .add_system(play_animations)
        .run();
}

#[derive(Component)]
struct LoadingScene(Handle<Gltf>);

#[derive(Component)]
struct Animations(Vec<Handle<AnimationClip>>);

fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    commands
        .spawn()
        .insert(LoadingScene(ass.load("Character.gltf")));
    commands
        .spawn()
        .insert(LoadingScene(ass.load("Rifle.gltf")));
    let mut animations = vec![];
    for i in 0..28 {
        let path = format!("Character.gltf#Animation{i}");
        animations.push(ass.load(&path));
    }
    commands.spawn_bundle((
        Transform::default(),
        GlobalTransform::default(),
        Player,
        Animations(animations),
        Name::new("0"),
    ));
    let mut camera = PerspectiveCameraBundle::new_3d();
    camera.transform =
        Transform::from_xyz(4.0, 4.0, 4.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y);
    commands.spawn_bundle(camera);
}

#[derive(Component)]
struct Player;

fn loading_done(
    mut commands: Commands,
    gltfs: Res<Assets<Gltf>>,
    loading: Query<(Entity, &LoadingScene)>,
    player: Query<Entity, With<Player>>,
) {
    for (entity, scene) in loading.iter() {
        if let Some(gltf) = gltfs.get(&scene.0) {
            let player = player.single();
            commands.entity(player).with_children(|parent| {
                parent.spawn_scene(gltf.scenes[0].clone());
            });
            commands.entity(entity).despawn();
        }
    }
}

fn play_animations(
    mut events: EventReader<KeyboardInput>,
    mut player: Query<&mut AnimationPlayer>,
    mut animations: Query<(&Animations, &mut Name)>,
    mut id: Local<AnimationId>,
) {
    for event in events.iter() {
        if let Ok(mut player) = player.get_single_mut() 
        && let Ok((Animations(animations), mut name)) = animations.get_single_mut() 
        && event.state == ElementState::Pressed {
            *id = (event.scan_code % 27).try_into().unwrap();
            if let Some(animation) = animations.get(*id as usize) {
                name.set(animation_to_string(*id as usize));
                player.play(animation.clone_weak()).repeat();
            }
        }
    }
}

#[derive(Clone, Copy)]
enum AnimationId {
    ShootingStanding,
    FrozenStanding,
    Neutral,
    StandingIdle,
    CrouchingBackwardsWalk,
    CrouchingForwardsWalk,
    CrouchingIdle,
    CrouchingStrafeLeft,
    CrouchingStrafeRight,
    CrouchingShooting,
    StandingIdleWithGun,
    LookingDownForward,
    LookingLeft,
    LookingDownLeft,
    LookingUpLeft,
    LookingRight,
    LookingDownRight,
    LookingUpRight,
    LookingUp,
    RunningBackwards,
    RunningForwards,
    StrafingLeft,
    StrafingRight,
    SlidingForwards,
    RunningForwardsWithKnife,
    RunningFrozen,
    TurningLeft,
    TurningRight,
}

impl Default for AnimationId {
    fn default() -> Self {
        Self::ShootingStanding
    }
}

impl TryFrom<u32> for AnimationId {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::ShootingStanding as u32 => Ok(Self::ShootingStanding),
            x if x == Self::FrozenStanding as u32 => Ok(Self::FrozenStanding),
            x if x == Self::Neutral as u32 => Ok(Self::Neutral),
            x if x == Self::StandingIdle as u32 => Ok(Self::StandingIdle),
            x if x == Self::CrouchingBackwardsWalk as u32 => Ok(Self::CrouchingBackwardsWalk),
            x if x == Self::CrouchingForwardsWalk as u32 => Ok(Self::CrouchingForwardsWalk),
            x if x == Self::CrouchingIdle as u32 => Ok(Self::CrouchingIdle),
            x if x == Self::CrouchingStrafeLeft as u32 => Ok(Self::CrouchingStrafeLeft),
            x if x == Self::CrouchingStrafeRight as u32 => Ok(Self::CrouchingStrafeRight),
            x if x == Self::CrouchingShooting as u32 => Ok(Self::CrouchingShooting),
            x if x == Self::StandingIdleWithGun as u32 => Ok(Self::StandingIdleWithGun),
            x if x == Self::LookingDownForward as u32 => Ok(Self::LookingDownForward),
            x if x == Self::LookingLeft as u32 => Ok(Self::LookingLeft),
            x if x == Self::LookingDownLeft as u32 => Ok(Self::LookingDownLeft),
            x if x == Self::LookingUpLeft as u32 => Ok(Self::LookingUpLeft),
            x if x == Self::LookingRight as u32 => Ok(Self::LookingRight),
            x if x == Self::LookingDownRight as u32 => Ok(Self::LookingDownRight),
            x if x == Self::LookingUpRight as u32 => Ok(Self::LookingUpRight),
            x if x == Self::LookingUp as u32 => Ok(Self::LookingUp),
            x if x == Self::RunningBackwards as u32 => Ok(Self::RunningBackwards),
            x if x == Self::RunningForwards as u32 => Ok(Self::RunningForwards),
            x if x == Self::StrafingLeft as u32 => Ok(Self::StrafingLeft),
            x if x == Self::StrafingRight as u32 => Ok(Self::StrafingRight),
            x if x == Self::SlidingForwards as u32 => Ok(Self::SlidingForwards),
            x if x == Self::RunningForwardsWithKnife as u32 => Ok(Self::RunningForwardsWithKnife),
            x if x == Self::RunningFrozen as u32 => Ok(Self::RunningFrozen),
            x if x == Self::TurningLeft as u32 => Ok(Self::TurningLeft),
            x if x == Self::TurningRight as u32 => Ok(Self::TurningRight),
            _ => Err(()),
        }
    }
}

fn animation_to_string(index: usize) -> String {
    match index {
        0 => "shooting standing",
        1 => "frozen standing",
        2 => "neutral",
        3 => "standing idle",
        4 => "crouching backwards walk",
        5 => "crouching forwards walk",
        6 => "crouching idle",
        7 => "crouching strafe left",
        8 => "crouching strafe right",
        9 => "crouching shooting",
        10 => "standing idle with gun",
        11 => "looking down forward",
        12 => "looking left",
        13 => "looking down left",
        14 => "looking up left",
        15 => "looking right",
        16 => "looking down right",
        17 => "looking up right",
        18 => "looking up",
        19 => "running backwards",
        20 => "running forwards",
        21 => "strafing left",
        22 => "strafing right",
        23 => "sliding forwards?",
        24 => "running forwards with a knife?",
        25 => "running but frozen?",
        26 => "turning left",
        27 => "turning right",
        _ => unimplemented!(),
    }
    .to_string()
}
