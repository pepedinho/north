use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
    terminal::ClearType,
    ExecutableCommand,
};

use crate::requests::find_repos::{GitHubClient, Repo};

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
                    Some(desc) => println!("> {} - {}", repo.full_name, desc),
                    None => println!("> {} - {}", repo.full_name, "null"),
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
                            println!("deeebug");
                        }
                        println!("debug")
                    }
                    KeyCode::Enter => {
                        let selected_repo = &repos[selected_index];
                        stdout.execute(cursor::MoveTo(0, (repos.len() + 2) as u16))?;
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
                                println!("Readme content:\n{}", readme);
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
                    _ => {}
                }
            }
        }
        //tokio::time::sleep(Duration::from_millis(50)).await;
    }
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}