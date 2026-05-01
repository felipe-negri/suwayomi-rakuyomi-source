use aidoku::{
    alloc::{String, Vec, format},
    Chapter, ContentRating, Manga, MangaStatus, Page, PageContent, Viewer,
};
use crate::api::{models::*, rest};

/// Convert a Suwayomi status string to an Aidoku MangaStatus.
pub fn status_from_str(status: Option<&str>) -> MangaStatus {
    match status {
        Some("ONGOING") => MangaStatus::Ongoing,
        Some("COMPLETED") => MangaStatus::Completed,
        Some("CANCELLED") => MangaStatus::Cancelled,
        Some("HIATUS") => MangaStatus::Hiatus,
        _ => MangaStatus::Unknown,
    }
}

/// Convert a Suwayomi MangaNode into an Aidoku Manga.
pub fn manga_from_node(node: &MangaNode, base_url: &str) -> Manga {
    let cover = Some(rest::thumbnail_url(base_url, node.id));

    let authors = node
        .author
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|a| Vec::from([String::from(a)]));

    let artists = node
        .artist
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|a| Vec::from([String::from(a)]));

    let tags = node.genre.clone().map(|g| g.into_iter().collect());

    let content_rating = if node
        .genre
        .as_ref()
        .map(|g| g.iter().any(|tag| {
            let t = tag.to_lowercase();
            t.contains("adult") || t.contains("mature") || t.contains("ecchi") || t.contains("hentai")
        }))
        .unwrap_or(false)
    {
        ContentRating::NSFW
    } else {
        ContentRating::Safe
    };

    Manga {
        key: format!("{}", node.id),
        title: node.title.clone(),
        cover,
        authors,
        artists,
        description: node.description.clone(),
        url: node.real_url.clone(),
        tags,
        status: status_from_str(node.status.as_deref()),
        content_rating,
        viewer: Viewer::RightToLeft,
        ..Default::default()
    }
}

/// Convert a Suwayomi ChapterNode into an Aidoku Chapter.
pub fn chapter_from_node(node: &ChapterNode) -> Chapter {
    // Parse upload date — Suwayomi returns a Unix timestamp in milliseconds as a string
    let date_uploaded = node
        .upload_date
        .as_deref()
        .and_then(|s| s.parse::<i64>().ok())
        .map(|ms| ms / 1000); // convert ms → seconds

    let scanlators = node
        .scanlator
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| Vec::from([String::from(s)]));

    // Extract chapter title by stripping the volume/chapter number prefix
    // e.g. "Vol.1 Ch.1 - Renascido" → "Renascido"
    let title = extract_chapter_title(&node.name);

    Chapter {
        key: format!("{}", node.id),
        title,
        chapter_number: node.chapter_number,
        date_uploaded,
        scanlators,
        url: node.real_url.clone(),
        ..Default::default()
    }
}

/// Convert page URLs returned by fetchChapterPages into Aidoku Pages.
pub fn pages_from_urls(urls: &[String], base_url: &str) -> Vec<Page> {
    urls.iter()
        .map(|path| Page {
            content: PageContent::url(rest::absolute_page_url(base_url, path)),
            ..Default::default()
        })
        .collect()
}

/// Convert a RecentChapterNode into an Aidoku Chapter.
pub fn chapter_from_node_recent(node: &RecentChapterNode) -> Chapter {
    let date_uploaded = node
        .upload_date
        .as_deref()
        .and_then(|s| s.parse::<i64>().ok())
        .map(|ms| ms / 1000);

    Chapter {
        key: format!("{}", node.id),
        title: extract_chapter_title(&node.name),
        chapter_number: node.chapter_number,
        date_uploaded,
        ..Default::default()
    }
}

/// Try to extract a clean title from chapter names like "Vol.1 Ch.1 - Title".
fn extract_chapter_title(name: &str) -> Option<String> {
    // If there's a " - " separator, use the part after it as the title
    if let Some(idx) = name.find(" - ") {
        let title = name[idx + 3..].trim();
        if !title.is_empty() {
            return Some(String::from(title));
        }
    }
    // If the name is just a number/volume marker with no additional text, return None
    None
}
