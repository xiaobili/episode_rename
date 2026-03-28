use openlist_tui::models::episode::{EpisodeInfo, EpisodeParser};

#[test]
fn test_parse_s01e01() {
    let p = EpisodeParser::new();
    let r = p.parse("The.Office.S01E01.mkv").unwrap();
    assert_eq!(r.title, "The.Office");
    assert_eq!(r.season, 1);
    assert_eq!(r.episode, 1);
}

#[test]
fn test_parse_1x01() {
    let p = EpisodeParser::new();
    let r = p.parse("Breaking.Bad.1x01.720p.mkv").unwrap();
    assert_eq!(r.season, 1);
    assert_eq!(r.episode, 1);
}

#[test]
fn test_parse_of_format() {
    let p = EpisodeParser::new();
    let r = p.parse("Planet.Earth.1.of.10.720p.mkv").unwrap();
    assert_eq!(r.episode, 1);
}

#[test]
fn test_generate_name() {
    let p = EpisodeParser::new();
    let info = EpisodeInfo {
        title: "Show".into(),
        season: 1,
        episode: 5,
    };
    assert_eq!(
        p.generate_name(&info, "{title}.S{season}E{episode}", ".mkv"),
        "Show.S01E05.mkv"
    );
}

#[test]
fn test_no_match_default() {
    let p = EpisodeParser::new();
    let r = p.parse("Random.Video.mkv").unwrap();
    assert_eq!(r.season, 1);
    assert_eq!(r.episode, 1);
}
