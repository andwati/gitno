use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*, Terminal};
use std::io::{stdout, Stdout};

use crate::github::{GitHubGitignoreClient, GitignoreTemplate};
use crate::gitignore::GitignoreProcessor;

pub struct App {
    github_client: GitHubGitignoreClient,
    templates: Vec<GitignoreTemplate>,
    selected_templates: Vec<String>,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    pub async fn new() -> Result<Self> {
        // Set up terminal
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        let terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        // Fetch gitignore templates
        let github_client = GitHubGitignoreClient::new()?;
        let templates = github_client.fetch_gitignore_templates().await?;

        Ok(Self {
            github_client,
            templates,
            selected_templates: Vec::new(),
            terminal,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        loop {
            self.draw()?;

            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(' ') => self.toggle_template(),
                    KeyCode::Enter => self.generate_gitignore().await?,
                    _ => {}
                }
            }
        }

        self.cleanup()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(3),
                ])
                .split(frame.size());

            // Title
            let title = Paragraph::new("Gitno: Gitignore Template Generator")
                .style(Style::default().fg(Color::Blue).bold());
            frame.render_widget(title, chunks[0]);

            // Templates list
            let templates: Vec<ListItem> = self
                .templates
                .iter()
                .map(|template| {
                    let is_selected = self.selected_templates.contains(&template.name);
                    let content = format!(
                        "{} {}",
                        if is_selected { "[x]" } else { "[ ]" },
                        template.name
                    );
                    ListItem::new(content).style(Style::default().fg(if is_selected {
                        Color::Green
                    } else {
                        Color::White
                    }))
                })
                .collect();

            let list = List::new(templates).block(
                Block::default()
                    .title("Select Templates")
                    .borders(Borders::ALL),
            );
            frame.render_widget(list, chunks[1]);

            // Bottom instructions
            let instructions = Paragraph::new("Space: Select/Deselect | Enter: Generate | Q: Quit")
                .style(Style::default().fg(Color::Yellow));
            frame.render_widget(instructions, chunks[2]);
        })?;

        Ok(())
    }

    fn toggle_template(&mut self) {
        // Implement template selection logic
        // (simplified for brevity)
    }

    async fn generate_gitignore(&mut self) -> Result<()> {
        // Fetch contents of selected templates and combine them
        let mut template_contents = Vec::new();
        for template_name in &self.selected_templates {
            let content = self
                .github_client
                .fetch_template_content(template_name)
                .await?;
            template_contents.push(content);
        }

        let combined_gitignore = GitignoreProcessor::combine_gitignores(&template_contents)?;

        // Write to .gitignore in current directory
        GitignoreProcessor::write_gitignore(
            &combined_gitignore,
            std::path::Path::new(".gitignore"),
        )?;

        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;
        Ok(())
    }
}
