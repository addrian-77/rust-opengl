#![allow(unused_imports)]
#![allow(missing_docs)]

use beryllium::{
    Sdl,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile, GlWindow},
};
use core::convert::{TryFrom, TryInto};
use gl33::{global_loader::*, *};

pub fn clear_color(gl: &GlFns, r: f32, g: f32, b: f32, a: f32) {
    unsafe { gl.ClearColor(r, g, b, a) }
}

pub fn create_window(sdl: &Sdl, window_title: &str, width: i32, height: i32) -> GlWindow {
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_minor_version(3).unwrap();
    sdl.set_gl_profile(GlProfile::Core).unwrap();
    let mut flags = GlContextFlags::default();
    flags |= GlContextFlags::FORWARD_COMPATIBLE;
    flags |= GlContextFlags::DEBUG;
    sdl.set_gl_context_flags(flags).unwrap();

    let win = sdl
        .create_gl_window(CreateWinArgs {
            title: window_title,
            width,
            height,
            resizable: true,
            ..Default::default()
        })
        .expect("couldn't make a window and context");
    win
}
