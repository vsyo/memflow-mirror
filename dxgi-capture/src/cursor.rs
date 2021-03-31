use std::mem::size_of;
use std::ptr;

use winapi::shared::windef::POINT;
use winapi::um::winuser::{GetCursorInfo, CURSORINFO, CURSOR_SHOWING};

#[repr(C)]
pub struct Cursor {
    is_visible: bool,
    cursor_id: u32, // TODO:
    x: i32,
    y: i32,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            is_visible: false,
            cursor_id: 0,
            x: 0,
            y: 0,
        }
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_state() -> Result<Cursor, &'static str> {
    let mut ci = CURSORINFO {
        cbSize: size_of::<CURSORINFO>() as u32,
        flags: 0,
        hCursor: ptr::null_mut(),
        ptScreenPos: POINT { x: 0, y: 0 },
    };
    let result = unsafe { GetCursorInfo(&mut ci) };
    if result != 0 {
        Ok(Cursor {
            is_visible: ci.flags == CURSOR_SHOWING,
            cursor_id: ci.hCursor as u32,
            x: ci.ptScreenPos.x,
            y: ci.ptScreenPos.y,
        })
    } else {
        Err("unable to get cursor info")
    }
}
