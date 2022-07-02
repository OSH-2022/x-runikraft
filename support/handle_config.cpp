#include <iostream>
#include <fstream>
#include <string>

const std::string const_prefix = "pub const ";
const std::string mod_prefix = "pub mod ";
const std::string mod_suffix = "{\n\t";
const std::string mod_end = "}\n";
const std::string empty;

std::string features1;
std::string features2;

std::string lib_name;
unsigned int count = 0;

std::string replace_content(std::string content, unsigned int flag);

int main(int argc, char* argv[]) {
    if(argc < 3) {
        std::cerr << "Too few arguments!\n";
    }
    const std::string SRC_ROOT_DIR = argv[1];
    const std::string BUILD_DIR = argv[2];

    std::string output_filename = BUILD_DIR + "/config.rs";
    std::ofstream OutputFile(output_filename);

    std::cout << output_filename << "\n";

    std::string message;
    message += "// RUNIKRAFT_CONFIG_FILE: " + BUILD_DIR + "/config.rs\n// Automatically generated file. DO NOT EDIT.\n\n\n";
    OutputFile << message;

    std::string input_filename = SRC_ROOT_DIR + "/.config";
    std::ifstream InputFile(input_filename);
    
    std::string line;
    while(std::getline(InputFile, line)) {
        if(line.length() > 3 && line.substr(0, 4) == "# rk") {
            std::string subline = line.substr(2);
            lib_name = subline.substr(0, std::min(subline.find(" "), subline.length()));
        } else if(line.length() > 2 && line[0] != '#') {
            if(line.find("CPU_TIME") != std::string::npos)
                line = replace_content(line, 0);
            else if(line.find("=y") != std::string::npos)
                line = replace_content(line, 1);
            else
                line = replace_content(line, 2);
            switch (count) {
                case 0: {
                    message = const_prefix + line + "\n";
                    OutputFile << message;
                    ++count;
                    break;
                }
                case 1: {
                    message = mod_prefix + "rksched" + mod_suffix;
                    message += const_prefix + line;
                    message = message + "\t" + "pub const STACK_SIZE: usize = super::rkplat::PAGE_SIZE*(1<<STACK_SIZE_PAGE_ORDER);\n\t";
                    OutputFile << message;
                    message = mod_prefix + "limit" + mod_suffix;
                    message += "\tuse core::time::Duration;\n\t";
                    OutputFile << message;
                    ++count;
                    break;
                }
                case 2:
                case 3:
                case 4: {
                    message = empty + "\t" + const_prefix + line + "\t";
                    OutputFile << message;
                    ++count;
                    break;
                }
                case 5: {
                    message = empty + "\t" + const_prefix + line + "\t";
                    OutputFile << message;
                    message = empty + mod_end + mod_end + "\n";
                    OutputFile << message;
                    message = empty + "/// rkplat Configuration\n";
                    message += mod_prefix + "rkplat" + mod_suffix + "/// Maximum number of CPU\n";
                    OutputFile << message;
                    ++count;
                    break;
                }
                case 6:
                case 7: {
                    message = empty + "\t" + const_prefix + line;
                    OutputFile << message;
                    ++count;
                    break;
                }
                case 8: {
                    message = empty + "\t" + const_prefix + line;
                    OutputFile << message;
                    message = mod_end + "\n";
                    message += "#[cfg(debug_assertions)]\npub const STACK_SIZE_SCALE: usize = 10;\n\n#[cfg(not(debug_assertions))]\npub const STACK_SIZE_SCALE: usize = 1;\n";
                    OutputFile << message;
                    ++count;
                    break;
                }
                default: {
                    if(!lib_name.empty() && line.find("=") == std::string::npos) {
                        features1 = features1 + "--features " + lib_name + "/" + line + " ";
                    } else if(!lib_name.empty() && line.find("=") != std::string::npos) {
                        message = empty + "\n" + mod_prefix + lib_name + mod_suffix;
                        message += const_prefix + line + mod_end;
                        OutputFile << message;
                        lib_name.clear();
                    } else if(line.find("=") == std::string::npos)
                        features2 = line;
                    break;
                }
            }
        }
    }
    OutputFile.close();
    InputFile.close();

    std::string features_filename = BUILD_DIR + "/features1.txt";
    std::ofstream FeaturesFile1(features_filename);
    FeaturesFile1 << features1;
    features_filename = BUILD_DIR + "/features2.txt";
    std::ofstream FeaturesFile2(features_filename);
    FeaturesFile2 << features2;
    FeaturesFile1.close();
    FeaturesFile2.close();
    return 0;
}

std::string replace_content(std::string content, unsigned int flag) {
    if(content.length() > 7 && content.substr(0, 7) == "CONFIG_") {
        content = content.replace(0, 7, "");
    }
    size_t pos;
    while((pos = content.find('\"')) != std::string::npos){
        content = content.replace(pos, 1, "");
    }
    if((pos = content.find('=')) != std::string::npos) {
        if(flag == 0) {
            std::string value_str = content.substr(pos+1);
            int value = std::atoi(value_str.c_str());
            if(value < 0) {
                content = content.replace(pos+1, content.length()-pos-1, "Duration::MAX");
            } else {
                if(value == 0) {
                    std::cerr << "Warning: CPU_TIME can't be 0!\n";
                    exit(-1);
                }
                std::string replace_str = "Duration::from_millis(";
                replace_str += content.substr(pos+1) + ")";
                content = content.replace(pos+1, content.length()-pos, replace_str);
            }
            content = content.replace(pos, 1, ": Duration = ") + ";\n";
        }
        else if(flag == 1) {
            content = content.replace(pos, 2, "");
        }
        else {
            std::string value_str = content.substr(pos+1);
            int value = std::atoi(value_str.c_str());
            if(value < 0) {
                content = content.replace(pos+1, content.length()-pos-1, "usize::MAX");
            }
            content = content.replace(pos, 1, ": usize = ") + ";\n";
        }
    }
    return content;
}