// Initialize game world

use amethyst::{
    assets::{DefaultLoader, Handle, Loader, ProcessingQueue},
    core::{
        math::{Point3, Vector3},
        Transform,
    },
    prelude::*,
    renderer::palette::Srgb,
    renderer::Camera,
    renderer::{
        light::{DirectionalLight, Light},
        loaders::load_from_linear_rgba,
        palette::{LinSrgba, Srgba},
        rendy::mesh::{Normal, Position, Tangent, TexCoord},
        shape::Shape,
        types::{MeshData, TextureData},
        Material, MaterialDefaults, Mesh, Texture,
    },
    SimpleState,
};
use std::f32::consts::PI;

use crate::{
    config::{ArenaConfig, CameraConfig},
    world::FwdWorld,
};

#[derive(Default)]
pub struct Bwf;

impl SimpleState for Bwf {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        let StateData {
            world, resources, ..
        } = data;

        FwdWorld::new(10, 10);
    }
}


/// Creates whole world tiles like ore, tree and so on
struct TileCreator {
    plane_material: Handle<Material>,
    plane_mash: Handle<Mesh>,
}

impl TileCreator {
    fn new(resources: &Resources) -> Self {
        let loader = resources.get::<DefaultLoader>().unwrap();
        let mesh_storage = resources.get::<ProcessingQueue<MeshData>>().unwrap();
        let tex_storage = resources.get::<ProcessingQueue<TextureData>>().unwrap();
        let mtl_storage = resources.get::<ProcessingQueue<Material>>().unwrap();

        let plane_mesh: Handle<Mesh> = loader.load_from_data(
            Shape::Plane(None)
                .generate::<(Vec<Position>, Vec<Normal>, Vec<Tangent>, Vec<TexCoord>)>(None)
                .into(),
            (),
            &mesh_storage,
        );

        let plane_albedo = loader.load_from_data(
            load_from_linear_rgba(LinSrgba::new(0.005, 0.005, 0.005, 0.005)).into(),
            (),
            &tex_storage,
        );

        let plane_material: Handle<Material> = {
            let mat_defaults = resources.get::<MaterialDefaults>().unwrap().0.clone();

            loader.load_from_data(
                Material {
                    albedo: plane_albedo,
                    ..mat_defaults
                },
                (),
                &mtl_storage,
            )
        };
        TileCreator {
            plane_material,
            plane_mash: plane_mesh,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn something() {
        unimplemented!()
    }
}

fn initialize_camera(world: &mut World, resources: &mut Resources) {
    // load config
    let camera_tilt = {
        let camera_config = resources.get::<CameraConfig>().unwrap();
        camera_config.camera_tilt
    };
    assert!(-PI / 2.0 < camera_tilt && camera_tilt < 0.0);

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 7.0, 10.0);
    transform.prepend_rotation_x_axis(camera_tilt);
    let camera = Camera::perspective(1.3, 1.0271975512, 0.1);


    world.extend(vec![(camera, transform)]);
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
    world.extend(vec![(light, pos)]);
}
