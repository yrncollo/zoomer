// X11 window + GLX context creation

// Force the linker to include libGL and libGLX
#[link(name = "GL")]
#[link(name = "GLX")]
extern "C" {}
