use core::cmp::{max, min};
use rkplat::drivers::virtio::GPU_DEIVCE;
use crate::*;

pub fn update_cursor(start_x: u32, start_y: u32, is_init: bool) {
    unsafe {
        let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
        if !is_init {
            for i in 1..(FB_CURSOR[0] + 1) as usize {
                FB[FB_CURSOR[5 * i - 4] as usize + 2] = FB_CURSOR[5 * i - 4 + 1] as u8;
                FB[FB_CURSOR[5 * i - 4] as usize + 1] = FB_CURSOR[5 * i - 4 + 2] as u8;
                FB[FB_CURSOR[5 * i - 4] as usize + 0] = FB_CURSOR[5 * i - 4 + 3] as u8;
                FB[FB_CURSOR[5 * i - 4] as usize + 3] = FB_CURSOR[5 * i - 4 + 4] as u8;
            }
        }
        let mut idx_cursor = 1;
        for y in max(start_y, 1) - 1..min(start_y + 1, height - 1) + 1 {
            for x in max(start_x, 10) - 10..min(start_x + 10, width - 1) + 1 {
                let idx = (y * width + x) * 4;
                FB_CURSOR[idx_cursor] = idx;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 2] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 1] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 0] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 3] as u32;
                idx_cursor += 1;
            }
        }
        for x in max(start_x, 1) - 1..min(start_x + 1, width - 1) + 1 {
            for y in max(start_y, 10) - 10..min(start_y + 10, height - 1) + 1 {
                let idx = (y * width + x) * 4;
                FB_CURSOR[idx_cursor] = idx;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 2] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 1] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 0] as u32;
                idx_cursor += 1;
                FB_CURSOR[idx_cursor] = FB[idx as usize + 3] as u32;
                idx_cursor += 1;
            }
        }
        FB_CURSOR[0] = (idx_cursor / 5) as u32;
        draw_line(Horizontal, (max(10, start_x) - 10) as u32, (max(1, start_y) - 1) as u32, min(21, start_x + 10), BLACK, 255, 3);
        draw_line(Vertical, (max(1, start_x) - 1) as u32, (max(10, start_y) - 10) as u32, min(21, start_y + 10), BLACK, 255, 3);
        GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
    }
}

pub unsafe fn draw_select(start_x: u32, start_y: u32, color: Color) {
    draw_line(Horizontal, start_x + 5, start_y + 5, 65, color, 255, 1);
    draw_line(Horizontal, start_x + 5, start_y + 70, 65, color, 255, 1);
    draw_line(Vertical, start_x + 5, start_y + 5, 65, color, 255, 1);
    draw_line(Vertical, start_x + 70, start_y + 5, 65, color, 255, 1);
    GPU_DEIVCE.as_mut().unwrap().flush().expect("failed to flush");
}
