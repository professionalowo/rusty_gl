use std::{fmt, path::PathBuf};

use crate::framework::{
    material::{Material, MaterialConversionError},
    mesh::normalize::NormalizeOptions,
};

use rmath::{vec2::Vec2, vec3::Vec3};

use gl_sys::{
    vao::VertexArrayObject,
    vbo::{Location, VBOError, VertexBufferObject},
};

pub mod load;
pub mod normalize;

#[derive(Debug, Default, Clone)]
pub struct VboData {
    pub vbo: VertexBufferObject,
    pub buffer_type: gl_sys::bindings::GLenum,
    pub dimensions: u32,
}

#[derive(Debug, Default)]
pub struct Mesh {
    pub vao: VertexArrayObject,
    pub ibo: VertexBufferObject,
    pub num_vertices: u32,
    pub num_indices: u32,
    pub vbos: [Option<VboData>; 4],
    pub primitive_type: gl_sys::bindings::GLenum,
}

impl Mesh {
    pub fn with_defaults() -> Self {
        Self {
            vao: VertexArrayObject::gen_vertex_arrays(),
            ibo: VertexBufferObject::gen_buffers(),
            num_vertices: 0,
            num_indices: 0,
            vbos: Default::default(),
            primitive_type: gl_sys::bindings::GL_TRIANGLES,
        }
    }

    pub fn bind(&self) {
        VertexArrayObject::bind_vertex_array(&self.vao);
    }
    pub fn unbind(&self) {
        VertexArrayObject::bind_vertex_array(&VertexArrayObject::zero());
    }
    pub fn draw(&self) {
        gl_sys::draw_elements(
            gl_sys::bindings::GL_TRIANGLES,
            self.num_indices as i32,
            gl_sys::bindings::GL_UNSIGNED_INT,
        );
    }

    pub fn from_ai_mesh(mesh: &assimp::Mesh<'_>) -> Result<Self, MeshLoadError> {
        let mut positions: Vec<Vec3<f32>> = Vec::with_capacity(mesh.num_vertices() as _);
        let mut normals: Vec<Vec3<f32>> = Vec::with_capacity(if mesh.has_normals() {
            mesh.num_vertices() as _
        } else {
            0
        });
        let mut texcoords: Vec<Vec2<f32>> = Vec::with_capacity(if mesh.has_texture_coords(0) {
            mesh.num_vertices() as _
        } else {
            0
        });
        let mut tangents: Vec<Vec3<f32>> =
            Vec::with_capacity(if mesh.has_tangents_and_bitangents() {
                mesh.num_vertices() as _
            } else {
                0
            });

        for i in 0..mesh.num_vertices() {
            let a_pos = mesh
                .get_vertex(i)
                .ok_or(MeshLoadError::MeshConversionFailed(format!(
                    "Could not get vertex[{i}]"
                )))?;
            positions.push(a_pos.into());

            if mesh.has_normals() {
                let a_norm = mesh
                    .get_normal(i)
                    .ok_or(MeshLoadError::MeshConversionFailed(format!(
                        "Could not get normal[{i}]"
                    )))?;
                normals.push(a_norm.into());
            }
            if mesh.has_texture_coords(0) {
                let a_tc =
                    mesh.get_texture_coord(0, i)
                        .ok_or(MeshLoadError::MeshConversionFailed(format!(
                            "Could not get texture_coord[{i}]"
                        )))?;
                texcoords.push(Vec2::new(a_tc.x, a_tc.y))
            }
            if mesh.has_tangents_and_bitangents() {
                let a_tan = mesh
                    .get_tangent(i)
                    .ok_or(MeshLoadError::MeshConversionFailed(format!(
                        "Could not get tangent[{i}]"
                    )))?;
                tangents.push(a_tan.into());
            }
        }

        let indices: Vec<u32> = mesh
            .face_iter()
            .filter(|f| f.num_indices == 3)
            .flat_map(|f| [f[0], f[1], f[2]])
            .collect();

        let mut m = Self::with_defaults();
        m.add_vbo(0, 3, &positions)?;
        if !normals.is_empty() {
            m.add_vbo(1, 3, &normals)?;
        }
        if !texcoords.is_empty() {
            m.add_vbo(2, 2, &texcoords)?;
        }
        if !tangents.is_empty() {
            m.add_vbo(3, 3, &tangents)?;
        }
        m.num_indices = indices.len() as _;
        VertexArrayObject::bind_vertex_array(&m.vao);
        VertexBufferObject::bind_buffer(gl_sys::bindings::GL_ELEMENT_ARRAY_BUFFER, &m.ibo);
        VertexBufferObject::buffer_data(
            gl_sys::bindings::GL_ELEMENT_ARRAY_BUFFER,
            &indices,
            gl_sys::bindings::GL_STATIC_DRAW,
        )?;
        VertexArrayObject::bind_vertex_array(&VertexArrayObject::zero());
        VertexBufferObject::bind_buffer(
            gl_sys::bindings::GL_ELEMENT_ARRAY_BUFFER,
            &VertexBufferObject::zero(),
        );
        Ok(m)
    }

