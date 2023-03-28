use js_sys::{Array, Float32Array};
use wasm_bindgen::prelude::*;
use web_sys::{WebGl2RenderingContext, WebGlProgram};

type Gl = WebGl2RenderingContext;

const VERTEX_SHADER: &'static str = "
    #version 300 es
    in vec4 aPosition;
    out vec4 vColor;
    void main() {
        gl_PointSize = 1.0;
        gl_Position = aPosition;
        vColor = vec4(gl_Position.xy / 2.0 + 0.5, 0.65, 1.0);
    }
";

const FRAGMENT_SHADER: &'static str = "
    #version 300 es
    precision mediump float;
    out vec4 fColor;
    in vec4 vColor;
    void main() {
        // fColor = vec4(1.0, 0.0, 0.0, 1.0);
        fColor = vColor;
    }
";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[allow(unused_macros)]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[allow(unused_macros)]
macro_rules! console_error {
    ($($t:tt)*) => (error(&format_args!($($t)*).to_string()))
}

fn init_shader(
    gl: &Gl,
    vertex_shader: &str,
    fragment_shader: &str,
) -> Result<WebGlProgram, &'static str> {
    let vs = gl
        .create_shader(Gl::VERTEX_SHADER)
        .ok_or("failed to create vertex shader")?;
    gl.shader_source(&vs, vertex_shader.trim());
    gl.compile_shader(&vs);

    let fs = gl
        .create_shader(Gl::FRAGMENT_SHADER)
        .ok_or("failed to create fragment shader")?;
    gl.shader_source(&fs, fragment_shader.trim());
    gl.compile_shader(&fs);

    let program = gl.create_program().ok_or("failed to create program")?;
    gl.attach_shader(&program, &vs);
    gl.attach_shader(&program, &fs);
    gl.link_program(&program);

    gl.detach_shader(&program, &vs);
    gl.detach_shader(&program, &fs);
    gl.delete_shader(Some(&vs));
    gl.delete_shader(Some(&fs));

    if !gl
        .get_program_parameter(&program, Gl::LINK_STATUS)
        .as_bool()
        .ok_or("failed to convert to bool")?
    {
        console_error!(
            "Link failed: {}",
            gl.get_program_info_log(&program)
                .ok_or("failed to get program info")?
        );
        console_error!(
            "vs info-log: {}",
            gl.get_shader_info_log(&vs)
                .ok_or("failed to get vertex shader info")?
        );
        console_error!(
            "fs info-log: {}",
            gl.get_shader_info_log(&fs)
                .ok_or("failed to get fragment shader info")?
        );
        return Err("failed to link");
    }

    return Ok(program);
}

fn create_buffer(gl: &Gl, positions: &[f32]) -> Result<(), &'static str> {
    let positions = positions
        .into_iter()
        .map(|&pos| JsValue::from_f64(pos as f64))
        .collect::<Array>();
    let positions = JsValue::from(positions);
    let positions = Float32Array::new(&positions);

    let buffer_id = gl.create_buffer().ok_or("failed to create buffer")?;
    gl.bind_buffer(Gl::ARRAY_BUFFER, Some(&buffer_id));
    gl.buffer_data_with_opt_array_buffer(
        Gl::ARRAY_BUFFER,
        Some(&positions.buffer()),
        Gl::STATIC_DRAW,
    );

    Ok(())
}

fn bind_buffer(gl: &Gl, program: &WebGlProgram, name: &str) {
    let a_position = gl.get_attrib_location(program, name) as u32;
    gl.vertex_attrib_pointer_with_i32(a_position, 2, Gl::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(a_position);
}

pub fn try_render(gl: Gl, width: i32, height: i32) -> Result<(), &'static str> {
    let points = [
        -0.5, -0.5, -0.5, 0.5, 0.5, 0.5, //
        -0.5, -0.5, 0.5, 0.5, 0.5, -0.5, //
    ];

    let program = init_shader(&gl, VERTEX_SHADER, FRAGMENT_SHADER)?;
    gl.use_program(Some(&program));
    create_buffer(&gl, &points)?;
    bind_buffer(&gl, &program, "aPosition");

    gl.viewport(0, 0, width, height);

    gl.clear_color(0.0, 0.0, 0.0, 1.0);
    gl.clear(Gl::COLOR_BUFFER_BIT);

    gl.draw_arrays(Gl::TRIANGLES, 0, points.len() as i32 / 2);

    Ok(())
}

#[wasm_bindgen]
pub fn render(gl: Gl, width: i32, height: i32) {
    if let Err(error) = try_render(gl, width, height) {
        console_error!("{error}");
    }
}
