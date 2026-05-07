// X11 screen capture
// Ported from boomer's screenshot.nim

use std::ffi::c_uint;
use x11::xlib::{
    Display, Window, XDestroyImage, XGetImage, XGetWindowAttributes, XImage,
    ZPixmap,
};
