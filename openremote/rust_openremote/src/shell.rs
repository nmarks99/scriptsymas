use std::process::{Command};
use std::env;

pub fn cmd_output(command:&str) -> String {

    // See if on windows 
    let windows = cfg!(target_os = "windows");
    let prog:&str;
    let prog_flag:&str;

    if windows == true {
        prog = "cmd";
        prog_flag = "/C";
    }
    else{
        // Presumably the other option is a UNIX machine with "sh"
        // If you have git-bash on windows this works for windows too
        prog = "sh";
        prog_flag = "-c";
    }

    let command = String::from(command);
    let output = Command::new(prog)
                .arg(prog_flag) // this is important but idk why
                .arg(command)
                .output()
                .expect("Command failed to execute");

    let output = String::from_utf8(output.stdout).unwrap();

    return output;
}

pub fn is_repo() -> bool {

    let cwd = env::current_dir().unwrap();
    let dir = cwd.as_path().display().to_string();

    // If directory name is "GitHub" open my repository page on GitHub 
    if &dir[dir.len()-6..dir.len()] == "GitHub"{
        open::that("https://github.com/").expect("Could not open GitHub.com");
        return false;
    }
    else{
        let check_if_repo_cmd = "git rev-parse --is-inside-work-tree";
        let output = cmd_output(check_if_repo_cmd);
        let output = &output[..];

        match output {
            "true\n" => return true,
            _ => {
                    println!("Current directory is not a git repository");
                    return false;
                }
        }
    }
    
}
