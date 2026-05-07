// OpenGL shader compilation, VAO/VBO/EBO and texture management
// Mirrors boomer's shader helpers and draw proc

use anyhow::anyhow;
use gl::types::*;
use std::ffi::CString;
