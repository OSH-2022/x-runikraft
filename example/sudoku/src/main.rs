/*
    一个数独程序

    库依赖：
    [dependencies]
    rand = { version = "0.8.5", features = ["small_rng"] }

*/
// extern crate rand;
// use rand::Rng;
#![no_std]
#![no_main]
extern crate rkboot;

// use rkplat::time::wall_clock;
use rkgpu::*;
use rkswrand::fast_random;
use rkinput::*;
use rksched::*;
use core::time::Duration;
use core::ptr::null_mut;






pub struct Sudoku {
    // 当前数独信息(玩家显示)
    map: [[usize; 9]; 9],
    // 标记是不是原来的数字。 1 为 原始数字
    tag: [[usize; 9]; 9]

}

impl Sudoku {
    // 打印当前数独信息
    pub unsafe fn map_print(&self) {
        for i in 0..9 {
            for j in 0..9 {
                // show_sudoku_number(pos_x: u8, pos_y: u8, number: u8);
                if tag.map[i][j] == 0 {
                    show_sudoku_number(i as u8, j as u8, self.map[i][j] as u8, GRAY);
                    continue;
                } else {
                    show_sudoku_number(i as u8, j as u8, self.map[i][j] as u8, BLACK);
                // print!("{} ", self.map[i][j]);
                }
            }
            // println!("");
        }
    }
}

// 数独零初始化
// 生成一个 9x9 零矩阵
pub fn sudoku_init_zero () -> Sudoku {

    let map_allzero = [[0; 9]; 9];
    let sudoku = Sudoku {
        map: map_allzero,
        tag: map_allzero
    };
    sudoku
}

// 数独行随机填充：
// 将 @map 的第 @row 行用 1-9 随机序列填充
// use rand::Rng;

pub fn row_random(map: &mut [[usize; 9]; 9], row: usize) {
    let mut rowtable = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    for i in 0..9 {
        // let mut rng = rand::thread_rng();
        // let index = rng.gen_range(0..9);
        let index: usize = fast_random::<usize>() % 9;
        let temp = rowtable[i];
        rowtable[i] = rowtable[index];
        rowtable[index] = temp;
    }

    for i in 0..9 {
        map[row][i] = rowtable[i];
    }
    
}

// 数独复制
// map1 <- map2
pub fn sudoku_copy(map1: & mut [[usize; 9]; 9], map2: &[[usize; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            map1[i][j] = map2[i][j];
        }
    }
}

pub fn sudoku_setzero(map: & mut [[usize; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            map[i][j] = 0;
        }
    }
}

/*  寻找下一个可填入的空位
 *  从第 @row 行开始寻找
 *  如果找到则将结果填入 @nextrow, @nextcol 中，并返回 true
 *  否则返回 false
 */
pub fn findnext_empty(map: &[[usize; 9]; 9], row: usize, nextrow: & mut usize, nextcol: & mut usize)-> bool {
    for i in row..9 {
        for j in 0..9 {
            if map[i][j] == 0 {
                *nextrow = i as usize;
                *nextcol = j as usize;
                return true;
            }
        }
    }
    return false;
}

/*
 * 检查该位置填入的数字是否合法，若合法则返回 true
 * 检测内容：在 (@x, @y) 位置填入 @number
 * @if_checkself 为 true 时，如果目标位置存在数字则视作不合法
 */
pub fn if_fit_check(map: &[[usize; 9]; 9], x: usize, y: usize, number: usize, if_checkself: bool) -> bool{

    // 检查目标位置
    if if_checkself && map[x][y] != 0 {
        return false;
    }

    // 检查行
    for i in 0..9 {
        if i == x {
            continue;
        }
        if map[i][y] == number {
            return false;
        } 
    }

    // 检查列
    for i in 0..9 {
        if i == y {
            continue;
        }
        if map[x][i] == number {
            return false;
        } 
    }

    // 检查九宫
    for i in (x/3)*3 .. (x/3)*3 + 3 {
        for j in (y/3)*3 .. (y/3)*3 + 3 {
            if i == x && j == y {
                continue;
            }
            if map[i][j] == number {
                return false;
            }
        }
    }
    return true;
}


