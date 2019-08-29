const ALPHABET: &[&[Line]] = &[
    //A
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //B
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //C
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //D
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //E
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0 / 2.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //F
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0 / 2.0,
        },
    ],
    //G
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 4.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 4.0,
            x1: 4.0,
            x2: 5.0,
        },
    ],
    //H
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //I:
    &[Line::Vertical {
        x: 2.5,
        y1: 0.0,
        y2: 5.0,
    }],
    //J
    &[
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //K TODO
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::AdvLine([0.0, 2.5, 5.0, 0.0]),
        Line::AdvLine([0.0, 2.5, 5.0, 5.0]),
    ],
    //L
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //M
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 2.5,
            y1: 0.0,
            y2: 2.5,
        },
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //N
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::AdvLine([0.0, 0.0, 5.0, 5.0]),
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
    ],
    //O
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //P
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 2.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //Q
    &[
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 2.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //R
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 2.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::AdvLine([0.0, 2.0, 5.0, 5.0]),
    ],
    //S
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 2.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 2.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //T
    &[
        Line::Vertical {
            x: 2.5,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //U
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //V
    &[
        Line::AdvLine([0.0, 0.0, 2.5, 5.0]),
        Line::AdvLine([5.0, 0.0, 2.5, 5.0]),
    ],
    //W
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 2.5,
            y1: 2.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //X
    &[
        Line::AdvLine([0.0, 0.0, 5.0, 5.0]),
        Line::AdvLine([0.0, 5.0, 5.0, 0.0]),
    ],
    //Y
    &[
        Line::Vertical {
            x: 2.5,
            y1: 2.5,
            y2: 5.0,
        },
        Line::AdvLine([0.0, 0.0, 2.5, 2.5]),
        Line::AdvLine([2.5, 2.5, 5.0, 0.0]),
    ],
    //Z
    &[
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::AdvLine([5.0, 0.0, 0.0, 5.0]),
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //.
    &[
        Line::Vertical {
            x: 2.0,
            y1: 4.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 3.0,
            y1: 4.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 4.0,
            x1: 2.0,
            x2: 3.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 2.0,
            x2: 3.0,
        },
    ],
    //1
    &[
        Line::Vertical {
            x: 2.5,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 2.3,
            x2: 2.7,
        },
        Line::AdvLine([2.5, 0.0, 2.3, 2.3]),
    ],
    //2
    &[
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::AdvLine([5.0, 0.0, 0.0, 5.0]),
    ],
    //3
    &[
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 5.0 / 2.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //4
    &[
        Line::Vertical {
            x: 2.5,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 2.5,
            x1: 0.0,
            x2: 5.0,
        },
        Line::AdvLine([2.5, 0.0, 0.0, 2.5]),
    ],
    //5
    &[
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 2.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 2.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
    ],
    //6
    &[
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Vertical {
            x: 5.0,
            y1: 2.0,
            y2: 5.0,
        },
    ],
    //7
    &[
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::AdvLine([5.0, 0.0, 0.0, 5.0]),
    ],
    //8
    &[
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 5.0,
        },
    ],
    //9
    &[
        Line::Vertical {
            x: 5.0,
            y1: 0.0,
            y2: 5.0,
        },
        Line::Horizontal {
            y: 0.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 2.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Horizontal {
            y: 5.0,
            x1: 0.0,
            x2: 5.0,
        },
        Line::Vertical {
            x: 0.0,
            y1: 0.0,
            y2: 2.0,
        },
    ],
    // SLASH
    &[Line::AdvLine([0.0, 5.0, 5.0, 0.0])],
    // - for floating point output
    &[Line::Horizontal {
        y: 2.5,
        x1: 0.0,
        x2: 5.0,
    }],
];

enum Line {
    Vertical { x: f32, y1: f32, y2: f32 },
    Horizontal { y: f32, x1: f32, x2: f32 },
    AdvLine([f32; 4]),
}

fn char2index(thechar: char) -> &'static [Line] {
    match thechar {
        'A' => {
            return ALPHABET[0];
        }
        'B' => {
            return ALPHABET[1];
        }
        'C' => {
            return ALPHABET[2];
        }
        'D' => {
            return ALPHABET[3];
        }
        'E' => {
            return ALPHABET[4];
        }
        'F' => {
            return ALPHABET[5];
        }
        'G' => {
            return ALPHABET[6];
        }
        'H' => {
            return ALPHABET[7];
        }
        'I' => {
            return ALPHABET[8];
        }
        'J' => {
            return ALPHABET[9];
        }
        'K' => {
            return ALPHABET[10];
        }
        'L' => {
            return ALPHABET[11];
        }
        'M' => {
            return ALPHABET[12];
        }
        'N' => {
            return ALPHABET[13];
        }
        'O' => {
            return ALPHABET[14];
        }
        'P' => {
            return ALPHABET[15];
        }
        'Q' => {
            return ALPHABET[16];
        }
        'R' => {
            return ALPHABET[17];
        }
        'S' => {
            return ALPHABET[18];
        }
        'T' => {
            return ALPHABET[19];
        }
        'U' => {
            return ALPHABET[20];
        }
        'V' => {
            return ALPHABET[21];
        }
        'W' => {
            return ALPHABET[22];
        }
        'X' => {
            return ALPHABET[23];
        }
        'Y' => {
            return ALPHABET[24];
        }
        'Z' => {
            return ALPHABET[25];
        }
        '0' => return ALPHABET[14],
        '1' => return ALPHABET[27],
        //2 gets rendered as s
        '2' => return ALPHABET[18],
        '3' => return ALPHABET[29],
        '4' => return ALPHABET[30],
        '5' => return ALPHABET[31],
        '6' => return ALPHABET[32],
        '7' => return ALPHABET[33],
        '8' => return ALPHABET[34],
        '9' => return ALPHABET[35],
        '/' => return ALPHABET[36],
        '-' => return ALPHABET[37],
        _ => return ALPHABET[26],
    }
}

#[macro_use]
#[derive(Clone, Copy)]
pub struct Textvertex {
    position: [f32; 2],
    color: [f32; 3],
}
implement_vertex!(Textvertex, position, color);

/// when using this libarry to render text this shader can be used to draw the given line information onto the screen.
extern crate glium;
pub fn textvertshader<'a>(
    display: &'a glium::Display,
) -> (glium::Program, glium::DrawParameters<'a>) {
    let textrender_src = r#"
    #version 140
    in vec2 position;
    in vec3 color;
    out vec4 textcolor;
    void main(){
        textcolor = vec4(color, 1.0);
        gl_Position = vec4(position, 1.0, 1.0);
    }
    "#;
    let textfragment_src = r#"
    #version 140
    out vec4 colour;
    in vec4 textcolor;
    void main(){
        colour = textcolor;
    }
    "#;

    return (
        glium::Program::from_source(display, textrender_src, textfragment_src, None).unwrap(),
        glium::draw_parameters::DrawParameters {
            polygon_mode: glium::draw_parameters::PolygonMode::Line,
            ..Default::default()
        },
    );
}

/// create a new vertexbuffer describing the text data given to it
pub fn text2vertexbuffer(
    data: &str,
    display: &glium::Display,
    scaley: f32,
    scalex: f32,
    xc: f32,
    yc: f32,
) -> (glium::VertexBuffer<Textvertex>, glium::index::NoIndices) {
    let mut lines = Vec::new();
    text2lines(data, &mut lines, scaley, scalex, xc, yc);
    return (
        glium::VertexBuffer::new(display, &lines).unwrap(),
        glium::index::NoIndices(glium::index::PrimitiveType::LinesList),
    );
}

/// Convert a given &str to a list of Vertexes that, when rendered with Polygonmode Line and Linelist indices represent primitve bitmap fonts.
/// scalex: the scalefactor on the x-Axis. The Size of one letterin Opengl Window coordinates is scalex*5.0.
/// scaley: the scalefactor on the y-Axis. The Size of one letter in Opengl Window coordinates is scaley*5.0.
//font size of every letter in relative screen coordinates = scaley*5.0;
//TODO use color fields of the vertexes
pub fn text2lines(
    data: &str,
    lines: &mut Vec<Textvertex>,
    scaley: f32,
    scalex: f32,
    xc: f32,
    yc: f32,
) {
    lines.clear();
    let mut cursorx = 0.0;
    let mut cursory = 0.0;
    for part in data.chars() {
        if part == '\n' {
            cursorx = 0.0;
            cursory -= 6.0 * scaley;
            continue;
        } else if part == ' ' {
            //println!("SPACE found");
            cursorx += 6.0 * scalex;
            continue;
        } else {
            for line in char2index(part).iter() {
                match line {
                    Line::Vertical { x, y1, y2 } => {
                        lines.push(Textvertex {
                            position: [
                                xc + cursorx + *x * scalex,
                                yc + cursory + (5.0 - *y1) * scaley,
                            ],
                            color: [1.0; 3],
                        });
                        lines.push(Textvertex {
                            position: [
                                xc + cursorx + *x * scalex,
                                yc + cursory + (5.0 - *y2) * scaley,
                            ],
                            color: [1.0; 3],
                        });
                    }
                    Line::Horizontal { y, x1, x2 } => {
                        lines.push(Textvertex {
                            position: [
                                xc + cursorx + *x1 * scalex,
                                yc + cursory + (5.0 - *y) * scaley,
                            ],
                            color: [1.0; 3],
                        });
                        lines.push(Textvertex {
                            position: [
                                xc + cursorx + *x2 * scalex,
                                yc + cursory + (5.0 - *y) * scaley,
                            ],
                            color: [1.0; 3],
                        });
                    }
                    Line::AdvLine(line) => {
                        lines.push(Textvertex {
                            position: [
                                xc + cursorx + line[0] * scalex,
                                yc + cursory + (5.0 - line[1]) * scaley,
                            ],
                            color: [1.0; 3],
                        });
                        lines.push(Textvertex {
                            position: [
                                xc + cursorx + line[2] * scalex,
                                yc + cursory + (5.0 - line[3]) * scaley,
                            ],
                            color: [1.0; 3],
                        });
                    }
                }
            }
        }

        cursorx += 6.0 * scalex;
    }
}
