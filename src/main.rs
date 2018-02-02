use std::env;
use std::process::Command;
use std::path::Path;
extern crate rand;
use rand::Rng;

fn string_to_vec(string : &str) -> Vec<&str> {
    let split = string.split(" ");
    return split.collect::<Vec<&str>>();
}

fn run_command(args : &str) {
    println!("Running: git {}", args);

    let args_vec = string_to_vec(args);

    /*let output =*/ Command::new("git")
            .args(args_vec)
            .output()
            .expect( "Could not execute the above git command.Is git installed?" ) ;

    //println!("Returned: {}", String::from_utf8_lossy(&output.stdout));
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

fn clean() {
    println!("Resetting...");
    run_command("reset --hard");

    git_clean_fancy();
}

fn get(args : &Vec<String>) {

    if args.len() < 3 && args.len() > 4 {
        help();
        return;
    }

    let url = &args[2];

    let mut branch = "master";
    if args.len() == 4 {
        branch = args[3].as_ref();
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

    let backup_branch: String = rand::thread_rng()
                                .gen_ascii_chars()
                                .take(16)
                                .collect();

    println!("Creating backup branch: {}...", backup_branch);
    run_command( format!("branch {}", backup_branch).as_ref() );

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
    "get <url> [branch (default=master)]" - Gets the specified repo into this folder (working dir), or if it already exists it will pull the latest if possible."
    "clean" - Resets and cleans everything in the repo."#);
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
        _ => help()
    }
}