// SPDX-License-Identifier: BSD-3-Clause
// rkargparse/lib.rs

// Authors: 吴骏东 <1904346407@qq.com>

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

//! 命令行参数分割与整合
//!
//!  规范的命令行输入格式：
//! 
//! 自动删除 \n \t \r 等转转义字符
//! 允许多条管道进行命令分割
//! 重定向符号后要有文件名(

#![no_std]

pub const MAX_ARGS_NUM: usize = 20;    // 单条指令参数个数上限

/// 原始命令的相关信息
pub struct CmdInfo {
    pipenum: usize,           // 管道的数目

    // 储存每一条子指令 (管道数目 + 1)
    single_command: Vec<String>,

    read_redirect: Vec<usize>,    // 输入重定向
    read_filename: Vec<String>,   

    write_redirect: Vec<usize>,   // 输出重定向
    write_filename: Vec<String>,

    add_redirect: Vec<usize>,     // 追加重定向
    add_filename: Vec<String>,

}

/// 单条命令的相关信息
pub struct SinglecmdInfo {

    command: String,    // 命令头
    args: Vec<String>,  // 所有参数

    read_redirect: usize,    // 输入重定向
    read_filename: String,   

    write_redirect: usize,   // 输出重定向
    write_filename: String,

    add_redirect: usize,     // 追加重定向
    add_filename: String,
}

/// 针对完整解析内容结构体部分操作
impl CmdInfo {
    /// 信息输出
    pub fn infoprint(&self) {
        println!("The infomation for this command:");
        println!("Number of pipes: {}", self.pipenum);
        
        let mut i = 0;
        while i < self.pipenum + 1 {
            println!("Command [{}]: {}", i, self.single_command[i]);
            println!("      read_redirect: {}, write_redirect: {}, add_redirect: {}", self.read_redirect[i], self.write_redirect[i], self.add_redirect[i]);
            println!("      read_filename: {}, write_filename: {}, add_filename: {}", self.read_filename[i], self.write_filename[i], self.add_filename[i]);
            println!("\n");
            i += 1;
        }
    }

    /// 获取单独指令的相关信息
    /// index 范围： 0 ~ pipenum
    pub fn get_command_info(&self, index: usize) -> SinglecmdInfo {

        let mut single_cmdinfo = SinglecmdInfo{
            command: String::from("NONE"),
            args: Vec::with_capacity(MAX_ARGS_NUM),
            read_redirect: 0,
            read_filename: String::from("NONE"),
            write_redirect : 0,
            write_filename : String::from("NONE"), 
            add_redirect : 0,
            add_filename : String::from("NONE")
        };

        if index <= self.pipenum {
            let mut args_origin = get_args(&self.single_command[index]);
            single_cmdinfo.command = args_origin[0].clone();
            args_origin.remove(0);
            single_cmdinfo.args = args_origin;

            single_cmdinfo.read_redirect = self.read_redirect[index].clone();
            single_cmdinfo.read_filename = self.read_filename[index].clone();
            single_cmdinfo.write_redirect = self.write_redirect[index].clone();
            single_cmdinfo.write_filename = self.write_filename[index].clone();
            single_cmdinfo.add_redirect = self.add_redirect[index].clone();
            single_cmdinfo.add_filename = self.add_filename[index].clone();
        }

        single_cmdinfo.del_redirect();

        return single_cmdinfo;
    }
}

// 针对单条命令结构体部分操作
impl SinglecmdInfo {
    pub fn infoprint(&self) {
        println!("The infomation for this single command:");
        println!("      Command: {}", self.command);
        
        let mut i = 0;
        println!("The args for this single command:   ");
        if self.args.len() == 0 {
            println!("      None")
        }
        else {
            while i < self.args.len() {
                println!("      {} ", self.args[i]);
                i += 1;
            }
        }
            
        println!("The redirect infomation for this single command:   ");
        println!("      read_redirect: {}, write_redirect: {}, add_redirect: {}", self.read_redirect, self.write_redirect, self.add_redirect);
        println!("      read_filename: {}, write_filename: {}, add_filename: {}", self.read_filename, self.write_filename, self.add_filename);
    }

    /// 删除结构体中关于重定向的参数内容
    pub fn del_redirect(& mut self) {
        let mut i = 0;
        while i < self.args.len() {
            let ch = self.args[i].as_str();
            if ch == "<" || ch == ">" || ch == ">>" {
                self.args.remove(i);
                self.args.remove(i);
                continue;
            }
            i += 1;
        }
    }
}


