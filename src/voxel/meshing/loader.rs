use super::generate_mesh_vertices;

use crate::{
    assets::{BoundedMesh, IndexedPosColorNormVertices, MeshLoader},
    voxel::{Voxel, VoxelArrayMaterialId, VoxelInfo},
};

use amethyst::{assets::ProgressCounter, core::ecs::prelude::*};
use ilattice3 as lat;
use ilattice3::ChunkedPaletteLatticeMap;
use std::collections::HashMap;

/// Loads the vertices for chunks into `ChunkMesh` objects.
#[derive(SystemData)]
pub struct VoxelMeshLoader<'a> {
    pub mesh_loader: MeshLoader<'a>,
}

#[derive(Clone)]
pub struct ChunkMesh {
    pub material_array_id: VoxelArrayMaterialId,
    pub mesh: BoundedMesh,
}

#[derive(Default)]
pub struct VoxelMeshes {
    pub chunk_meshes: HashMap<lat::Point, ChunkMesh>,
}

impl<'a> VoxelMeshLoader<'a> {
    pub fn start_loading_all_chunks(
        &mut self,
        voxels: &ChunkedPaletteLatticeMap<VoxelInfo, Voxel>,
        progress: &mut ProgressCounter,
    ) -> VoxelMeshes {
        let chunk_meshes = voxels
            .iter_chunks_with_boundary()
            .filter_map(|(chunk_key, chunk_and_boundary)| {
                let vertices = generate_mesh_vertices(&chunk_and_boundary);

                vertices.map(|v| (*chunk_key, self.start_loading_chunk(v, progress)))
            })
            .collect();

        VoxelMeshes { chunk_meshes }
    }

    pub fn start_loading_chunk(
        &self,
        vertices: IndexedPosColorNormVertices,
        progress: &mut ProgressCounter,
    ) -> ChunkMesh {
        let mesh = self
            .mesh_loader
            .start_loading_pos_color_norm_mesh(vertices, &mut *progress);

        ChunkMesh {
            // TODO: support multiple array materials
            material_array_id: VoxelArrayMaterialId(1),
            mesh,
        }
    }
}
