use reqwest::{Client, Error};
use std::collections::HashSet;
use scraper::{Html, Selector};
use tokio::time::{sleep, Duration};

pub struct Scraper {
    client: Client,
}

impl Scraper {
    pub fn new() -> Self {
        Scraper {
            client: Client::new(),
        }
    }

    pub async fn fetch_page(&self, url: &str) -> Result<String, Error> {
        let response = self.client.get(url).send().await?;
        println!("Status: {}", response.status());
        let body = response.text().await?;
        Ok(body)
    }

    pub async fn scrape(&self, url: &str, depth: usize) -> Result<String, Box<dyn std::error::Error>> {
        let mut visited = HashSet::new();
        let mut to_visit = vec![url.to_string()];
        let mut results = String::new();

        for _ in 0..depth {
            if to_visit.is_empty() {
                break;
            }

            let current_url = to_visit.remove(0);
            if visited.contains(&current_url) {
                continue;
            }

            visited.insert(current_url.clone());
            println!("Visiting: {}", current_url);

            match self.fetch_page(&current_url).await {
                Ok(body) => {
                    // Format content as Markdown (with a header)
                    results.push_str(&format!("# Content from {}\n\n", current_url));
                    results.push_str(&self.format_as_markdown(&body));
                    results.push_str("\n\n");

                    // Extract links and add to visit list
                    let links = self.extract_links(&body);
                    for link in links {
                        if !visited.contains(&link) {
                            to_visit.push(link);
                        }
                    }

                    sleep(Duration::from_millis(500)).await; // Prevent overloading servers
                }
                Err(e) => {
                    eprintln!("Failed to fetch {}: {}", current_url, e);
                }
            }
        }

        Ok(results)
    }

    fn extract_links(&self, body: &str) -> Vec<String> {
        let document = Html::parse_document(body);
        let selector = Selector::parse("a[href]").unwrap();
        
        document
            .select(&selector)
            .filter_map(|elem| elem.value().attr("href"))
            .map(String::from)
            .collect()
    }

    fn format_as_markdown(&self, body: &str) -> String {
        format!("```html\n{}\n```", body)
    }
}
