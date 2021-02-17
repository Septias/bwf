use amethyst::prelude::*;
use noise::{NoiseFn, Perlin};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Plane {
    x: u32,
    z: u32
}


pub fn create_planes(world: &mut World, width: u32, depth: u32){

    let mut planes = Vec::with_capacity((width * depth) as usize);

    for x in 0..width {
        for z in 0..depth {
            planes.push( (Plane{x, z}, ));
        } 
    }

    world.extend(planes);
}

pub fn create_trees(world: &mut World){
    let mut query = <(Entity, &Plane)>::query();
    let noise = Perlin::new();
    
    for (entity, plane) in query.iter_mut(world) {
        entity.add_component();
    }
} 