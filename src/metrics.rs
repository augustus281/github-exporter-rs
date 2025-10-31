use prometheus::{Encoder, TextEncoder, IntGaugeVec, IntGauge, Registry};

/// The `Metrics` struct holds all Prometheus metric definitions
/// and provides helper methods to register and export them.
///
/// Each metric corresponds to a GitHub repository or API property,
/// such as stars, forks, open issues, or rate-limit information.
#[derive(Clone)]
pub struct Metrics {
    /// The Prometheus registry that holds all metrics.
    pub registry: Registry,

    /// Number of stars for a given repository.
    /// Label: `repo` (e.g. `"openai/gpt-5"`)
    pub repo_stars: IntGaugeVec,

    /// Number of forks for a given repository.
    /// Label: `repo`
    pub repo_forks: IntGaugeVec,

    /// Number of open issues for a given repository.
    /// Label: `repo`
    pub repo_issues: IntGaugeVec,

    /// Number of watchers for a given repository.
    /// Label: `repo`
    pub repo_watchers: IntGaugeVec,

    /// Remaining API requests for the current GitHub rate-limit window.
    pub rate_limit_remaining: IntGauge,

    /// Unix timestamp for when the current rate-limit window resets.
    pub rate_limit_reset: IntGauge,
}

impl Metrics {
    /// Creates and registers a new set of GitHub metrics in a fresh Prometheus registry.
    ///
    /// This method defines all the metric names, types, help texts, and labels,
    /// and then registers them into the internal `Registry`.
    ///
    /// # Returns
    /// A `Metrics` instance with all metrics initialized and registered.
    pub fn new() -> Self {
        // Create a new empty Prometheus registry
        let registry = Registry::new();

        // --- Define metrics for repositories ---
        let repo_stars = IntGaugeVec::new(
            prometheus::Opts::new("github_repo_stars", "Number of stars for a repo"),
            &["repo"],
        ).unwrap();

        let repo_forks = IntGaugeVec::new(
            prometheus::Opts::new("github_repo_forks", "Number of forks for a repo"),
            &["repo"],
        ).unwrap();

        let repo_issues = IntGaugeVec::new(
            prometheus::Opts::new("github_repo_open_issues", "Number of open issues for a repo"),
            &["repo"],
        ).unwrap();

        let repo_watchers = IntGaugeVec::new(
            prometheus::Opts::new("github_repo_watchers", "Number of watchers for a repo"),
            &["repo"],
        ).unwrap();

        // --- Define metrics for rate limits ---
        let rate_limit_remaining = IntGauge::new(
            "github_rate_limit_remaining",
            "Remaining API requests for the current rate limit window",
        ).unwrap();

        let rate_limit_reset = IntGauge::new(
            "github_rate_limit_reset",
            "Unix timestamp when the GitHub API rate limit resets",
        ).unwrap();

        // --- Register all metrics in the registry ---
        registry.register(Box::new(repo_stars.clone())).unwrap();
        registry.register(Box::new(repo_forks.clone())).unwrap();
        registry.register(Box::new(repo_issues.clone())).unwrap();
        registry.register(Box::new(repo_watchers.clone())).unwrap();
        registry.register(Box::new(rate_limit_remaining.clone())).unwrap();
        registry.register(Box::new(rate_limit_reset.clone())).unwrap();

        Self {
            registry,
            repo_stars,
            repo_forks,
            repo_issues,
            repo_watchers,
            rate_limit_remaining,
            rate_limit_reset,
        }
    }

    /// Gathers all metric values and encodes them into the standard
    /// Prometheus text exposition format.
    ///
    /// # Returns
    /// A `String` containing all metrics in the same format Prometheus scrapes
    /// from HTTP endpoints (e.g., `/metrics`).
    ///
    /// # Example
    /// ```
    /// let metrics = Metrics::new();
    /// println!("{}", metrics.gather_text());
    /// ```
    pub fn gather_text(&self) -> String {
        let mut buffer = Vec::new();
        let encoder = TextEncoder::new();

        // Gather all registered metrics
        let mf = self.registry.gather();

        // Encode to text format
        encoder.encode(&mf, &mut buffer).unwrap();

        // Convert to UTF-8 string
        String::from_utf8(buffer).unwrap()
    }
}

