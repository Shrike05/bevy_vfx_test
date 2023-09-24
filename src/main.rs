use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{camera::RenderTarget, render_resource::*, view::RenderLayers},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle}, window::PrimaryWindow,
};

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
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<Custom2DMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut custom_material: ResMut<Assets<Custom2DMaterial>>,
    asset_server: Res<AssetServer>,
    wind: Query<&Window, With<PrimaryWindow>>,
) {
    let window_res = &wind.single().resolution;
    let window_size = Vec2{x: window_res.width(), y: window_res.height()};

    let size = Extent3d {
        width: window_size.x as u32,
        height: window_size.y as u32,
        ..default()
    };
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);

    let image_handle = images.add(image);

    let layer = RenderLayers::layer(1);

    let cam = Camera {
        order: -1,
        target: RenderTarget::Image(image_handle.clone()),
        ..default()
    };
    commands.spawn((
        Camera2dBundle {
            camera: cam,
            ..default()
        },
        layer,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(window_size),
                ..default()
            },
            texture: asset_server.load("textures/a.png"),
            ..default()
        },
        layer,
    ));

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform::default().with_scale(Vec3 {
            x: window_size.x,
            y: window_size.y,
            z: 0.,
        }),
        material: custom_material.add(Custom2DMaterial {
            color: Color::WHITE,
            color_texture: Some(image_handle),
        }),
        ..Default::default()
    },));
    commands.spawn(Camera2dBundle::default());
}