/// 查找转义字符并将其删除
pub fn del_escape_ch(command: &String) -> String{
    let mut del_command = command.clone();
    del_command = del_command.replace("\n", "");
    del_command = del_command.replace("\r", "");
    del_command = del_command.replace("\t", "");
    del_command
}


/// 将可能包含的管道命令进行拆分，去除前后空格
pub fn get_single(command: &String) -> Vec<String>{ 
    let temp_vec: Vec<&str> = command.split("|").collect();
    let mut result: Vec<String> = Vec::with_capacity(10);
    let mut i = 0;
    
    while i < temp_vec.len() {
        result.push(temp_vec[i].trim().to_string());
        i += 1;
    }

    result
}

/// 检查是否有重定向,如果有则将目标文件返回
/// 目前仅支持单文件重定向（< > >> 各至多一个）
pub fn check_redirect(single_command: &String) -> (String, String, String) {
    let args: Vec<String> = get_args(&single_command);
    let mut i = 0;
    
    let mut read_filename: String = String::from("NONE");
    let mut write_filename: String = String::from("NONE");
    let mut add_filename: String = String::from("NONE");

    while i < args.len() - 1{
        if args[i] == "<" {
            read_filename = args[i + 1].clone();
        }
        if args[i] == ">" {
            write_filename = args[i + 1].clone();
        }
        if args[i] == ">>" {
            add_filename = args[i + 1].clone();
        }
        i += 1;
    }

    (read_filename, write_filename, add_filename)
}

/// 将每条命令中的参数进行分割, 返回一个 String 向量
pub fn get_args(single_command: &String) -> Vec<String>{
    let temp_str: Vec<&str> = single_command.split(" ").collect();
    let mut result: Vec<String> = Vec::with_capacity(10);
    let mut i = 0;

    while i < temp_str.len() {            
        result.push(temp_str[i].trim().to_string());
        i += 1;
    }
    result
}

/// 测试输出
pub fn print_single(single_cmd: &Vec<String>) {
    let mut i = 0;
    while i < single_cmd.len() {
        println!("The commond {}, {}", i, single_cmd[i]);
        i += 1;
    }
}


/// 根据输入的命令得到相关参数
/// 
/// 输入： @command 完整的一条命令
/// 
/// 输出： 一个 CmdInfo 结构体，包含了命令的完整解析
pub fn command_analysis (command: &String) -> CmdInfo {

    let single_command: Vec<String> = get_single(&del_escape_ch(&command));

    let mut read_redirect: Vec<usize> = Vec::with_capacity(10);
    let mut write_redirect: Vec<usize> = Vec::with_capacity(10);
    let mut add_redirect: Vec<usize> = Vec::with_capacity(10);
    let mut read_filename: Vec<String> = Vec::with_capacity(10);
    let mut write_filename: Vec<String> = Vec::with_capacity(10);
    let mut add_filename: Vec<String> = Vec::with_capacity(10);

    let mut i = 0;
    while i < single_command.len() {
        let (read_name, write_name, add_name) = check_redirect(&single_command[i]);

        if read_name == "NONE" {
            read_redirect.push(0);
        }
        else {
            read_redirect.push(1);
        }    
        if write_name == "NONE" {
            write_redirect.push(0);
        }
        else {
            write_redirect.push(1);
        }
        if add_name == "NONE" {
            add_redirect.push(0);
        }
        else {
            add_redirect.push(1);
        }
            
        read_filename.push(read_name);
        write_filename.push(write_name);
        add_filename.push(add_name);
        i += 1;
    }

    CmdInfo {
        pipenum : single_command.len() - 1,

        single_command : single_command,

        read_redirect : read_redirect,
        read_filename : read_filename,

        write_redirect : write_redirect,
        write_filename : write_filename, 

        add_redirect : add_redirect,
        add_filename : add_filename
    }
}

// 测试程序
/*
fn main() {
    let command: String= String::from(" ls > \n 1.txt| pwd|echo \"$USER\"|- 33 | cat >> 1.txt < 2.txt > 3.txt");
    // command: 存储命令的 String

    let info = command_analysis(&command);
    // info: 存储完整解析结果的结构体
    info.infoprint();

    let single_info = info.get_command_info(4); // 4 is index
    // single_info: 存储单条指令解析结果的结构体
    single_info.infoprint();

}
*/
