use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    sprite::Mesh2dHandle,
};

use std::f32::consts::TAU;

pub struct MeshMaterial {
    pub mesh: Mesh2dHandle,
    pub material: Handle<ColorMaterial>,
}

#[derive(Clone, Copy)]
pub struct RegPoly {
    pub sides: u32,
    pub radius: f32,
}

impl RegPoly {
    pub fn new(sides: u32, radius: f32) -> Self {
        Self { sides, radius }
    }
}

impl From<RegPoly> for Mesh {
    fn from(polygon: RegPoly) -> Self {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut positions = Vec::with_capacity(polygon.sides as usize + 1);
        positions.push([0.0, 0.0, 0.0]);
        for i in 0..polygon.sides {
            let angle = (i as f32) / (polygon.sides as f32) * TAU;
            positions.push([
                angle.cos() * polygon.radius,
                angle.sin() * polygon.radius,
                0.0,
            ]);
        }

        let normals = vec![[0.0, 0.0, 1.0]; (polygon.sides + 1) as usize];

        let uvs = positions
            .iter()
            .map(|v| {
                [
                    v[0] / polygon.radius / 2.0 + 0.5,
                    v[1] / polygon.radius / 2.0 + 0.5,
                ]
            })
            .collect::<Vec<[f32; 2]>>();

        let mut indices = Vec::with_capacity(polygon.sides as usize * 3);
        for i in 1..(polygon.sides) {
            indices.push(0);
            indices.push(i);
            indices.push(i + 1);
        }
        indices.push(0);
        indices.push(polygon.sides);
        indices.push(1);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
