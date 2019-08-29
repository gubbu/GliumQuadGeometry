#[derive(Clone, Copy)]
pub enum Direction {
    //the back face lies in the x,y Plane and is only visible when viewing from -z towards +z
    BACK,
    FRONT,
    TOP,
    BUTTOM,
    RIGHT,
    LEFT,
}

impl Direction {
    pub fn all_directions() -> [Direction; 6] {
        [
            Direction::BACK,
            Direction::FRONT,
            Direction::TOP,
            Direction::BUTTOM,
            Direction::RIGHT,
            Direction::LEFT,
        ]
    }
    pub fn all_directions_with_vector_isize() -> [(Direction, [isize; 3]); 6] {
        [
            (Direction::BACK, [0, 0, -1]),
            (Direction::FRONT, [0, 0, 1]),
            (Direction::TOP, [0, 1, 0]),
            (Direction::BUTTOM, [0, -1, 0]),
            (Direction::RIGHT, [1, 0, 0]),
            (Direction::LEFT, [-1, 0, 0]),
        ]
    }

    pub fn all_directions_with_vector_f32() -> [(Direction, [f32; 3]); 6] {
        [
            (Direction::BACK, [0f32, 0f32, -1f32]),
            (Direction::FRONT, [0f32, 0f32, 1f32]),
            (Direction::TOP, [0f32, 1f32, 0f32]),
            (Direction::BUTTOM, [0f32, -1f32, 0f32]),
            (Direction::RIGHT, [1f32, 0f32, 0f32]),
            (Direction::LEFT, [-1f32, 0f32, 0f32]),
        ]
    }
}

///generate a cube face, where coordinates are the cubes center written for back face culling and right hand coordinate system using the halfed size of the cube (positive z-devide -Perspective Matrix)
pub fn make_cube_face(coordinates: [f32; 3], halfsize: f32, dir: Direction) -> [[f32; 3]; 6] {
    match dir {
        // Top face [1] = y = const. +, Counterclock
        Direction::TOP => [
            [
                coordinates[0] + halfsize,
                coordinates[1] + halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] + halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] + halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] + halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] + halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] + halfsize,
                coordinates[2] + halfsize,
            ],
        ],
        // Buttom face [1] = y = const. -, Clockwise
        Direction::BUTTOM => [
            [
                coordinates[0] + halfsize,
                coordinates[1] - halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] - halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] - halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] - halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] - halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] - halfsize,
                coordinates[2] + halfsize,
            ],
        ],
        //Left [0] = const. - halfsize Counterclockwise
        Direction::LEFT => [
            [
                coordinates[0] - halfsize,
                coordinates[1] + halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] - halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] - halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] - halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] + halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] + halfsize,
                coordinates[2] + halfsize,
            ],
        ],
        //Clockwise [0] = +
        Direction::RIGHT => [
            [
                coordinates[0] + halfsize,
                coordinates[1] + halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] + halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] - halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] - halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] - halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] + halfsize,
                coordinates[2] + halfsize,
            ],
        ],
        //Front [z] = const +clockwise
        Direction::FRONT => [
            [
                coordinates[0] + halfsize,
                coordinates[1] + halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] - halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] - halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] - halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] + halfsize,
                coordinates[2] + halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] + halfsize,
                coordinates[2] + halfsize,
            ],
        ],
        //BACK: Counterclockwise
        Direction::BACK => [
            [
                coordinates[0] + halfsize,
                coordinates[1] + halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] + halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] - halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] - halfsize,
                coordinates[1] - halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] - halfsize,
                coordinates[2] - halfsize,
            ],
            [
                coordinates[0] + halfsize,
                coordinates[1] + halfsize,
                coordinates[2] - halfsize,
            ],
        ],
    }
}
