// X11 window + GLX context creation

// Force the linker to include libGL and libGLX
#[link(name = "GL")]
#[link(name = "GLX")]
extern "C" {}

use anyhow::anyhow;
use std::ffi::CString;
use std::ptr;
use x11::glx::{
    glXChooseVisual, glXCreateContext, glXMakeCurrent, glXQueryVersion,
    glXSwapBuffers, GLX_DEPTH_SIZE, GLX_DOUBLEBUFFER, GLX_RGBA,
};
