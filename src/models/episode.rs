use regex::Regex;

#[derive(Debug, Clone)]
pub struct EpisodeInfo {
    pub title: String,
    pub season: u32,
    pub episode: u32,
}

pub struct EpisodeParser {
    patterns: Vec<Regex>,
}

impl EpisodeParser {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                // S01E01 format
                Regex::new(r"(.+?)[\s._-]*S(\d+)E(\d+)").unwrap(),
                // 1x01 format (must have exactly 2 digits for episode)
                Regex::new(r"(.+?)[\s._-]*(\d+)x(\d{2})").unwrap(),
                // EP1 format
                Regex::new(r"(.+?)[\s._-]*EP(\d+)").unwrap(),
                // "1 of 10" format
                Regex::new(r"(.+?)[\s._-]*(\d+)\s*of\s*\d+").unwrap(),
            ],
        }
    }

    pub fn parse(&self, filename: &str) -> Option<EpisodeInfo> {
        let name = filename
            .rsplit_once('.')
            .map(|(n, _)| n)
            .unwrap_or(filename);
        for p in &self.patterns {
            if let Some(caps) = p.captures(name) {
                if caps.len() >= 4 {
                    return Some(EpisodeInfo {
                        title: caps.get(1)?.as_str().trim().to_string(),
                        season: caps.get(2)?.as_str().parse().unwrap_or(1),
                        episode: caps.get(3)?.as_str().parse().unwrap_or(1),
                    });
                }
            }
        }
        // No match - return default
        Some(EpisodeInfo {
            title: name.into(),
            season: 1,
            episode: 1,
        })
    }

    pub fn generate_name(&self, info: &EpisodeInfo, pattern: &str, ext: &str) -> String {
        let s = format!("{:02}", info.season);
        let e = format!("{:02}", info.episode);
        format!(
            "{}{}",
            pattern
                .replace("{title}", &info.title)
                .replace("{season}", &s)
                .replace("{episode}", &e),
            ext
        )
    }
}

impl Default for EpisodeParser {
    fn default() -> Self {
        Self::new()
    }
}
