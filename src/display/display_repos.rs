use std::{
    io::{self, Write},
    process::Stdio,
    time::Duration,
};

use std::process::Command;

use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
    terminal::ClearType,
    ExecutableCommand,
};

use crate::{
    parsing::parsing_readme::extract_install_section,
    requests::find_repos::{GitHubClient, Repo},
};

pub async fn display_repos(repos: Vec<Repo>, client: GitHubClient) -> io::Result<()> {
    let mut selected_index = 0;
    let mut stdout = io::stdout();

    crossterm::terminal::enable_raw_mode()?;
    stdout.execute(crossterm::terminal::Clear(ClearType::All))?;
    loop {
        stdout.execute(cursor::MoveTo(0, 0))?;

        for (i, repo) in repos.iter().enumerate() {
            if i == selected_index {
                stdout.execute(cursor::MoveTo(0, i as u16))?;
                match &repo.description {
                    Some(desc) => {
                        println!("\x1b[31m>\x1b[34m {} - {}\x1b[0m", repo.full_name, desc)
                    }
                    None => println!("\x1b[31m>\x1b[34m {} - {}\x1b[0m", repo.full_name, "null"),
                }
            } else {
                stdout.execute(cursor::MoveTo(0, i as u16))?;
                match &repo.description {
                    Some(desc) => println!("  {} - {}", repo.full_name, desc),
                    None => println!("  {} - {}", repo.full_name, "null"),
                }
            }
        }

        stdout.flush()?;

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Up => {
                        if selected_index > 0 {
                            selected_index -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if selected_index < repos.len() - 1 {
                            selected_index += 1;
                        }
                    }
                    KeyCode::Enter => {
                        let selected_repo = &repos[selected_index];
                        stdout.execute(cursor::MoveTo(0, (repos.len() + 2) as u16))?;
                        crossterm::terminal::disable_raw_mode()?;
                        match &selected_repo.description {
                            Some(desc) => {
                                println!("\nSelected repo: {} - {}", selected_repo.full_name, desc)
                            }
                            None => println!("  {} - {}", selected_repo.full_name, "null"),
                        };
                        let readme_result = client
                            .get_readme(&selected_repo.owner.login, &selected_repo.name)
                            .await;
                        match readme_result {
                            Ok(readme) => {
                                if let Some(cmd) = extract_install_section(&readme) {
                                    let formated_cmd =
                                        cmd.lines().skip(1).collect::<Vec<&str>>().join(";");
                                    println!("Installation Command:\n {}", formated_cmd);
                                    //let mut command_parts = cmd.split_whitespace();
                                    //let program = command_parts.next().unwrap();
                                    //let args: Vec<&str> = command_parts.collect();
                                    let mut process = Command::new("sh")
                                        .arg("-c")
                                        .arg(formated_cmd)
                                        .stdin(Stdio::inherit())
                                        .stdout(Stdio::inherit())
                                        .stderr(Stdio::inherit())
                                        .spawn()
                                        .expect("Failed to run the install process");
                                    let output = process
                                        .wait_with_output()
                                        .expect("Failed to wait proccess");

                                    if output.status.success() {
                                        break;
                                    } else {
                                        println!(
                                            "Command execution failed :\n{}",
                                            String::from_utf8_lossy(&output.stderr)
                                        );
                                        println!(
                                            "Command executed failed :\n{}",
                                            String::from_utf8_lossy(&output.stdout)
                                        );
                                    }
                                }
                                //println!("Readme content:\n{}", readme);
                            }
                            Err(e) => {
                                eprintln!(
                                    "Failed to fetch README for {} - {}",
                                    selected_repo.full_name, e
                                );
                            }
                        }
                        break;
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
        //tokio::time::sleep(Duration::from_millis(50)).await;
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
