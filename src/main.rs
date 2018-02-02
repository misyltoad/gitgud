use std::env;
use std::process::Command;
use std::path::Path;
extern crate rand;
use rand::Rng;

fn string_to_vec(string : &str) -> Vec<&str> {
    let split = string.split(" ");
    return split.collect::<Vec<&str>>();
}

fn run_command(args : &str) -> String {
    println!("Running: git {}", args);

    let args_vec = string_to_vec(args);

    let output = Command::new("git")
            .args(args_vec)
            .output()
            .expect( "Could not execute the above git command. Is git installed?" );

    //println!("Returned: {}", String::from_utf8_lossy(&output.stdout));

    let mut ret_val: String = "".to_string();
    ret_val.push_str(&String::from_utf8_lossy(&output.stdout));
    return ret_val.trim().to_string();
}

fn path_exists(dir_path : &str) -> bool {
    let path = Path::new(dir_path);
    return path.exists();
}

fn repo_exists() -> bool {
    return path_exists(".git");
}

fn git_clean_fancy() {
    println!("Cleaning...");
    run_command("clean -fdx");
}

fn git_get_current_branch() -> String {
    return run_command("rev-parse --abbrev-ref HEAD");
}

fn clean() {
    println!("Resetting...");
    run_command("reset --hard");

    git_clean_fancy();
}

fn get_random_branch_name() -> String {
    return rand::thread_rng()
                .gen_ascii_chars()
                .take(16)
                .collect();
}

fn create_backup_branch() {
    let backup_branch = get_random_branch_name();

    println!("Creating backup branch: {}...", backup_branch);
    run_command( format!("branch {}", backup_branch).as_ref() );
}

fn update(args : &Vec<String>) {
    if args.len() > 3 {
        help();
        return;
    }

    let mut branch = git_get_current_branch();

    if args.len() == 3 {
        branch = args[2].to_string();
    }

    create_backup_branch();

    println!("Fetching...");
    run_command("fetch --depth=1");

    println!("Resetting to origin/{}...", branch);
    run_command( format!("reset --hard origin/{}", branch).as_ref() );

    git_clean_fancy();
}

fn get(args : &Vec<String>) {

    if args.len() < 3 && args.len() > 4 {
        help();
        return;
    }

    let url = &args[2];

    let mut branch = git_get_current_branch();
    if args.len() == 4 {
        branch = args[3].to_string();
    }

    println!("Getting repo: {}.", url);

    if !repo_exists() {
        println!("Local repo doesn't exist here. Creating...");
        run_command("init");
    }
    else {
        println!("Using existing local git.");
    }
    
    println!("Setting origin to {}...", url);
    run_command("remote remove origin");
    run_command(format!("remote add origin {}", url).as_ref());

    println!("Checking out branch {}...", branch);
    run_command( format!("checkout {}", branch).as_ref() );

    create_backup_branch();

    println!("Fetching...");
    run_command("fetch --depth=1");

    println!("Resetting to origin/{}...", branch);
    run_command( format!("reset --hard origin/{}", branch).as_ref() );

    git_clean_fancy();
}

fn help() {
    println!(
r#"Usage: gitgud <command> [<args>]
Commands:
    "get <url> [branch (default is master or current)]" - Gets the specified repo into this folder (working dir), or if it already exists it will pull the latest if possible. Creates a backup branch and then cleans and resets to avoid conflicts."
    "clean" - Resets and cleans everything in the repo."
    "update [branch (default is current)]" - Updates the current repo and creates a backup branch to avoid conflicts, then resets and cleans up."#);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        help();
        return;
    }

    let command = &args[1];

    match command.as_ref() {
        "get" => get(&args),
        "clean" => clean(),
        "update" => update(&args),
        _ => help()
    }
}