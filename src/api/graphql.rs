use aidoku::{
    alloc::{String, format},
    imports::net::Request,
    prelude::*,
    Result,
};
use serde::Serialize;
use super::models::*;

const PAGE_SIZE: i32 = 20;

/// Build the Authorization header value from optional credentials.
fn auth_header(username: &Option<String>, password: &Option<String>) -> Option<String> {
    let u = username.as_deref().filter(|s| !s.is_empty())?;
    let p = password.as_deref().unwrap_or("");
    // base64 encode "user:pass"  — we build it manually to avoid std/alloc issues
    let creds = format!("{}:{}", u, p);
    Some(format!("Basic {}", base64_encode(creds.as_bytes())))
}

/// Minimal base64 encoder (no padding stripping needed for Basic Auth).
fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    let mut i = 0;
    while i < input.len() {
        let b0 = input[i] as u32;
        let b1 = if i + 1 < input.len() { input[i + 1] as u32 } else { 0 };
        let b2 = if i + 2 < input.len() { input[i + 2] as u32 } else { 0 };
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((n >> 18) & 0x3f) as usize] as char);
        out.push(TABLE[((n >> 12) & 0x3f) as usize] as char);
        if i + 1 < input.len() {
            out.push(TABLE[((n >> 6) & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        if i + 2 < input.len() {
            out.push(TABLE[(n & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
        i += 3;
    }
    out
}

/// Send a GraphQL query to the Suwayomi server.
fn graphql_query<T>(
    base_url: &str,
    query: &str,
    username: &Option<String>,
    password: &Option<String>,
) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    #[derive(Serialize)]
    struct GqlBody<'a> {
        query: &'a str,
    }

    let url = format!("{}/api/graphql", base_url);
    let body = serde_json::to_vec(&GqlBody { query })
        .map_err(|e| error!("JSON serialize error: {:?}", e))?;

    let mut req = Request::post(&url).map_err(|e| error!("Request error: {:?}", e))?;
    req.set_header("Content-Type", "application/json");
    req.set_header("Accept", "application/json");
    if let Some(auth) = auth_header(username, password) {
        req.set_header("Authorization", &auth);
    }
    req.set_body(&body);

    let resp: GqlResponse<T> = req
        .send()
        .map_err(|e| error!("Send error: {:?}", e))?
        .get_json_owned()
        .map_err(|e| error!("JSON error: {:?}", e))?;

    Ok(resp.data)
}

/// Helper: collapse whitespace in a GraphQL query string.
fn build_query(q: &str) -> String {
    q.replace('\n', " ").replace("  ", " ")
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Fetch a paginated list of manga, optionally filtered by title/status/sort.
pub fn fetch_mangas(
    base_url: &str,
    username: &Option<String>,
    password: &Option<String>,
    title_query: Option<&str>,
    status_filter: Option<&str>,
    sort_field: &str,
    sort_asc: bool,
    page: i32,
) -> Result<MangasData> {
    let offset = (page - 1) * PAGE_SIZE;
    let order_dir = if sort_asc { "ASC" } else { "DESC" };

    // Build optional filter clauses — use real double-quotes; serde_json will escape them
    let title_clause = match title_query {
        Some(q) if !q.is_empty() => {
            let safe = q.replace('"', "").replace('\\', "");
            format!(r#"filter: {{title: {{likeInsensitive: "%{}%"}}}}"#, safe)
        }
        _ => String::new(),
    };

    let condition_clause = match status_filter {
        Some(s) if s != "Any" && !s.is_empty() => {
            format!("condition: {{inLibrary: true, status: {}}}", s.to_uppercase())
        }
        _ => String::from("condition: {inLibrary: true}"),
    };

    let query = build_query(&format!(
        r#"{{
  mangas(
    {condition_clause}
    {title_clause}
    orderBy: {sort_field}
    orderByType: {order_dir}
    first: {PAGE_SIZE}
    offset: {offset}
  ) {{
    totalCount
    nodes {{
      id title thumbnailUrl description author artist
      genre status realUrl inLibrary
    }}
  }}
}}"#
    ));

    graphql_query(base_url, &query, username, password)
}

/// Fetch a single manga by its numeric ID.
pub fn fetch_manga(
    base_url: &str,
    username: &Option<String>,
    password: &Option<String>,
    manga_id: i32,
) -> Result<MangaData> {
    let query = build_query(&format!(
        r#"{{
  manga(id: {manga_id}) {{
    id title thumbnailUrl description author artist
    genre status realUrl inLibrary
  }}
}}"#
    ));

    graphql_query(base_url, &query, username, password)
}

/// Fetch all chapters for a manga.
pub fn fetch_chapters(
    base_url: &str,
    username: &Option<String>,
    password: &Option<String>,
    manga_id: i32,
) -> Result<ChaptersData> {
    let query = build_query(&format!(
        r#"{{
  chapters(
    condition: {{mangaId: {manga_id}}}
    orderBy: SOURCE_ORDER
    orderByType: ASC
  ) {{
    nodes {{
      id name chapterNumber uploadDate scanlator
      realUrl sourceOrder
    }}
  }}
}}"#
    ));

    graphql_query(base_url, &query, username, password)
}

/// Fetch page URLs for a chapter via mutation.
pub fn fetch_chapter_pages(
    base_url: &str,
    username: &Option<String>,
    password: &Option<String>,
    chapter_id: i32,
) -> Result<FetchChapterPagesData> {
    let query = build_query(&format!(
        r#"mutation {{
  fetchChapterPages(input: {{chapterId: {chapter_id}}}) {{
    pages
  }}
}}"#
    ));

    graphql_query(base_url, &query, username, password)
}

/// Fetch all user-defined categories.
pub fn fetch_categories(
    base_url: &str,
    username: &Option<String>,
    password: &Option<String>,
) -> Result<CategoriesData> {
    let query = build_query(
        r#"{
  categories {
    nodes { id name }
  }
}"#,
    );

    graphql_query(base_url, &query, username, password)
}

/// Fetch recently-updated chapters with manga info.
pub fn fetch_recent_chapters(
    base_url: &str,
    username: &Option<String>,
    password: &Option<String>,
    page: i32,
) -> Result<RecentChaptersData> {
    let offset = (page - 1) * PAGE_SIZE;

    let query = build_query(&format!(
        r#"{{
  chapters(
    orderBy: FETCH_DATE
    orderByType: DESC
    first: {PAGE_SIZE}
    offset: {offset}
  ) {{
    nodes {{
      id name chapterNumber uploadDate
      manga {{
        id title thumbnailUrl description author artist
        genre status realUrl inLibrary
      }}
    }}
  }}
}}"#
    ));

    graphql_query(base_url, &query, username, password)
}

/// Fetch manga for a specific category.
pub fn fetch_mangas_by_category(
    base_url: &str,
    username: &Option<String>,
    password: &Option<String>,
    category_id: i32,
    page: i32,
) -> Result<MangasData> {
    let offset = (page - 1) * PAGE_SIZE;

    let query = build_query(&format!(
        r#"{{
  mangas(
    filter: {{categoryId: {{equalTo: {category_id}}}}}
    condition: {{inLibrary: true}}
    orderBy: TITLE
    orderByType: ASC
    first: {PAGE_SIZE}
    offset: {offset}
  ) {{
    totalCount
    nodes {{
      id title thumbnailUrl description author artist
      genre status realUrl inLibrary
    }}
  }}
}}"#
    ));

    graphql_query(base_url, &query, username, password)
}

pub fn page_count() -> i32 {
    PAGE_SIZE
}
