mod config;
mod metrics;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cfg = config::Config::from_env()?;
    let metrics = metrics::Metrics::new();

    // Example: simulate repo metrics
    for repo in &cfg.repos {
        metrics.repo_stars.with_label_values(&[repo]).set(120);
        metrics.repo_forks.with_label_values(&[repo]).set(40);
        metrics.repo_issues.with_label_values(&[repo]).set(7);
        metrics.repo_watchers.with_label_values(&[repo]).set(9);
    }

    metrics.rate_limit_remaining.set(4990);
    metrics.rate_limit_reset.set(1730000000);

    println!("{}", metrics.gather_text());
    Ok(())
}
