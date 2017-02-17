use nalgebra::{BaseFloat, Point3};


type EdgeInd = usize;
type VertInd = usize;
type FaceInd = usize;


struct Edge {
    head:  VertInd,
    tail:  VertInd,
    left:  FaceInd,
    right: FaceInd,
    lprev: EdgeInd,
    lnext: EdgeInd,
    rprev: EdgeInd,
    rnext: EdgeInd,
}

struct Vertex<N> where N: BaseFloat {
    position: Point3<N>,
    // An incident edge to the vertex.
    edge: EdgeInd,
}

struct Face {
    v0: VertInd,
    v1: VertInd,
    v2: VertInd,
    // Normal vectors, etc.
    // An adjacent edge to the face.
    edge: EdgeInd,
}

/// A triangle mesh. Here are are assuming the scenes are not changing much,
/// otherwise using vectors to tabulate simplex data will be very slow
/// if we are going to be adding and deleting lots of simplices.
struct Mesh<N> where N: BaseFloat {
    vertex_table: Vec<Vertex<N>>,
    edge_table: Vec<Edge>,
    face_table: Vec<Face>,
}

impl<N> Mesh<N> where N: BaseFloat {
    /// Create a mesh with at most `nverts` vertices, `nedges` edges, and
    /// `nfaces` faces.
    fn new(nverts: usize, nedges: usize, nfaces: usize) -> Mesh<N> {
        Mesh {
            vertex_table: Vec::with_capacity(nverts),
            edge_table:   Vec::with_capacity(nedges),
            face_table:   Vec::with_capacity(nfaces), 
        }
    }

    #[inline]
    fn edge_count(&self) -> usize {
        self.vertex_table.len()
    }

    #[inline]
    fn face_count(&self) -> usize {
        self.face_table.len()
    }

    #[inline]
    fn vertex_count(&self) -> usize {
        self.edge_table.len()
    }
}


#[cfg(test)]
mod tests {

}
