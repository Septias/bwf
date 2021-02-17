// Initialize game world

use amethyst::{
    assets::{DefaultLoader, Handle},
    core::{
        math::{Point3, Vector3},
        Transform,
    },
    prelude::*,
    renderer::palette::Srgb,
    renderer::Camera,
    renderer::{
        debug_drawing::DebugLinesComponent,
        light::{DirectionalLight, Light},
        loaders::load_from_linear_rgba,
        palette::{LinSrgba, Srgba},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        Material, MaterialDefaults, Mesh, Texture,
    },
    SimpleState,
};
use std::f32::consts::PI;

use crate::config::{ArenaConfig, CameraConfig};

#[derive(Default)]
pub struct Bwf {}

impl SimpleState for Bwf {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let StateData {
            world, resources, ..
        } = data;

        

    }
}

fn initialize_ground(world: &mut World) {
    let mat_defaults = world.read_resource::<MaterialDefaults>().0.clone();
    let mesh = create_plane(world);
    let basic_albedo = create_albedo(world, 0.005);
    let hover_albedo = create_albedo(world, 0.01);
    let roughness = 1.0f32;
    let metallic = 1.0f32;

    // load config
    let (width, depth, tile_size) = {
        let arena_config = world.read_resource::<ArenaConfig>();
        (
            arena_config.width,
            arena_config.depth,
            arena_config.tile_size,
        )
    };

    let basic_mtl = create_material(
        world,
        roughness,
        metallic,
        basic_albedo.clone(),
        mat_defaults.clone(),
    );
    let hover_mtl = create_material(
        world,
        roughness,
        metallic,
        hover_albedo.clone(),
        mat_defaults.clone(),
    );

    // initialize planes
    // last term because coords are in center of plane
    let x0 = -(tile_size * width as f32 / 2.) + 0.5 * tile_size;
    let z0 = -(tile_size * depth as f32 / 2.) + 0.5 * tile_size;

    for x in 0..width {
        for y in 0..depth {
            let mut pos = Transform::default();
            pos.append_rotation_x_axis(-1.5707);
            pos.set_translation_xyz(
                x0 + (tile_size * x as f32),
                0.0,
                z0 + (tile_size * y as f32),
            );
            let piece_info = PieceInfo::new(x, y, basic_mtl.clone(), hover_mtl.clone());
        }
    }

    // create grid-lines
    let mut debug_lines_component = DebugLinesComponent::with_capacity((width * depth) as usize);
    let main_color = Srgba::new(0.0, 0.0, 0.0, 0.8);
    for x in 0..=width {
        let position = Point3::new(
            x0 - tile_size / 2.0 + (x as f32 * tile_size),
            0.0,
            z0 - tile_size / 2.00,
        );
        let direction = Vector3::new(0.0, 0.0, tile_size * depth as f32);
        debug_lines_component.add_direction(position, direction, main_color);
    }
    for z in 0..=depth {
        let position = Point3::new(
            x0 - tile_size / 2.0,
            0.0,
            z0 - tile_size / 2.00 + (z as f32 * tile_size),
        );
        let direction = Vector3::new(tile_size * width as f32, 0.0, 0.0 as f32);
        debug_lines_component.add_direction(position, direction, main_color);
    }
    world.create_entity().with(debug_lines_component).build();

    let x0 = x0 - 0.5 * tile_size;
    let z0 = z0 - 0.5 * tile_size;
}

fn create_plane(world: &mut World) -> Handle<Mesh> {
    world.exec(|loader: DefaultLoader<'_, Mesh>| {
        loader.load_from_data(
            Shape::Plane(None)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
        )
    })
}

fn create_albedo(world: &mut World, brightness: f32) -> Handle<Texture> {
    world.exec(|loader: DefaultLoader<'_, Texture>| {
        loader.load_from_data(
            load_from_linear_rgba(LinSrgba::new(brightness, brightness, brightness, 1.0)).into(),
            (),
        )
    })
}

fn initialize_camera(world: &mut World) {
    // load config
    let camera_tilt = {
        let camera_config = world.read_resource::<CameraConfig>();
        camera_config.camera_tilt
    };
    assert!(-PI / 2.0 < camera_tilt && camera_tilt < 0.0);

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 7.0, 10.0);
    transform.prepend_rotation_x_axis(camera_tilt);
    let camera = Camera::perspective(1.3, 1.0271975512, 0.1);

    world.create_entity().with(camera).with(transform).build();
}

fn initialize_light(world: &mut World) {
    let mut pos = Transform::default();
    pos.prepend_translation_y(10.0);

    let light: Light = DirectionalLight {
        color: Srgb::new(1.0, 1.0, 1.0),
        intensity: 5.0,
        direction: Vector3::new(0.0, -1.0, 0.0),
    }
    .into();
    world.create_entity().with(light).with(pos).build();
}

fn create_material(
    world: &mut World,
    roughness: f32,
    metallic: f32,
    albedo: Handle<Texture>,
    mat_defaults: Material,
) -> Handle<Material> {
    world.exec(
        |(mtl_loader, tex_loader): (DefaultLoader<'_, Material>, DefaultLoader<'_, Texture>)| {
            // inefficient to do that for every plane
            let metallic_roughness = tex_loader.load_from_data(
                load_from_linear_rgba(LinSrgba::new(0.0, roughness, metallic, 0.0)).into(),
                (),
            );

            mtl_loader.load_from_data(
                Material {
                    albedo: albedo.clone(),
                    metallic_roughness,
                    ..mat_defaults.clone()
                },
                (),
            )
        },
    )
}
