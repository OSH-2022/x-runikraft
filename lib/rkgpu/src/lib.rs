#![no_std]

use core::cmp::min;
use rkplat::drivers::virtio::GPU_DEIVCE;
use crate::DIRECTION::{Horizontal, Vertical};

static mut _EMPTY: [u8;0] = [0;0];

pub static mut FB: &mut [u8] = unsafe {&mut _EMPTY};
pub unsafe fn init() {
    FB = GPU_DEIVCE.as_mut().unwrap().setup_framebuffer().expect("failed to get FB");
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    for y in 0..height as usize {
        for x in 0..width as usize {
            let idx = (y * width as usize + x) * 4;
            FB[idx] = x as u8;
            FB[idx + 1] = y as u8;
            FB[idx + 2] = (x + y) as u8;
        }
    }
    //TODO A start photo posed by LJW
    GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
}

pub enum DIRECTION {
    Horizontal,
    Vertical,
}

pub unsafe fn draw_line(direction: DIRECTION, start_x: u32, start_y: u32, length: u32, color: u8) {
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    match direction {
        Horizontal => {
            for x in 0..min(length, width - start_x) {
                let idx = start_y * width + x;
                FB[idx as usize] = color as u8;
            }
        }
        Vertical => {
            for y in 0..min(length, height - start_y) {
                let idx = y * width + start_x;
                FB[idx as usize] = color as u8;
            }
        }
    }
    GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
}

pub unsafe fn draw_font(start_x:u32,start_y:u32,color:u8,font:u8){
    //TODO
}