    fn add_vbo<T>(
        &mut self,
        index: usize,
        dimensions: u32,
        data: &[T],
    ) -> Result<(), MeshLoadError> {
        if self.num_vertices != 0 && self.num_vertices != data.len() as _ {
            return Err(MeshLoadError::UnequalNumberOfVertices {
                expected: self.num_indices,
                actual: data.len() as _,
            });
        }
        self.num_vertices = data.len() as _;

        let buffer_type = gl_sys::bindings::GL_ARRAY_BUFFER;

        VertexArrayObject::bind_vertex_array(&self.vao);
        let vbo = VertexBufferObject::gen_buffers();
        VertexBufferObject::bind_buffer(buffer_type, &vbo);
        VertexBufferObject::buffer_data(buffer_type, &data, gl_sys::bindings::GL_STATIC_DRAW)?;
        let loc = Location(index as _);

        VertexBufferObject::enable_vertex_attrib_array(&loc);
        VertexBufferObject::vertex_attrib_pointer(
            &loc,
            dimensions,
            gl_sys::bindings::GL_FLOAT,
            false,
            (dimensions * 4) as _,
            None,
        )?;
        VertexArrayObject::bind_vertex_array(&VertexArrayObject::zero());
        VertexBufferObject::bind_buffer(buffer_type, &VertexBufferObject::zero());

        self.vbos[index] = Some(VboData {
            vbo,
            buffer_type,
            dimensions,
        });

        Ok(())
    }
}

#[derive(Debug)]
pub enum MeshLoadError {
    LoadFailed(String),
    InvalidPath(PathBuf),
    InvalidParent(PathBuf),
    MaterialNotFound(usize),
    MaterialConversionFailed(MaterialConversionError),
    MeshConversionFailed(String),
    UnequalNumberOfVertices { expected: u32, actual: u32 },
    VboError(VBOError),
}

impl fmt::Display for MeshLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadFailed(s) => write!(f, "Failed to load mesh: {}", s),
            Self::InvalidPath(p) => write!(f, "Invalid path: {:?}", p),
            Self::InvalidParent(p) => write!(f, "Invalid parent: {:?}", p),
            Self::MaterialNotFound(index) => write!(f, "Material not found: {}", index),
            Self::MaterialConversionFailed(e) => fmt::Display::fmt(e, f),
            Self::MeshConversionFailed(reason) => write!(f, "Mesh conversion failed: {:?}", reason),
            Self::VboError(e) => fmt::Display::fmt(e, f),
            Self::UnequalNumberOfVertices {
                expected: e,
                actual: a,
            } => write!(f, "Expected {e} but got {a} vertices"),
        }
    }
}

impl From<VBOError> for MeshLoadError {
    fn from(value: VBOError) -> Self {
        Self::VboError(value)
    }
}

impl From<MaterialConversionError> for MeshLoadError {
    fn from(value: MaterialConversionError) -> Self {
        Self::MaterialConversionFailed(value)
    }
}
