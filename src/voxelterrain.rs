#[macro_use]

//TODO: add some sort of information
pub struct TerrainVertex {
    pub position: [f32; 3],
}

const CHUNKWIDTH: usize = 5;
const CHUNKHEIGHT: usize = 5;
const CHUNKDEPTH: usize = CHUNKWIDTH;
const BLOCKSIZE: f32 = 1.0;
const HALFBLOCKSIZE: f32 = BLOCKSIZE / 2.0;

pub struct Chunk<T> {
    data: [[[T; CHUNKDEPTH]; CHUNKHEIGHT]; CHUNKWIDTH],
    offset: [f32; 3],
}

pub fn test() -> Chunk<bool> {
    return Chunk {
        data: testterrain(),
        offset: [0.0; 3],
    };
}

pub trait Solid {
    fn issolid(&self) -> bool;
}

impl<T: Solid> Chunk<T> {
    fn outofbound(x: isize, y: isize, z: isize) -> bool {
        (x < 0
            || y < 0
            || z < 0
            || x >= CHUNKWIDTH as isize
            || y >= CHUNKHEIGHT as isize
            || z >= CHUNKDEPTH as isize)
    }

    pub fn gen_terrain_vertexes(&self) -> Vec<[f32; 3]> {
        let mut vertexvec = vec![];
        for (x, rowy) in self.data.iter().enumerate() {
            for (y, rowz) in rowy.iter().enumerate() {
                for (z, zblock) in rowz.iter().enumerate() {
                    if zblock.issolid() {
                        //if the given block is solid it should be tested, wich of its faces shall be visible
                        let coordinates = [
                            x as f32 * BLOCKSIZE - self.offset[0],
                            y as f32 * BLOCKSIZE - self.offset[1],
                            z as f32 * BLOCKSIZE - self.offset[2],
                        ];
                        use super::cubemaker;
                        for (direction, intvec) in
                            cubemaker::Direction::all_directions_with_vector_isize().iter()
                        {
                            let xi = intvec[0] + x as isize;
                            let yi = intvec[1] + y as isize;
                            let zi = intvec[2] + z as isize;
                            if Self::outofbound(xi, yi, zi) {
                                vertexvec.extend_from_slice(&cubemaker::make_cube_face(
                                    coordinates,
                                    HALFBLOCKSIZE,
                                    *direction,
                                ));
                            } else {
                                let xi = xi as usize;
                                let yi = yi as usize;
                                let zi = zi as usize;
                                if !self.data[xi][yi][zi].issolid() {
                                    vertexvec.extend_from_slice(&cubemaker::make_cube_face(
                                        coordinates,
                                        HALFBLOCKSIZE,
                                        *direction,
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
        return vertexvec;
    }
}

impl Solid for bool {
    fn issolid(&self) -> bool {
        *self
    }
}

fn testterrain() -> [[[bool; CHUNKDEPTH]; CHUNKHEIGHT]; CHUNKWIDTH] {
    let mut test = [[[false; CHUNKDEPTH]; CHUNKHEIGHT]; CHUNKWIDTH];
    for (x, iterx) in test.iter_mut().enumerate() {
        for (y, itery) in iterx.iter_mut().enumerate() {
            for (z, iterz) in itery.iter_mut().enumerate() {
                //tested: if only the condition is x==0 than there will be 420 Vertexes, wich represent a 5x5 block tarrain with 70 faces
                if x == 0 || (x == y && y == z) {
                    *iterz = true;
                }
            }
        }
    }
    test
}
