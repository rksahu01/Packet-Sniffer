/// This enum defines the possible web pages to be opened.
#[derive(Debug, Clone)]
pub enum WebPage {
    // /// Sniffnet's GitHub repository.
    // Repo,
    // /// Sniffnet's website main page.
    // Website,
    // /// Sniffnet's website/download page.
    // WebsiteDownload,
    // /// My sponsor page
    // Sponsor,
    /// Sniffnet issue #60 on GitHub
    IssueLanguages,
    /// Sniffnet Wiki
    Wiki,
}

impl WebPage {
    pub fn get_url(&self) -> &str {
        match self {
            // WebPage::Repo => "https://github.com/ayushshaw90",
            // WebPage::Website => "https://github.com/ayushshaw90",
            // WebPage::Sponsor => "https://github.com/sponsors/ayushshaw90",
            // WebPage::WebsiteDownload => "https://www.sniffnet.net/download/",
            WebPage::IssueLanguages => "https://github.com/ayushshaw90",
            WebPage::Wiki => "https://github.com/ayushshaw90",
        }
    }
}
