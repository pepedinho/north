use std::{
    io::{self, Cursor, Write},
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
    parsing::parsing::extract_install_section, requests::requests::{GitHubClient, Repo}
};


pub async fn display_repos(repos: Vec<Repo>, client: GitHubClient) -> io::Result<()> {
    let mut selected_index = 0;
    let mut stdout = io::stdout();

    crossterm::terminal::enable_raw_mode()?;
    stdout.execute(crossterm::terminal::Clear(ClearType::All))?;
    loop {
        stdout.execute(cursor::MoveTo(0, 0))?;

        // displaying all repos matching with the asked name 
        for (i, repo) in repos.iter().enumerate() {
            stdout.execute(cursor::MoveTo(0, i as u16))?;
            let (prefix, color_start, color_end) = if i == selected_index {
                (">", "\x1b[31m>\x1b[34m", "\x1b[0m")
            } else {
                (" ", "", "")
            };

            let desc = repo.description.as_deref().unwrap_or("No desc");

            if i == selected_index {
                println!("{} {} - {}{}", color_start, repo.full_name, desc, color_end);
            } else {
                println!("{} {} - {}", prefix, repo.full_name, desc);
            }

        }

        stdout.flush()?;

        if event::poll(Duration::from_millis(100))? {
            if let event::Event::Key(KeyEvent {code, ..}) = event::read()? {
                match code {
                    KeyCode::Up if selected_index > 0 => selected_index -= 1,
                    KeyCode::Down if selected_index < repos.len() - 1 => selected_index += 1,
                    KeyCode::Enter => {
                        let selected_repo = &repos[selected_index];

                        stdout.execute(cursor::MoveTo(0, (repos.len() + 2) as u16))?;
                        crossterm::terminal::disable_raw_mode()?;
                        display_selected_repos(selected_repo);
                        if let Err(e) = instal_from_readme(&client, selected_repo).await {
                            eprintln!("Error processing README for {} : {}", selected_repo.full_name, e);
                        }

                        break;
                    }
                    KeyCode::Esc => break,
                    KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
        //tokio::time::sleep(Duration::from_millis(50)).await;
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

fn display_selected_repos(repo: &Repo) {
    let desc = repo.description.as_deref().unwrap_or("No desc");
    println!("\nnSelected repo: {} - {}", repo.full_name, desc);
}

async fn instal_from_readme(client: &GitHubClient, repo: &Repo) -> Result<(), Box<dyn std::error::Error>>{
    match client.get_readme(&repo.owner.login, &repo.name).await {
        Ok(readme) => {
            if let Some(cmd) = extract_install_section(&readme) {
                let formated_cmd = cmd.lines().skip(1).collect::<Vec<_>>().join(";");
                println!("Instalation command :\n {}", formated_cmd);
                let status = Command::new("sh")
                    .arg("-c")
                    .arg(&formated_cmd)
                    .stdin(Stdio::inherit())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .status()?;
                
                if !status.success() {
                    eprintln!("failed to install {}.", repo.name);
                }
            }
        }
        Err(e) => eprintln!("Failed to fetch the README : {}", e),
    }
    Ok(())
}