/* 求解函数（递归实现）
 * 从 (row, col) 开始求解当前 @map 中的数独，将结果存储到 @map 中
 * 返回 true 为有解，返回 false 为无解
 */

pub fn sudoku_solve(map: & mut [[usize; 9]; 9], row: usize, col: usize) -> bool{

    let mut nextrow: usize = 0;
    let mut nextcol: usize = 0;

    let mut number = 0;
    loop {
        number += 1;
        if number >= 10 {
            break;
        }

        if !(if_fit_check(map, row, col, number, false)) {
            continue;
        }

        map[row][col] = number;

        if !(findnext_empty(map, row, & mut nextrow, & mut nextcol)) {
            // 没有空位了，数独求解完成 
            return true;
        }

        if !(sudoku_solve(map, nextrow, nextcol)) {
            map[row][col] = 0;
            continue;
        }
        else {
            return true;
        }
    }
    false
}

/* 
    挖洞函数： 对于生成的数独进行随机挖空
    以 @map 为模板，将挖空的结果写入 @map
    @num 为留下的非空格数字数目，最低为 10
*/
pub fn hole_dig(map:& mut [[usize; 9]; 9], num: usize, tag: &mut [[usize; 9]; 9]) {
    let mut hole_map = [[0; 9]; 9];

    let mut number_num = num % 81;
    if number_num < 10 {
        number_num = 10;
    }
    // let mut rng = rand::thread_rng();
    let mut i = 0;

    while i < number_num {
        // let mut index = rng.gen_range(0..81);
        // let time = wall_clock();
        // let mut index = (time.as_nanos() % 81) as usize;
        let mut index:usize = fast_random::<usize>() % 81;
        loop {
            if index >= 81 {
                index %= 81;
            }
            if hole_map[index / 9][index % 9] != 0 {
                index += 1;
                continue
            }
            hole_map[index / 9][index % 9] = map[index / 9][index % 9];
            tag[index / 9][index % 9] = 1;
            break;
        }
        i += 1;
    }

    sudoku_copy(map, & hole_map);

}

/*
    添加数字的函数
    向 @map 中 (@row, @col) 的位置上写入 @num。 [0~8]
    如果该位置原先有数字，则写入失败
    @ifcheck 为 true 时，如果填入的数字破坏了数独规则也写入失败
*/
pub fn add_num(map: &mut [[usize; 9]; 9], row:usize, col:usize, num: usize, ifcheck:bool) -> bool {
    if row > 8 || col > 8 {
        // 访问越界
        return false;
    }
    if map[row][col] != 0 {
        return false;
    }
    if num > 9 || num <= 0{
        return false;
    }
    if ifcheck && !(if_fit_check(map, row, col, num, false)) {
        return false;
    }

    map[row][col] = num;
    return true;
}

/*
    删除数字的函数
    将 @map 中 (@row, @col) 的位置上数字删除（写入 0）
    如果该位置原先就是 0，则删除失败
*/
pub fn del_num(map: &mut [[usize; 9]; 9], tag: &[[usize; 9]; 9], row:usize, col:usize) -> bool {
    if row > 8 || col > 8  {
        // 访问越界
        return false;
    }
    if map[row][col] == 0 {
        return false;
    }

    if tag[row][col] == 1 {
        return false;
    }

    map[row][col] = 0;
    return true;
}


/*
    提示函数：该函数将根据当前 @map 的内容，求解数独，并将解填入第一个空格。
 */
pub fn hint(map: &mut [[usize; 9]; 9]) -> bool{
    let mut map_allzero:[[usize; 9]; 9] = [[0; 9]; 9];
    sudoku_copy(& mut map_allzero, map);

    let mut nextrow: usize = 0;
    let mut nextcol: usize = 0;

    if !(findnext_empty(map, 0, & mut nextrow, & mut nextcol)) {
        // 没有空位了 
        return false;
    }
    sudoku_solve(& mut map_allzero, nextrow, nextcol);

    add_num(map, nextrow, nextcol, map_allzero[nextrow][nextcol], false);

    unsafe{show_sudoku_number(nextrow, nextcol, map_allzero[nextrow][nextcol], GOLD);}

    return true;
}


