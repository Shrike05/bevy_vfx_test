use bevy::{
    asset::LoadState,
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages, TextureViewDescriptor, TextureViewDimension,
        },
        view::RenderLayers,
    },
};

/// This example illustrates how to create a texture for use with a `texture_2d_array<f32>` shader
/// uniform variable.
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            MaterialPlugin::<ArrayTextureMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, create_array_texture)
        .run();
}

#[derive(Resource)]
struct LoadingTexture {
    is_loaded: bool,
    handle: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Start loading the texture.
    commands.insert_resource(LoadingTexture {
        is_loaded: false,
        handle: asset_server.load(r"textures\a.png"),
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(-3.0, 2.0, -1.0),
        ..Default::default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 3000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(3.0, 2.0, 1.0),
        ..Default::default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, 0.0).looking_at(Vec3::new(0., 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });
}

fn create_array_texture(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut loading_texture: ResMut<LoadingTexture>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ArrayTextureMaterial>>,
) {
    if loading_texture.is_loaded
        || asset_server.get_load_state(loading_texture.handle.clone()) != LoadState::Loaded
    {
        return;
    }
    loading_texture.is_loaded = true;
    //let image = images.get_mut(&loading_texture.handle).unwrap();
    let size = Extent3d {
        width: 512,
        height: 512,
        depth_or_array_layers: 1,
    };

    // This is the texture that will be rendered to.
    let mut cam_image = Image {
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
        texture_view_descriptor: Some(TextureViewDescriptor{
            dimension: Some(TextureViewDimension::D2),
            ..default()
        }),
        ..default()
    };

    // fill image.data with zeroes
    cam_image.resize(size);
    
    println!("DEPTH: {}", cam_image.texture_descriptor.size.depth_or_array_layers);
    println!("DIM: {:?}", cam_image.clone().texture_view_descriptor.unwrap().dimension.unwrap());

    let cam_image_handle = images.add(cam_image);
    

    // This specifies the layer used for the first pass, which will be attached to the first pass camera and cube.
    let first_pass_layer = RenderLayers::layer(1);

    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(cam_image_handle.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 15.0))
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        first_pass_layer,
    ));
    
    let c_image = images.get_mut(&cam_image_handle).unwrap();

    let mut n_image = Image {
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
        texture_view_descriptor: Some(TextureViewDescriptor{
            dimension: Some(TextureViewDimension::D2Array),
            ..default()
        }),
        ..default()
    };

    n_image.data = c_image.data.clone();

    let c_array_layers = 4;
    n_image.reinterpret_stacked_2d_as_array(c_array_layers);
    println!("c_DEPTH: {}", n_image.texture_descriptor.size.depth_or_array_layers);
    println!("c_DIM: {:?}", n_image.clone().texture_view_descriptor.unwrap().dimension.unwrap());

    let n_handle = images.add(n_image);

    // let image = images.get_mut(&loading_texture.handle).unwrap();
    // println!("DEPTH: {}", image.texture_descriptor.size.depth_or_array_layers);
    // let array_layers = 4;
    // image.reinterpret_stacked_2d_as_array(array_layers);
    // println!("secondDEPTH: {}", image.texture_descriptor.size.depth_or_array_layers);

    // Spawn Plane
    let material_handle = materials.add(ArrayTextureMaterial {
        array_texture: n_handle.clone(),
    });
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: material_handle.clone(),
        transform: Transform::from_xyz(0., 0.0, 0.0),
        ..Default::default()
    });
}

#[derive(TypeUuid, TypePath, AsBindGroup, Debug, Clone)]
#[uuid = "8a79a178-855c-44d6-a0ca-bacf7b988219"]
struct ArrayTextureMaterial {
    #[texture(0, dimension = "2d_array")]
    #[sampler(1)]
    array_texture: Handle<Image>,
}

impl Material for ArrayTextureMaterial {
    fn fragment_shader() -> ShaderRef {
        r"shaders\array_texture.wgsl".into()
    }
}
