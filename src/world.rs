use amethyst::{ecs::storage::Component, prelude::*};
use noise::{NoiseFn};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Plane {
    x: u32,
    z: u32
}

#[derive(PartialEq)]
struct Tree{
    height: u8
}

#[derive(PartialEq)]
enum PlaneType {
    Tree(Tree),
    Plane(Plane),
    None
}

impl PlaneType {
    fn as_tree(self) -> Option<Tree>{
        if let Self::Tree(tree) = self {
            Some(tree)
        } else {
            None
        }
    }
}

pub struct FwdWorld{
    planes: Vec<(Plane, PlaneType)>
}


impl FwdWorld {
    fn with_trees(mut self, noise: impl NoiseFn<[f64; 2]> ) -> Self {
        self.planes.iter_mut().for_each(|planeinfo| {
            if noise.get([planeinfo.0.x as f64, planeinfo.0.z as f64]) > 0.5 {
                planeinfo.1 = PlaneType::Tree(Tree { height: 3});
            };
        });
        self
    }

    pub fn new(width: u32, depth: u32) -> Self{
        let mut planes = Vec::with_capacity((width * depth) as usize);
        for x in 0..width {
            for z in 0..depth {
                planes.push((Plane{x, z}, PlaneType::None));
            } 
        }
        
        FwdWorld {
            planes
        }
    }

    pub fn build(self, world: &mut World){
        let trees = self.planes
            .drain_filter(|planeinfo| planeinfo.1.as_tree().is_some())
            .map(|planeinfo| (planeinfo.0, planeinfo.1.as_tree().unwrap()))
            .collect::<Vec<_>>();
        world.extend(trees);
    }
}   