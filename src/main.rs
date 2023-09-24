//! A shader and a material that uses it.

use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, MaterialMesh2dBundle, Material2dPlugin},
};

// This is the struct that will be passed to your shader
#[derive(TypeUuid, TypePath, AsBindGroup, Debug, Clone)]
#[uuid = "8b79a178-855c-44d6-a0ca-bacf7b988219"]
pub struct Custom2DMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material2d for Custom2DMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_2d_material.wgsl".into()
    }
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, Material2dPlugin::<Custom2DMaterial>::default()))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut custom_material: ResMut<Assets<Custom2DMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    // commands.spawn(MaterialMesh2dBundle {
    //     mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
    //     transform: Transform::default().with_scale(Vec3::splat(128.)),
    //     material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //     ..default()
    // });
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3::splat(128.)),
        material: custom_material.add(Custom2DMaterial {
            color: Color::WHITE,
            color_texture: Some(asset_server.load("textures/a.png")),
        }),
        ..Default::default()
    });
}
