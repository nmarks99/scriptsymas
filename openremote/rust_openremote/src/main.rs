mod shell;

fn main() {


    if shell::is_repo() { // check that cwd is a repo
        
        // cmd output is a std::process::Output struct
        let cmd_output: String;
        let get_repo_url_cmd = "git config --get remote.origin.url";
        
        // Run the command
        cmd_output = shell::cmd_output(get_repo_url_cmd);

        // Make a string from the output
        let repo_link:&str = &cmd_output[..];
        
        // Open url in browser
        open::that(repo_link).expect("Couldn't open the repo in browser"); 
    
    }
    
}

