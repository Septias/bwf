use std::{fmt::Display, ops::Deref};

use amethyst::{ecs::storage::Component, prelude::*, renderer::Mesh};
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
    fn is_tree(&self) -> bool {
        if let Self::Tree(_) = self {
            true
        } else {
            false
        }
    }
}

enum TilePart {
    Plane(Plane),
    PlaneType(PlaneType),
    Mesh(Mesh),
}

#[derive(Debug)]
enum TilePartError {
    NotAPlane,
}

impl Display for TilePartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TilePartError::NotAPlane => {
                write!(f, "This enum-variant is not a plane")
            }
        }
    }
}

impl std::error::Error for TilePartError {}

impl TilePart {
    fn is_plane(&self) -> bool{
        if let Self::Plane(_) = self {
            true
        } else {
            false
        }
    }
    fn as_plane(&self) -> Result<&Plane, TilePartError> {
        if let Self::Plane(plane) = self {
            Ok(plane)
        } else {
            Err(TilePartError::NotAPlane)
        }
    }
}

struct TilePartVec {
    components: Vec<TilePart>
}

impl TilePartVec {
    fn get_plane(&self) -> Option<&Plane>{
        Some(self.components.iter().filter_map(|elem| elem.as_plane().ok()).nth(0)?)
    }
}

impl Deref for TilePartVec {
    type Target = Vec<TilePart>;
    fn deref(&self) -> &Self::Target {
        &self.components
    }
}

pub struct FwdWorld{
    planes: Vec<TilePartVec>
}

impl FwdWorld {
    fn with_trees(mut self, noise: impl NoiseFn<[f64; 2]> ) -> Self {
        self.planes.iter_mut().for_each(|tilepart| {
 
            if let Some(plane) =  tilepart.get_plane(){
                if noise.get([plane.x as f64, plane.z as f64]) > 0.5 {
                    tilepart.push(TilePart::PlaneType( PlaneType::Tree(Tree { height: 3})));
                };
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

    pub fn build(mut self, world: &mut World){
        /* let trees = self.planes
            .drain_filter(|planeinfo| planeinfo.1.is_tree())
            .map(|planeinfo| (planeinfo.0, planeinfo.1.as_tree().unwrap()))
            .collect::<Vec<_>>(); */
        world.extend(self.planes);
    }
}   