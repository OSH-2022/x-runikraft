// 命令行参数分割与整合



// 命令的相关信息
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

impl CmdInfo {
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
}


// 将可能包含的管道命令进行拆分，去除前后空格
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

// 检查是否有重定向,如果有则将目标文件返回
// 目前仅支持单文件重定向（< > >> 各至多一个）
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

// 将每条命令中的参数进行分割, 返回一个 String 向量
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

// 测试输出
pub fn print_single(single_cmd: &Vec<String>) {
    let mut i = 0;
    while i < single_cmd.len() {
        println!("The commond {}, {}", i, single_cmd[i]);
        i += 1;
    }
}

// 根据输入的命令得到相关参数
pub fn command_analysis (command: &String) -> CmdInfo {

    let single_command: Vec<String> = get_single(&command);

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