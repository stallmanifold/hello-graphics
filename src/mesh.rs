use nalgebra::{BaseFloat, Point3};
use std::marker::PhantomData;
use std::ops;


type VertexIdx = usize;


struct VertexMap<N> where N: BaseFloat {
    data: Box<Vec<Point3<N>>>,
}

impl<N> VertexMap<N> where N: BaseFloat {
    fn with_capacity(n_verts: usize) -> VertexMap<N> {
        let data = Box::new(Vec::with_capacity(n_verts));
        VertexMap {
            data: data,
        }
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn push(&mut self, vertex: Point3<N>) {
        self.data.push(vertex);
    }

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.data.capacity()
    }

    fn as_slice(&self) -> &[Point3<N>] {
        &self.data
    }
}

impl<N> ops::Index<VertexIdx> for VertexMap<N> where N: BaseFloat {
    type Output = Point3<N>;

    fn index(&self, _index: VertexIdx) -> &Self::Output {
        &self.data[_index]
    }
}

/// A Face is a triangle on the face of a simplex. The face stores the
/// vertices in clockwise order.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Face<N> {
    pub v0: VertexIdx,
    pub v1: VertexIdx,
    pub v2: VertexIdx,
    _phantom: PhantomData<N>,
}

impl<N> ops::Index<usize> for Face<N> where N: BaseFloat {
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

impl<N> FaceMap<N> where N: BaseFloat {
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

    #[inline]
    fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    fn capacity(&self) -> usize {
        self.data.capacity()
    }

    fn as_slice(&self) -> &[Face<N>] {
        &self.data
    }
}

impl<N> ops::Index<FaceIdx> for FaceMap<N> {
    type Output = Face<N>;

    fn index(&self, _index: usize) -> &Self::Output {
        &self.data[_index]
    }
}

/// Represents a static triangle mesh. Here are are assuming the scenes are not 
/// changing much, we would have to switch to something like a wing-edge
/// graph to represent the edges and faces of the simplices to make them
/// easy to manipulate dynamically, i.e. creating and deleting lots of triangles.
struct Mesh<N> where N: BaseFloat {
    vertex_table: VertexMap<N>,
    face_table: FaceMap<N>,
}

impl<N> Mesh<N> where N: BaseFloat {
    /// Create a mesh with at most `nverts` vertices, `nedges` edges, and
    /// `nfaces` faces.
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
}


#[cfg(test)]
mod tests {

}
