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
    pub fill: bool,
}

impl RegPoly {
    pub fn new(sides: u32, radius: f32, fill: bool) -> Self {
        Self {
            sides,
            radius,
            fill,
        }
    }

    pub fn fill(sides: u32, radius: f32) -> Self {
        Self::new(sides, radius, true)
    }

    pub fn outline(sides: u32, radius: f32) -> Self {
        Self::new(sides, radius, false)
    }

    fn fill_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        let mut positions = Vec::with_capacity(self.sides as usize + 1);
        positions.push([0.0, 0.0, 0.0]);
        for i in 0..self.sides {
            let angle = (i as f32) / (self.sides as f32) * TAU;
            positions.push([angle.cos() * self.radius, angle.sin() * self.radius, 0.0]);
        }

        let normals = vec![[0.0, 0.0, 1.0]; (self.sides + 1) as usize];

        let uvs = positions
            .iter()
            .map(|v| {
                [
                    v[0] / self.radius / 2.0 + 0.5,
                    v[1] / self.radius / 2.0 + 0.5,
                ]
            })
            .collect::<Vec<[f32; 2]>>();

        let mut indices = Vec::with_capacity(self.sides as usize * 3);
        for i in 1..(self.sides) {
            indices.push(0);
            indices.push(i);
            indices.push(i + 1);
        }
        indices.push(0);
        indices.push(self.sides);
        indices.push(1);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }

    fn outline_mesh(&self) -> Mesh {
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

        let mut positions = Vec::with_capacity(self.sides as usize + 1);
        for i in 0..self.sides {
            let angle = (i as f32) / (self.sides as f32) * TAU;
            positions.push([angle.cos() * self.radius, angle.sin() * self.radius, 0.0]);
        }

        let normals = vec![[0.0, 0.0, 1.0]; (self.sides) as usize];

        let uvs = positions
            .iter()
            .map(|v| {
                [
                    v[0] / self.radius / 2.0 + 0.5,
                    v[1] / self.radius / 2.0 + 0.5,
                ]
            })
            .collect::<Vec<[f32; 2]>>();

        let mut indices = positions
            .iter()
            .enumerate()
            .map(|(i, _)| i as u32)
            .collect::<Vec<_>>();
        indices.push(0);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}

impl From<RegPoly> for Mesh {
    fn from(polygon: RegPoly) -> Self {
        match polygon.fill {
            true => polygon.fill_mesh(),
            false => polygon.outline_mesh(),
        }
    }
}
