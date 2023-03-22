use std::{
    fs::File,
    io::{BufReader, Read},
};

use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

use crate::util::serde_utils::Vector3F;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub submeshes: Vec<Submesh>,
}

#[derive(Error, Debug)]
pub enum MeshParseError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("Invalid mesh header, expected {expected:?} but got {actual:?}")]
    InvalidHeader { expected: [u8; 8], actual: [u8; 8] },
    #[error("Invalid block header, expected {expected:?} but got {actual:?}")]
    InvalidBlockHeader { expected: [u8; 4], actual: [u8; 4] },
    #[error("Invalid face vertex count, expected a multiple of 3 but got {actual:?}")]
    InvalidFaceCount { actual: u32 },
    #[error("Invalid submesh triangle count, expected a multiple of 3 but got {actual:?}")]
    InvalidTriangleCount { actual: u32 },
    #[error("Invalid submesh position, expected zeros but got {actual:?}")]
    InvalidSubmeshPosition { actual: u32 },
    #[error("Wrong submesh padding, expected zeros but got {actual:?}")]
    WrongSubmeshPadding { actual: [u8; 2] },
    #[error("Invalid submesh material, expected 0, 1, or 2, but got {actual:?}")]
    InvalidSubmeshMaterial { actual: u16 },
}

impl Mesh {
    pub fn load_file(file: File) -> Result<Self, MeshParseError> {
        let mut br = BufReader::new(file);

        const HEADER: [u8; 8] = [0x6D, 0x65, 0x73, 0x68, 0x07, 0x00, 0x01, 0x00];
        let header = br.read_bytes();
        if header != HEADER {
            Err(MeshParseError::InvalidHeader { expected: HEADER, actual: header })?
        }

        // Vertices

        let n_vertices = br.read_u16::<LittleEndian>()?;

        const BLOCK_HEADER: [u8; 4] = [0x13, 0x00, 0x00, 0x00];
        let block_header = br.read_bytes();
        if block_header != BLOCK_HEADER {
            Err(MeshParseError::InvalidBlockHeader {
                expected: BLOCK_HEADER,
                actual: block_header,
            })?
        }

        let vertices = (0..n_vertices)
            .map(|_| {
                Ok(Vertex {
                    position: Vector3F {
                        x: br.read_f32::<LittleEndian>()?,
                        y: br.read_f32::<LittleEndian>()?,
                        z: br.read_f32::<LittleEndian>()?,
                    },
                    color: Color {
                        r: br.read_u8()?,
                        g: br.read_u8()?,
                        b: br.read_u8()?,
                        a: br.read_u8()?,
                    },
                    normal: Vector3F {
                        x: br.read_f32::<LittleEndian>()?,
                        y: br.read_f32::<LittleEndian>()?,
                        z: br.read_f32::<LittleEndian>()?,
                    },
                })
            })
            .collect::<Result<Vec<_>, MeshParseError>>()?;

        // Faces

        println!("{}", vertices.len());

        let n_faces = br.read_u32::<LittleEndian>()?;
        if n_faces % 3 != 0 {
            Err(MeshParseError::InvalidFaceCount { actual: n_faces })?
        }
        let n_faces = n_faces / 3;

        let faces = (0..n_faces)
            .map(|_| {
                Ok(Face {
                    indices: [
                        br.read_u16::<LittleEndian>()? as _,
                        br.read_u16::<LittleEndian>()? as _,
                        br.read_u16::<LittleEndian>()? as _,
                    ],
                })
            })
            .collect::<Result<Vec<_>, MeshParseError>>()?;

        // Submeshes

        let n_submeshes = br.read_u16::<LittleEndian>()?;

        let submeshes = (0..n_submeshes)
            .map(|_| {
                let pos = br.read_u32::<LittleEndian>()?;
                if pos % 3 != 0 {
                    Err(MeshParseError::InvalidSubmeshPosition { actual: pos })?
                }
                let pos = pos / 3;

                let n_tris = br.read_u32::<LittleEndian>()?;
                if n_tris % 3 != 0 {
                    println!("non 3 {n_tris}");
                    // Err(MeshParseError::InvalidTriangleCount { actual: n_tris })?
                }
                let n_tris = n_tris / 3;

                const PADDING: [u8; 2] = [0x00, 0x00];
                let padding = br.read_bytes();
                if padding != PADDING {
                    Err(MeshParseError::WrongSubmeshPadding { actual: padding })?
                }

                let material = br.read_u16::<LittleEndian>()?;
                let material: Material = match material {
                    0 => Material::Normal,
                    1 => Material::Glass,
                    2 => Material::Emissive,
                    3 => Material::_Unknown,
                    _ => Err(MeshParseError::InvalidSubmeshMaterial { actual: material })?,
                };

                let cull_min = Vector3F {
                    x: br.read_f32::<LittleEndian>()?,
                    y: br.read_f32::<LittleEndian>()?,
                    z: br.read_f32::<LittleEndian>()?,
                };

                let cull_max = Vector3F {
                    x: br.read_f32::<LittleEndian>()?,
                    y: br.read_f32::<LittleEndian>()?,
                    z: br.read_f32::<LittleEndian>()?,
                };

                // unknown but not always zero
                br.read_u16::<LittleEndian>()?;

                let skip = br.read_u16::<LittleEndian>()?;
                br.seek_relative(skip as i64 - 2)?;

                br.seek_relative(14)?;

                let tris = (0..n_tris)
                    .map(|i| Ok(faces[(pos + i) as usize]))
                    .collect::<Result<Vec<_>, MeshParseError>>()?;

                Ok(Submesh { material, cull_min, cull_max, tris })
            })
            .collect::<Result<Vec<_>, MeshParseError>>()?;

        Ok(Self { vertices, faces, submeshes })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    pub position: Vector3F,
    pub color: Color,
    pub normal: Vector3F,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Face {
    pub indices: [usize; 3],
}

#[derive(Debug, Clone, PartialEq)]
pub struct Submesh {
    pub material: Material,
    pub cull_min: Vector3F,
    pub cull_max: Vector3F,
    pub tris: Vec<Face>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Material {
    Normal,
    Glass,
    Emissive,
    /// Unknown material that shows up in `lava_level.mesh` (and maybe others)
    _Unknown,
}

trait ReadBytes {
    fn read_bytes<const N: usize>(&mut self) -> [u8; N];
}

impl<R: Read> ReadBytes for R {
    fn read_bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut buf = [0; N];
        self.read_exact(&mut buf).unwrap();
        buf
    }
}
