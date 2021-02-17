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

struct Tree{
    height: u8
}

pub fn create_trees(world: &mut World){
    let mut query = <(Entity, &Plane)>::query();
    let noise = Perlin::new();
    let trees: Vec<(Entity, f64)> = query.iter_mut(world)
        .map(|chunk| (chunk.0.clone(), noise.get([chunk.1.x as f64, chunk.1.z as f64])))
        .filter(|chunk| chunk.1.gt(&0.5)).collect();

    for tree in trees {
        world.entry(tree.0).unwrap().add_component( Tree {height: 3})
    }
} 