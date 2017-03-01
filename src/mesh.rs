use nalgebra::{Point3};
use num_traits::Float;
use alga::general::Real;
use std::marker::PhantomData;
use std::ops;
use std::convert::AsRef;


type VertexIdx = usize;
type Vertex<N> = Point3<N>;

struct VertexMap<N> where N: Float + Real {
    data: Box<Vec<Vertex<N>>>,
}

impl<N> VertexMap<N> where N: Float + Real {
    fn with_capacity(n_verts: usize) -> VertexMap<N> {
        let data = Box::new(Vec::with_capacity(n_verts));
        VertexMap {
            data: data,
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, vertex: Vertex<N>) {
        self.data.push(vertex);
    }

    fn lookup(&self, index: VertexIdx) -> Option<&Vertex<N>> {
        self.data.get(index)
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.data.capacity()
    }

    fn as_slice(&self) -> &[Vertex<N>] {
        self.data.as_slice()
    }
}

impl<N> ops::Index<VertexIdx> for VertexMap<N> where N: Float + Real {
    type Output = Vertex<N>;

    fn index(&self, _index: VertexIdx) -> &Self::Output {
        &self.data[_index]
    }
}

impl<N> AsRef<[Vertex<N>]> for VertexMap<N> where N: Float + Real {
    fn as_ref(&self) -> &[Vertex<N>] {
        self.data.as_ref()
    }
}

/// A Face is a triangle on the outside of a simplex. The face stores the
/// vertices in clockwise order.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Face<N> {
    pub v0: VertexIdx,
    pub v1: VertexIdx,
    pub v2: VertexIdx,
    _phantom: PhantomData<N>,
}

impl<N> Face<N> {
    fn new(v0: VertexIdx, v1: VertexIdx, v2: VertexIdx) -> Face<N> {
        Face {
            v0: v0,
            v1: v1,
            v2: v2,
            _phantom: PhantomData,
        }
    }

    fn len(&self) -> usize {
        3
    }
}

impl<N> ops::Index<usize> for Face<N> where N: Float + Real {
    type Output = usize;

    fn index(&self, _index: usize) -> &Self::Output {
        match _index {
            0 => &self.v0,
            1 => &self.v1,
            2 => &self.v2,
            _ => {
                panic!("index out of bounds: length is 3 but index is {}", _index);
            }
        }
    }
}

type FaceIdx = usize;

struct FaceMap<N> {
    data: Box<Vec<Face<N>>>,
    _phantom: PhantomData<N>,
}

impl<N> FaceMap<N> where N: Float + Real {
    fn with_capacity(n_faces: usize) -> FaceMap<N> {
        let data = Box::new(Vec::with_capacity(n_faces));
        FaceMap {
            data: data,
            _phantom: PhantomData,
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, face: &Face<N>) {
        self.data.push(*face);
    }

    fn lookup(&self, index: FaceIdx) -> Option<&Face<N>> {
        self.data.get(index)
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.data.capacity()
    }

    fn as_slice(&self) -> &[Face<N>] {
        self.data.as_slice()
    }
}

impl<N> ops::Index<FaceIdx> for FaceMap<N> {
    type Output = Face<N>;

    fn index(&self, _index: usize) -> &Self::Output {
        &self.data[_index]
    }
}

impl<N> AsRef<[Face<N>]> for FaceMap<N> {
    fn as_ref(&self) -> &[Face<N>] {
        self.data.as_ref()
    }
}

/// Represents a static triangle mesh. Here are are assuming the scenes are not 
/// changing much, we would have to switch to something like a wing-edge
/// graph to represent the edges and faces of the simplices to make them
/// easy to manipulate dynamically, i.e. creating and deleting lots of triangles.
struct Mesh<N> where N: Float + Real {
    vertex_table: VertexMap<N>,
    face_table: FaceMap<N>,
}

impl<N> Mesh<N> where N: Float + Real {
    /// Create a mesh with at most `n_verts` vertices, and `n_faces` faces.
    fn with_dims(n_verts: usize, n_faces: usize) -> Mesh<N> {
        Mesh {
            vertex_table: VertexMap::with_capacity(n_verts),
            face_table:   FaceMap::with_capacity(n_faces),
        }
    }

    #[inline]
    fn vertex_count(&self) -> usize {
        self.vertex_table.len()
    }

    #[inline]
    fn face_count(&self) -> usize {
        self.face_table.len()
    }

    fn push_vertex(&mut self, vertex: Vertex<N>) {
        self.vertex_table.push(vertex);
    }

    fn push_face(&mut self, face: &Face<N>) {
        self.face_table.push(face);
    }

    fn vertices(&self) -> &[Vertex<N>] {
        self.vertex_table.as_slice()
    }

    fn faces(&self) -> &[Face<N>] {
        self.face_table.as_slice()
    }
}


#[cfg(test)]
mod tests {

}
