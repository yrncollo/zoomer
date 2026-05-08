// Main application loop

use anyhow::Result;
use gl::types::*;
use std::ffi::CString;
use x11::xlib::{
    self, ButtonPress, ButtonRelease, Button1, Button4, Button5, ControlMask,
    Expose, KeyPress, MotionNotify, RevertToParent, XDefaultRootWindow,
    XEvent, XGetInputFocus, XGetWindowAttributes, XKeyEvent, XLookupKeysym,
    XNextEvent, XPending, XQueryPointer, XSetInputFocus, XSync,
};

use crate::capture::Screenshot;
use crate::config::Config;
use crate::input::{Camera, Mouse};
use crate::math::Vec2;
use crate::renderer::{uniform_loc, upload_texture, link_program, create_quad};
use crate::window::XWindow;
