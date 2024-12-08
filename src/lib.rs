pub mod app;
pub mod github;
pub mod gitignore;
pub mod ui;

// Optional: Re-export key types if needed
pub use app::App;
pub use github::GitHubGitignoreClient;
pub use gitignore::GitignoreProcessor;
