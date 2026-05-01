use aidoku::alloc::{String, format};

/// URL for a manga's thumbnail image.
pub fn thumbnail_url(base_url: &str, manga_id: i32) -> String {
    format!("{}/api/v1/manga/{}/thumbnail", base_url, manga_id)
}

/// URL for a single page image.
///
/// Suwayomi page URLs use `sourceOrder` (1-indexed chapter position),
/// NOT the database chapter id.
/// The pages array returned by `fetchChapterPages` already contains
/// the correct relative paths, e.g. `/api/v1/manga/{id}/chapter/{sourceOrder}/page/{n}`.
/// This helper converts a relative path into an absolute URL.
pub fn absolute_page_url(base_url: &str, relative_path: &str) -> String {
    if relative_path.starts_with("http://") || relative_path.starts_with("https://") {
        String::from(relative_path)
    } else {
        format!("{}{}", base_url, relative_path)
    }
}
