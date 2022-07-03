// SPDX-License-Identifier: BSD-3-Clause
// sudoku/main.rs

// Authors:  吴骏东 <1904346407@qq.com>
// Authors:  郭耸霄 <logname@mail.ustc.edu.cn>

// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.

#![no_std]
#![no_main]
#![allow(unused)]
#![allow(non_upper_case_globals)]
extern crate rkboot;
extern crate alloc;

use rkplat::time::wall_clock;
use rkgpu::*;
use rkswrand::fast_random;

use rksched::*;
use core::time::Duration;
use core::ptr::null_mut;
use rklock::*;
use rktimeconv::TimePoint;

pub mod key;
pub mod input;
pub mod cursor;
pub mod output;

pub use key::*;
pub use input::*;
pub use cursor::*;
pub use output::*;

static mutex: Semaphore = Semaphore::new(0);

pub struct Sudoku {
    // 当前数独信息(玩家显示)
    map: [[usize; 9]; 9],
    // 标记是不是原来的数字。 1 为 原始数字
    tag: [[usize; 9]; 9],
}


// 数独零初始化
// 生成一个 9x9 零矩阵
pub fn sudoku_init_zero() -> Sudoku {
    let map_allzero = [[0; 9]; 9];
    let sudoku = Sudoku {
        map: map_allzero,
        tag: map_allzero,
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
pub fn sudoku_copy(map1: &mut [[usize; 9]; 9], map2: &[[usize; 9]; 9]) {
    for i in 0..9 {
        for j in 0..9 {
            map1[i][j] = map2[i][j];
        }
    }
}

pub fn sudoku_setzero(map: &mut [[usize; 9]; 9]) {
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
pub fn findnext_empty(map: &[[usize; 9]; 9], row: usize, nextrow: &mut usize, nextcol: &mut usize) -> bool {
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
pub fn if_fit_check(map: &[[usize; 9]; 9], x: usize, y: usize, number: usize, if_checkself: bool) -> bool {

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
    for i in (x / 3) * 3..(x / 3) * 3 + 3 {
        for j in (y / 3) * 3..(y / 3) * 3 + 3 {
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

pub fn sudoku_solve(map: &mut [[usize; 9]; 9], row: usize, col: usize, depth: usize) -> bool {
    let mut nextrow: usize = 0;
    let mut nextcol: usize = 0;
    if depth > 81 {
        return false;
    }

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

        if !(findnext_empty(map, row, &mut nextrow, &mut nextcol)) {
            // 没有空位了，数独求解完成 
            return true;
        }

        if !(sudoku_solve(map, nextrow, nextcol, depth + 1)) {
            map[row][col] = 0;
            continue;
        } else {
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
pub fn hole_dig(map: &mut [[usize; 9]; 9], num: usize, tag: &mut [[usize; 9]; 9]) {
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
        let mut index: usize = fast_random::<usize>() % 81;
        loop {
            if index >= 81 {
                index %= 81;
            }
            if hole_map[index / 9][index % 9] != 0 {
                index += 1;
                continue;
            }
            hole_map[index / 9][index % 9] = map[index / 9][index % 9];
            tag[index / 9][index % 9] = 1;
            break;
        }
        i += 1;
    }

    sudoku_copy(map, &hole_map);
}

/*
    添加数字的函数
    向 @map 中 (@row, @col) 的位置上写入 @num。 [0~8]
    如果该位置原先有数字，则写入失败
    @ifcheck 为 true 时，如果填入的数字破坏了数独规则也写入失败
*/
pub fn add_num(map: &mut [[usize; 9]; 9], row: usize, col: usize, num: usize, ifcheck: bool) -> bool {
    if row > 8 || col > 8 {
        // 访问越界
        return false;
    }
    if map[row][col] != 0 {
        unsafe { mutex.signal(); }
        return false;
    }
    if num > 9 || num <= 0 {
        unsafe { mutex.signal(); }
        return false;
    }
    if ifcheck && !(if_fit_check(map, row, col, num, false)) {
        unsafe { mutex.signal(); }
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
pub fn del_num(map: &mut [[usize; 9]; 9], tag: &[[usize; 9]; 9], row: usize, col: usize) -> bool {
    if row > 8 || col > 8 {
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
pub fn hint(map: &mut [[usize; 9]; 9]) -> bool {
    let mut map_allzero: [[usize; 9]; 9] = [[0; 9]; 9];
    sudoku_copy(&mut map_allzero, map);

    let mut nextrow: usize = 0;
    let mut nextcol: usize = 0;
    if !findnext_empty(map, 0, &mut nextrow, &mut nextcol) {
        return false;
    }
    if !sudoku_solve(&mut map_allzero, nextrow, nextcol, 0) {
        return false;
    }

    let mut index: usize = fast_random::<usize>() % 81;
    let mut times = 0;
    loop {
        times += 1;
        if times > 81 {
            return false;
        }
        if index >= 81{
            index %= 81;
        }
        if map[index / 9][index % 9] != 0 {
            index += 1;
            continue;
        }
        nextrow = index / 9;
        nextcol = index % 9;
        break;
    }

    // if !(findnext_empty(map, 0, &mut nextrow, &mut nextcol)) {
    //     // 没有空位了
    //     return false;
    // }

    if !add_num(map, nextrow, nextcol, map_allzero[nextrow][nextcol], false) {
        return false;
    }

    unsafe { show_sudoku_number(nextrow as u8, nextcol as u8, map_allzero[nextrow][nextcol] as u8, TAN); }

    return true;
}

pub fn error_hinter(_null: *mut u8) {
    unsafe {
        loop {
            mutex.wait();
            printg("You can't write this number HERE!", 700, 500, RED, 255, 2);
            rksched::this_thread::sleep_for(Duration::from_secs(1));
            printg("                                 ", 700, 500, RED, 255, 2);
        }
    }
}

fn init(sudoku: &mut Sudoku) {
    unsafe {
        sched::create_thread("", rkalloc::get_default().unwrap(),
                             thread::ThreadAttr::default(), rksched::thread::ThreadLimit::default(),
                             input_tracer, null_mut()).unwrap();

        sched::create_thread("", rkalloc::get_default().unwrap(),
                             thread::ThreadAttr::default(), rksched::thread::ThreadLimit::default(),
                             error_hinter, null_mut()).unwrap();

        sched::create_thread("", rkalloc::get_default().unwrap(),
                             thread::ThreadAttr::default(), rksched::thread::ThreadLimit::default(),
                             show_time, null_mut()).unwrap();
        rkgpu::init();
        printg("Hello, world!\nHello, OSH-2022!\nHello, Runikraft!\n", 700, 10, RED, 255, 4);
        printg("Use W, A, S, and D to move selecting rectangle.\nUse up, left, down, and right to move cursor.\nUse H for hint, use O for solution.", 0, 700, BLACK, 255, 2);
        update_cursor(900, 400, true);
        draw_select(0, 0, RED);
        draw_sudoku_lattices(PURPLE, BLACK);
        screen_flush();
        row_random(&mut sudoku.map, 0);
        sudoku_solve(&mut sudoku.map, 1, 1, 0);
        hole_dig(&mut sudoku.map, 15, &mut sudoku.tag);
        sudoku.map_print();
    }
}

#[no_mangle]
fn main() {
    let mut sudoku: Sudoku = sudoku_init_zero();
    init(&mut sudoku);
    unsafe {
        loop {
            this_thread::sleep_for(Duration::from_millis(1));

            if INPUT_NUMBER >= 1 && INPUT_NUMBER <= 9 && add_num(&mut sudoku.map, SELECT_X as usize / 75, SELECT_Y as usize / 75, INPUT_NUMBER, true) {
                //if add_num(&mut sudoku.map, 0 , 0, INPUT_NUMBER, true) {
                show_sudoku_number((SELECT_X / 75) as u8, (SELECT_Y / 75) as u8, (INPUT_NUMBER) as u8, GRAY);
                //show_sudoku_number(0, 0, INPUT_NUMBER as u8, GRAY);
            }

            if INPUT_NUMBER == 0 && del_num(&mut sudoku.map, &sudoku.tag, SELECT_X as usize / 75, SELECT_Y as usize / 75) {
                show_sudoku_number((SELECT_X / 75) as u8, (SELECT_Y / 75) as u8, 0, GRAY);
                //show_sudoku_number(SELECT_X as u8 / 75, SELECT_Y as u8 / 75, 255, BLACK);
            }

            if INPUT_NUMBER == 35 {
                hint(&mut sudoku.map);
            }

            if INPUT_NUMBER == 24 {
                if !sudoku_solve(&mut sudoku.map, 0, 0, 0) {
                    INPUT_NUMBER = 200;
                    continue;
                }
                sudoku.map_print();
            }
            INPUT_NUMBER = 200;
        }
    }
}
