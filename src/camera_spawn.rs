use bevy::{
    app::PluginGroupBuilder,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::RenderTarget,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        texture::BevyDefault,
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

pub struct SpawnCameraPlugins;

impl PluginGroup for SpawnCameraPlugins{
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(Material2dPlugin::<PostProcessingMaterial>::default())
            .add(SpawnCameraPlugin)
    }
}

#[derive(Component)]
pub struct MainCam;

struct SpawnCameraPlugin;

impl Plugin for SpawnCameraPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_camera)
        ;

    }
}

fn spawn_camera(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut post_processing_materials: ResMut<Assets<PostProcessingMaterial>>,
){
    let window = windows.primary_mut();
    let size = Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..default()
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::bevy_default(),
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..default()
    };

    image.resize(size);
    let image_handle = images.add(image);


    commands.spawn((
        Camera3dBundle{
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_xyz(0.0,5.0,0.0).looking_at(Vec3::new(5.0,0.0,5.0), Vec3::Y),
            ..default()
        },
        UiCameraConfig { show_ui: false },
        MainCam,
    ));

    let post_processing_pass_layer = RenderLayers::layer((RenderLayers::TOTAL_LAYERS - 1) as u8);

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        size.width as f32,
        size.height as f32,
    ))));

    let material_handle = post_processing_materials.add(PostProcessingMaterial {
        source_image: image_handle,
    });

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: quad_handle.into(),
            material: material_handle,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 1.5),
                ..default()
            },
            ..default()
        },
        post_processing_pass_layer,
    ));

    // The post-processing pass camera.
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                // renders after the first main camera which has default value: 0.
                priority: 1,
                ..default()
            },
            ..Camera2dBundle::default()
        },
        post_processing_pass_layer,
    ));
}

#[derive(AsBindGroup, TypeUuid, Debug,Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct PostProcessingMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    source_image: Handle<Image>,
}

impl Material2d for PostProcessingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/pixelate_shader.wgsl".into()
    }
}