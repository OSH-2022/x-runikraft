use std::cmp::min;
use std::ptr::null;
use rkplat::drivers::virtio::GPU_DEIVCE;
use crate::DIRECTION::{Horizontal, Vertical};

pub static mut FB: &mut [u8] = null() as &mut [u8];

pub unsafe fn init() {
    FB = GPU_DEIVCE.unwrap().setup_framebuffer().expect("failed to get FB");
    let (width, height) = GPU_DEIVCE.unwrap().resolution();
    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;
            FB[idx] = x as u8;
            FB[idx + 1] = y as u8;
            FB[idx + 2] = (x + y) as u8;
        }
    }
    //TODO A start photo posed by LJW
    gpu.flush().expect("failed to flush");
}

enum DIRECTION {
    Horizontal,
    Vertical,
}

pub unsafe fn draw_line(direction: DIRECTION, start_x: u32, start_y: u32, length: u32, color: u8) {
    let (width, height) = GPU_DEIVCE.unwrap().resolution();
    match direction {
        Horizontal => {
            for x in 0..min(length, width - start_x) {
                let idx = (start_y * width + x) ;
                FB[idx] = color as u8;
            }
        }
        Vertical => {
            for y in 0..min(length, height - start_y) {
                let idx = (y * width + start_x) ;
                FB[idx] = color as u8;
            }
        }
    }
    gpu.flush().expect("failed to flush");
}

pub unsafe fn draw_font(start_x:u32,start_y:u32,color:u8,font:u8){
    //TODO
}
