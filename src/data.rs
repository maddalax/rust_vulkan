use cgmath::{Vector3, Zero};

use crate::Instance;
use crate::structs::Vertex;

pub const CUBE: &[Vertex] = &[
    Vertex { position: [-1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
    // bottom (0, 0, -1.0)
    Vertex { position: [-1.0, 1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },
    // right (1.0, 0, 0)
    Vertex { position: [1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    // left (-1.0, 0, 0)
    Vertex { position: [-1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, 1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },
    // front (0, 1.0, 0)
    Vertex { position: [1.0, 1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, 1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, 1.0, 1.0], color: [1.0, 0.0, 0.0] },
    // back (0, -1.0, 0)
    Vertex { position: [1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, -1.0, 1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [1.0, -1.0, -1.0], color: [1.0, 0.0, 0.0] },
];

pub const CUBE_INDICES: &[u16] = &[
    0, 1, 2, 2, 3, 0, // top
    4, 5, 6, 6, 7, 4, // bottom
    8, 9, 10, 10, 11, 8, // right
    12, 13, 14, 14, 15, 12, // left
    16, 17, 18, 18, 19, 16, // front
    20, 21, 22, 22, 23, 20, // back
];

pub const TRIANGLE: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E

    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E

    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];


pub const TRIANGLE_INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
    /* padding */ 0,
];