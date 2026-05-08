// OpenGL shader compilation, VAO/VBO/EBO and texture management
// Mirrors boomer's shader helpers and draw proc

use anyhow::anyhow;
use gl::types::*;
use std::ffi::CString;

pub unsafe fn compile_shader(src: &str, kind: GLenum) -> anyhow::Result<GLuint> {
    let id = gl::CreateShader(kind);
    let c_src = CString::new(src).unwrap();
    let ptr = c_src.as_ptr();
    gl::ShaderSource(id, 1, &ptr, std::ptr::null());
    gl::CompileShader(id);

    let mut ok: GLint = 0;
    gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut ok);
    if ok == 0 {
        let mut len: GLint = 0;
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        let mut buf = vec![0u8; len as usize];
        gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), buf.as_mut_ptr() as *mut i8);
        gl::DeleteShader(id);
        let msg = String::from_utf8_lossy(&buf).into_owned();
        return Err(anyhow!("Shader compile error:\n{}", msg));
    }
    Ok(id)
}
