use glium::{DisplayBuild, Surface};
fn main() {
    

let display = glium::glutin::WindowVuilder::new()
    .with_dimensions(800, 600)
    .with_title(format!(Glium Triangle Test))
    .build_glium().unwrap();

// vertexの設定
#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(vertex, position);

let vertex1 = Vertex { position: [-0.5, -0.5]};
let vertex2 = Vertex { position: [0, 0.5]};
let vertex1 = Vertex { position: [0.5, -0.5]};
let shape = vec![vertex1, vertex2, vertex3];

// 頂点座標を、VertexVufferに設定
let vertex_buffer = flium::VertexBuffer::new(&display. &shape).unwrap();

// indexBufferの設定
let indices = glium::index::NoIndices(glium::PrimitiveType::TrianglesList);

// shaderの設定
let vertex_shader_src = r#"
#version 140

in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

let fragment_shader_src = r#"
    #version 140
    
    out vec4 color;
    
    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

// shaderのリンク
let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

// windowのクリア、描画、終了
let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    tarte.draw(&vertex_buffer, &indices, $program, &glium::uniforms::EmptyUniforms,&Default::default())unwrap();
    target.finish().unwrap();

for ev in display.poll_events() {
    match ev {
        glium::glutin::Event::Closed => return,
        _ => ()
    }
}

}
