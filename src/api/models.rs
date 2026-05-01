use aidoku::alloc::{String, Vec};
use serde::Deserialize;

// ── GraphQL response wrappers ─────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct GqlResponse<T> {
    pub data: T,
}

// ── Manga list ────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct MangasData {
    pub mangas: MangasConnection,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MangasConnection {
    pub nodes: Vec<MangaNode>,
    pub total_count: i32,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MangaNode {
    pub id: i32,
    pub title: String,
    pub thumbnail_url: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub artist: Option<String>,
    pub genre: Option<Vec<String>>,
    pub status: Option<String>,
    pub real_url: Option<String>,
    pub in_library: Option<bool>,
    pub last_fetched_at: Option<String>,
}

// ── Single manga ──────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct MangaData {
    pub manga: MangaNode,
}

// ── Chapters ──────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ChaptersData {
    pub chapters: ChaptersConnection,
}

#[derive(Deserialize)]
pub struct ChaptersConnection {
    pub nodes: Vec<ChapterNode>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChapterNode {
    pub id: i32,
    pub name: String,
    pub chapter_number: Option<f32>,
    pub upload_date: Option<String>,
    pub scanlator: Option<String>,
    pub real_url: Option<String>,
    pub source_order: i32,
}

// ── Categories ────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct CategoriesData {
    pub categories: CategoriesConnection,
}

#[derive(Deserialize)]
pub struct CategoriesConnection {
    pub nodes: Vec<CategoryNode>,
}

#[derive(Deserialize, Clone)]
pub struct CategoryNode {
    pub id: i32,
    pub name: String,
}

// ── fetchChapterPages mutation ────────────────────────────────────────────────

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchChapterPagesData {
    pub fetch_chapter_pages: FetchChapterPagesPayload,
}

#[derive(Deserialize)]
pub struct FetchChapterPagesPayload {
    pub pages: Vec<String>,
}

// ── Recently updated (chapters with manga info) ───────────────────────────────

#[derive(Deserialize)]
pub struct RecentChaptersData {
    pub chapters: RecentChaptersConnection,
}

#[derive(Deserialize)]
pub struct RecentChaptersConnection {
    pub nodes: Vec<RecentChapterNode>,
}

#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RecentChapterNode {
    pub id: i32,
    pub name: String,
    pub chapter_number: Option<f32>,
    pub upload_date: Option<String>,
    pub manga: MangaNode,
}
