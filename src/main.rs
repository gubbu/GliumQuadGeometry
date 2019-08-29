#[macro_use]
extern crate glium;
mod myglview;
use myglview::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 3],
}

implement_vertex!(Vertex, position);

fn main() {
    use glium::{glutin, Surface};

    let mut events_loop = glium::glutin::EventsLoop::new();
    let wb = glium::glutin::WindowBuilder::new();
    let cb = glium::glutin::ContextBuilder::new().with_depth_buffer(24);
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    //The Vertex buffer that is rendered with the camera
    let vertex_buffer = glium::VertexBuffer::new(&display, &{
        let mut x = Vec::new();
        x.push(Vertex{position: [0.0;3]});
        x
    })
    .unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let vertex_shader_src = r#"
        #version 150
        in vec3 position;
        void main() {
            gl_Position = vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150
        out vec3 colour;
        void main() {
            colour = vec3(1.0);
        }
    "#;

    let geometry_shader_src = r#"
        #version 150
        uniform mat4 transl;
        uniform mat4 persp;

        layout (points) in;
        layout (triangle_strip, max_vertices = 6) out;
        //now no data other than position in vec3 info[]

        //generate a quad for ich point inputted
        void main(void){
            vec4 offset = vec4(1.0, 1.0, 0.0, 0.0);
            vec4 newvertex = offset + gl_in[0].gl_Position;
            gl_Position = persp*transl*newvertex;
            EmitVertex();

            offset = vec4(-1.0, 1.0, 0.0, 0.0);
            newvertex = offset + gl_in[0].gl_Position;
            gl_Position = persp*transl*newvertex;
            EmitVertex();

            offset = vec4(-1.0, -1.0, 0.0, 0.0);
            newvertex = offset + gl_in[0].gl_Position;
            gl_Position = persp*transl*newvertex;
            EmitVertex();

            EndPrimitive();

            offset = vec4(-1.0, -1.0, 0.0, 0.0);
            newvertex = offset + gl_in[0].gl_Position;
            gl_Position = persp*transl*newvertex;
            EmitVertex();

            offset = vec4(+1.0, -1.0, 0.0, 0.0);
            newvertex = offset + gl_in[0].gl_Position;
            gl_Position = persp*transl*newvertex;
            EmitVertex();

            offset = vec4(+1.0, +1.0, 0.0, 0.0);
            newvertex = offset + gl_in[0].gl_Position;
            gl_Position = persp*transl*newvertex;
            EmitVertex();

            EndPrimitive();
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, Some(geometry_shader_src))
            .unwrap();

    //one block distance away
    let mut camera = Camera{position: [0.5, 0.5, -1.0], rotation: [0.0;2]};
    //creating lines for rendering the camera position
    //after setting the position the camerematrix has to be updatet:
    let mut cameramatrix = [[0.0f32; 4]; 4];
    camera.genmatrix(&mut cameramatrix);
    //w shall not get zeroed, because otherwise z-devide does not work (?):
    cameramatrix[3][3] = 1.0;
    let perspectivematrix = gen_perspective(45.0, 100.0, 0.4, 1.0);

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            ..Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        polygon_mode: glium::draw_parameters::PolygonMode::Fill,
        ..Default::default()
    };

    let mut closed = false;
    while !closed {
        //1. DRAWCALL
        let mut target = display.draw();
        target.clear_color_and_depth((1.0, 0.0, 0.0, 1.0), 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniform!(transl: cameramatrix, persp: perspectivematrix),
                &params,
            )
            .unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            //2. EVENT HANDLING
            match ev {
                glutin::Event::WindowEvent { event, .. } => {
                    closed = glutin::WindowEvent::CloseRequested == event;
                },
                glutin::Event::DeviceEvent { event, .. } => {
                    if let glium::glutin::DeviceEvent::Motion { axis, value } = event {
                        if axis == 0 {
                            camera.mousemove(value as f32, 0.0, 0.005);
                        } else {
                            camera.mousemove(0.0, value as f32, 0.005);
                        }
                        camera.genmatrix(&mut cameramatrix);
                    } else if let glium::glutin::DeviceEvent::MouseWheel { delta } = event {
                        if let glium::glutin::MouseScrollDelta::LineDelta(_x, y) = delta {
                            //println!("DATA: {} {}", x,y);
                            camera.forward(y * 1.0);
                            //updating the view matrix
                            camera.genmatrix(&mut cameramatrix);
                        }
                        //println!("scroll detexted!");
                    }
                }
                _ => {}
            }
        });
        // CPU-WORKLOAD != 100%
        std::thread::sleep(std::time::Duration::from_millis(33));
    }
}
