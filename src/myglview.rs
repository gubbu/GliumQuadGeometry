//fov in degrees
pub fn gen_perspective(fov: f32, far: f32, near: f32, aspect_ratio: f32) -> [[f32; 4]; 4] {
    let y_scale = 1.0 / (fov * std::f32::consts::PI / 180.0 / 2.0).tan();
    let x_scale = y_scale / aspect_ratio;
    [
        //prevent clipping
        [1.0 / (1.0 - 2.0 * near)/3.0, 0.0, 0.0, 0.0],
        [0.0, 1.0 / (1.0 - 2.0 * near)/3.0, 0.0, 0.0],
        [0.0, 0.0, (far + near) / (far - near), 1.0],
        [0.0, 0.0, 2.0 * far * near / (near - far), 0.0],
    ]
}

//first person Camera
/// rotation[0] rotation around y-Axis (x-z-Plane)
/// rotation[1] rotation around x-Axis (y-z-Plane)
#[derive(Debug, Default)]
pub struct Camera {
    pub position: [f32; 3],
    pub rotation: [f32; 2],
}

impl Camera {
    /// based on right hand perspectibe look along the positive z-Axis
    pub fn new() -> Camera {
        Camera {
            position: [0.0; 3],
            rotation: [0.0, 0.0],
        }
    }

    ///move forwad based on rotation and magnitude speed: can be negative
    pub fn forward(&mut self, speed: f32) {
        /*
        default: both rotations are 0: Moving down the z-Axis only.
        */
        self.position[0] += speed * self.rotation[0].sin() * self.rotation[1].cos();
        self.position[1] += speed * self.rotation[1].sin();
        self.position[2] += speed * self.rotation[0].cos() * self.rotation[1].cos();
    }

    /*
     * Remember: when you move forward, the world around you moves backwards to come closer to you
     * When you rotate to the right, you expect the points to your right to roate to the left to come into your field of view
     * (if no inverted axis)
     *used the following relation:
     * sin(-x) = -sin(x)
     * cos(-x) = cos(x)
     * your coordinate sysmtem is the center of the universe
     *
     * COLLUMN MAJOR ORDER!!!
     */

    pub fn genmatrix(&self, m: &mut [[f32; 4]; 4]) {
        //rotation matrix from https://de.wikipedia.org/wiki/Drehmatrix
        m[0][0] = self.rotation[0].cos();
        m[1][0] = 0.0;
        m[2][0] = -self.rotation[0].sin();

        m[0][1] = -self.rotation[1].sin() * self.rotation[0].sin();
        m[1][1] = self.rotation[1].cos();
        m[2][1] = -self.rotation[1].sin() * self.rotation[0].cos(); //-

        m[0][2] = self.rotation[1].cos() * self.rotation[0].sin(); //-
        m[1][2] = self.rotation[1].sin();
        m[2][2] = self.rotation[0].cos() * self.rotation[1].cos();
        //it makes a difference, if you rotate around the origin and than translate, because the distance to the coordinate center is important for rotation.
        m[3][0] =
            -m[0][0] * self.position[0] - m[1][0] * self.position[1] - m[2][0] * self.position[2];
        m[3][1] =
            -m[0][1] * self.position[0] - m[1][1] * self.position[1] - m[2][1] * self.position[2];
        m[3][2] =
            -m[0][2] * self.position[0] - m[1][2] * self.position[1] - m[2][2] * self.position[2];
    }

    pub fn mousemove(&mut self, horizontal: f32, vertical: f32, sensitivity: f32) {
        self.rotation[0] += horizontal * sensitivity;
        //dont let the player turn upside down
        //When the mouse gets moved updwards, vertical is negative, but rotation around x should increase:
        if vertical < 0.0 && self.rotation[1] < std::f32::consts::PI * 0.5 {
            self.rotation[1] -= vertical * sensitivity;
        } else if vertical > 0.0 && self.rotation[1] > -std::f32::consts::PI * 0.5 {
            self.rotation[1] -= vertical * sensitivity;
        }
    }
}
