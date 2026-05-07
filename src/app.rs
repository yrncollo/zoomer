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