#[no_mangle]
fn main() {
    unsafe {
        rksched::sched::create_thread("", rkalloc::get_default().unwrap(),
                                      rksched::thread::ThreadAttr::default(), rksched::thread::ThreadLimit::default(),
                                      input_tracer, null_mut());

        let mut sudoku = sudoku_init_zero();
        let mut map_old: [[usize; 9]; 9] = [[0; 9]; 9];

        init();
        printg("Use W, A, S, and D to move selecting rectangle.\nUse up, left, down, and right to move cursor.",0,700,BLACK,255,2);
        draw_sudoku_lattices(PURPLE, BLACK);
        screen_flush();
        row_random(&mut sudoku.map, 0);
        sudoku_solve(&mut sudoku.map, 1, 1);

        hole_dig(&mut sudoku.map, 15,&mut sudoku.tag);
        sudoku.map_print();

    sudoku_copy(& mut map_old, & sudoku.map);
    
    

    
    
    loop {
        rksched::this_thread::sleep_for(Duration::from_millis(1));

        if INPUT_NUMBER >= 1 && INPUT_NUMBER <= 9 && add_num(&mut sudoku.map, SELECT_X as usize / 75 , SELECT_Y as usize / 75, INPUT_NUMBER - 1, true) {
        //if add_num(&mut sudoku.map, 0 , 0, INPUT_NUMBER, true) {
            show_sudoku_number((SELECT_X / 75) as u8, (SELECT_Y / 75) as u8, (INPUT_NUMBER - 1) as u8, GRAY);
            //show_sudoku_number(0, 0, INPUT_NUMBER as u8, GRAY);
        }

        if INPUT_NUMBER == 0 && del_num(&mut sudoku.map, &sudoku.tag, SELECT_X as usize / 75, SELECT_Y as usize / 75) {
            show_sudoku_number((SELECT_X / 75) as u8, (SELECT_Y / 75) as u8, 0, GRAY);
            //show_sudoku_number(SELECT_X as u8 / 75, SELECT_Y as u8 / 75, 255, BLACK);
        }

        if INPUT_NUMBER == KEY_H as usize {
            hint(&mut &mut sudoku.map);
        }

        if INPUT_NUMBER == KEY_O as usize {
            sudoku_solve(& mut sudoku.map, 0, 0);
            sudoku.map_print();
        }

    }
    // sudoku_solve(& mut sudoku.map, & mut sudoku.answer, 0, 0);
    // unsafe { sudoku.map_print(); }
    }
}

use rkplat::drivers::virtio::GPU_DEIVCE;
use rkgpu::{draw_font,DIRECTION,draw_line};
unsafe fn draw_sudoku_lattices(color0:Color,color1:Color) -> u8 {
    let (width, height) = GPU_DEIVCE.as_mut().unwrap().resolution();
    if width >= 750 && height >= 750 {
        for x in 0..10 {
            if x % 3 == 0 {
                draw_line(DIRECTION::Vertical, x * 75, 0, 675, color0, 255, 4);
            } else {
                draw_line(DIRECTION::Vertical, x * 75, 0, 675, color1, 255, 1);
            }
        }
        for y in 0..10 {
            if y % 3 == 0 {
                draw_line(DIRECTION::Horizontal, 0, y * 75, 675, color0, 255, 4);
            } else {
                draw_line(DIRECTION::Horizontal, 0, y * 75, 675, color1, 255, 1);
            }
        }
        1
    } else { 0 }
}

unsafe fn show_sudoku_number(pos_x: u8, pos_y: u8, number: u8,color:Color) -> u8 {
    if pos_x <= 8 && pos_y <= 8 {
        let start_x: u32 = 75 * pos_x as u32 + 20;
        let start_y: u32 = 75 * pos_y as u32 + 6;
        if number == 0 {
            draw_font(start_x, start_y, CYAN, 255,' ', 4);
        } else {
            draw_font(start_x, start_y, color, 255,(number + 48).into(), 4);
        }
        screen_flush();
        0
    } else { 1 }
}

