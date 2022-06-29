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

use rkgpu::show_sudoku_number;
use rkplat::time::wall_clock;

pub struct Sudoku {
    // 当前数独信息(玩家显示)
    map: [[usize; 9]; 9],
    // 当前数独的一个解
    answer: [[usize; 9]; 9]

}

impl Sudoku {
    // 打印当前数独信息
    pub unsafe fn map_print(&self) {
        for i in 0..9 {
            for j in 0..9 {
                // show_sudoku_number(pos_x: u8, pos_y: u8, number: u8);
                show_sudoku_number(i as u8, j as u8, self.map[i][j] as u8);

                // print!("{} ", self.map[i][j]);
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
        answer: map_allzero
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
        let time = wall_clock();
        let index = (time.as_nanos() % 9) as usize;
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
 *  如果找到则将结果填入 @nextrow, @nextcol 中，并返回 1
 *  否则返回 0
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
 * 检查该位置填入的数字是否合法，若合法则返回 rrue
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


// 求解函数（递归实现）
// 从 (row, col) 开始求解当前 @map 中的数独，将结果存储到 @answer 中
// 返回 true 为有解，返回 false 为无解
pub fn sudoku_solve(map: & mut [[usize; 9]; 9], answer: &mut [[usize; 9]; 9], row: usize, col: usize) -> bool{

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

        if !(sudoku_solve(map, answer, nextrow, nextcol)) {
            map[row][col] = 0;
            continue;
        }
        else {
            return true;
        }
    }
    false
}

// 挖洞函数： 对于生成的数独进行随机挖空
// 以 @map 为模板，将挖空的结果写入 @map
// @num 为留下的非空格数字数目，最低为 10
pub fn hole_dig(map:& mut [[usize; 9]; 9], num: usize) {
    let mut hole_map = [[0; 9]; 9];

    let number_num = num % 81;
    // let mut rng = rand::thread_rng();
    let mut i = 0;

    while i < number_num {
    
        // let mut index = rng.gen_range(0..81);
        let time = wall_clock();
        let mut index = (time.as_nanos() % 81) as usize;
        loop {
            if index >= 81 {
                index %= 81;
            }
            if hole_map[index / 9][index % 9] != 0 {
                index += 1;
                continue
            }
            hole_map[index / 9][index % 9] = map[index / 9][index % 9];
            break;
        }
        i += 1;
    }

    sudoku_copy(map, & hole_map);

}

#[no_mangle]
fn main() {
    let mut sudoku = sudoku_init_zero();

    row_random(& mut sudoku.map, 0);
    sudoku_solve(& mut sudoku.map, & mut sudoku.answer, 1, 1);
    
    hole_dig(& mut sudoku.map, 10);
    unsafe { sudoku.map_print(); }

    // sudoku_solve(& mut sudoku.map, & mut sudoku.answer, 0, 0);
    // unsafe { sudoku.map_print(); }
}
