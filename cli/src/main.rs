use serde::{Deserialize, Serialize};
use std::error;
use std::io::Write;
use std::{env, io, process::Command};

#[derive(Serialize, Deserialize)]
pub struct BranchNameGeneratorRequest {
    input: Input,
}

#[derive(Serialize, Deserialize)]
pub struct Input {
    diff: String,
}

#[derive(Serialize, Deserialize)]
pub struct BranchNameGeneratorResponse {
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    result: Result,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    branch: String,
    reason: String,
}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let diff = get_diff()?;
    if diff.is_empty() {
        println!("Nothing to generate the branch name from");
        return Ok(());
    }

    println!("Generating the new branch name...");
    let result = execute_branch_name_generator(diff).await?;
    println!();
    println!("Generated branch name: {}", result.branch);
    println!();
    println!("Reason: {}", result.reason);
    println!();

    match prompt("Do you want to approve this branch name?")? {
        true => {
            move_branch(&result.branch)?;
        }
        false => {
            println!("Aborted");
        }
    }
    Ok(())
}

fn get_diff() -> std::result::Result<String, Box<dyn error::Error>> {
    let git = Command::new("git")
        .arg("--no-pager")
        .arg("diff")
        .arg("main..HEAD")
        .output()?;
    let stdout = String::from_utf8_lossy(&git.stdout);
    Ok(stdout.to_string())
}

async fn execute_branch_name_generator(
    diff: String,
) -> std::result::Result<Result, Box<dyn error::Error>> {
    let host = env::var("AIBRANCH_HOST")
        .unwrap_or_else(|_e| "https://aibranch-worker.sakkeam.workers.dev".to_string());

    let url = format!("{}/workflows/branch-name-generator/execute", host);
    let json = BranchNameGeneratorRequest {
        input: Input { diff },
    };
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .json(&json)
        .send()
        .await?
        .json::<BranchNameGeneratorResponse>()
        .await?;
    Ok(res.data.result)
}

fn prompt(message: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
    print!("{} [Y/n] ", message);
    io::stdout().flush()?;

    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;

    match buf.to_lowercase().trim() {
        "y" | "" => Ok(true),
        _ => Ok(false),
    }
}

fn move_branch(name: &str) -> std::result::Result<bool, Box<dyn std::error::Error>> {
    let git = Command::new("git")
        .arg("branch")
        .arg("--move")
        .arg(name)
        .output()?;
    Ok(git.status.success())
}
