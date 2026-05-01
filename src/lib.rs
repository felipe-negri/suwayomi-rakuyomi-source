#![no_std]

use aidoku::{
    alloc::{String, Vec},

    imports::defaults::defaults_get,
    prelude::*,
    Chapter, Filter, FilterValue, Home, HomeComponent, HomeComponentValue, HomeLayout,
    Listing, ListingKind, ListingProvider, Manga, MangaPageResult, MangaWithChapter, Page,
    DynamicFilters, DynamicSettings, Result, SelectFilter, Setting, SortFilter,
    SortFilterDefault, Source,
};

mod api;
mod helpers;

use api::graphql;

const PAGE_SIZE: i32 = 20;

// ── Sort field constants (GraphQL enum values) ────────────────────────────────
const SORT_TITLE: &str = "TITLE";
const SORT_LAST_FETCHED: &str = "LAST_FETCHED_AT";
const SORT_IN_LIBRARY_AT: &str = "IN_LIBRARY_AT";

struct SuwayomiSource;

impl SuwayomiSource {
    fn server_url(&self) -> Result<String> {
        defaults_get::<String>("serverUrl")
            .filter(|s| !s.is_empty())
            .ok_or(aidoku::AidokuError::Message(aidoku::alloc::String::from("Server URL not configured. Please set the Suwayomi server URL in source settings.")))
    }

    fn username(&self) -> Option<String> {
        defaults_get::<String>("username").filter(|s| !s.is_empty())
    }

    fn password(&self) -> Option<String> {
        defaults_get::<String>("password").filter(|s| !s.is_empty())
    }

    /// Parse filter values into sort field, sort direction, and status string.
    fn parse_filters(filters: &[FilterValue]) -> (&'static str, bool, String) {
        let mut sort_field = SORT_LAST_FETCHED;
        let mut sort_asc = false;
        let mut status = String::new(); // empty = no status filter

        for filter in filters {
            match filter {
                FilterValue::Sort { id, index, ascending } if id == "sort" => {
                    sort_field = match *index {
                        0 => SORT_TITLE,
                        1 => SORT_LAST_FETCHED,
                        2 => SORT_IN_LIBRARY_AT,
                        _ => SORT_LAST_FETCHED,
                    };
                    sort_asc = *ascending;
                }
                FilterValue::Select { id, value } if id == "status" => {
                    if value != "Any" && !value.is_empty() {
                        status = value.clone();
                    }
                }
                _ => {}
            }
        }

        (sort_field, sort_asc, status)
    }
}

impl Source for SuwayomiSource {
    fn new() -> Self {
        Self
    }

    fn get_search_manga_list(
        &self,
        query: Option<String>,
        page: i32,
        filters: Vec<FilterValue>,
    ) -> Result<MangaPageResult> {
        let base_url = self.server_url()?;
        let username = self.username();
        let password = self.password();

        let (sort_field, sort_asc, status) = Self::parse_filters(&filters);

        let data = graphql::fetch_mangas(
            &base_url,
            &username,
            &password,
            query.as_deref(),
            Some(&status),
            sort_field,
            sort_asc,
            page,
        )?;

        let entries: Vec<Manga> = data
            .mangas
            .nodes
            .iter()
            .map(|n| helpers::manga_from_node(n, &base_url))
            .collect();

        let has_next_page = (page * PAGE_SIZE) < data.mangas.total_count;

        Ok(MangaPageResult { entries, has_next_page })
    }

    fn get_manga_update(
        &self,
        mut manga: Manga,
        needs_details: bool,
        needs_chapters: bool,
    ) -> Result<Manga> {
        let base_url = self.server_url()?;
        let username = self.username();
        let password = self.password();

        let manga_id: i32 = manga.key.parse().map_err(|_| aidoku::AidokuError::Message(aidoku::alloc::String::from("Invalid ID")))?;

        if needs_details {
            let data = graphql::fetch_manga(&base_url, &username, &password, manga_id)?;
            let updated = helpers::manga_from_node(&data.manga, &base_url);
            manga.copy_from(updated);
        }

        if needs_chapters {
            let data = graphql::fetch_chapters(&base_url, &username, &password, manga_id)?;
            let chapters: Vec<Chapter> = data
                .chapters
                .nodes
                .iter()
                .map(helpers::chapter_from_node)
                .collect();
            manga.chapters = Some(chapters);
        }

        Ok(manga)
    }

    fn get_page_list(&self, _manga: Manga, chapter: Chapter) -> Result<Vec<Page>> {
        let base_url = self.server_url()?;
        let username = self.username();
        let password = self.password();

        let chapter_id: i32 = chapter
            .key
            .parse()
            .map_err(|_| aidoku::AidokuError::Message(aidoku::alloc::String::from("Invalid ID")))?;

        let data = graphql::fetch_chapter_pages(&base_url, &username, &password, chapter_id)?;

        Ok(helpers::pages_from_urls(&data.fetch_chapter_pages.pages, &base_url))
    }
}

impl ListingProvider for SuwayomiSource {
    fn get_manga_list(&self, listing: Listing, page: i32) -> Result<MangaPageResult> {
        let base_url = self.server_url()?;
        let username = self.username();
        let password = self.password();

        if listing.id.starts_with("category:") {
            // Category listing: "category:42"
            let cat_id: i32 = listing.id["category:".len()..]
                .parse()
                .map_err(|_| aidoku::AidokuError::Message(aidoku::alloc::String::from("Invalid ID")))?;
            let data = graphql::fetch_mangas_by_category(
                &base_url, &username, &password, cat_id, page,
            )?;
            let entries: Vec<Manga> = data
                .mangas
                .nodes
                .iter()
                .map(|n| helpers::manga_from_node(n, &base_url))
                .collect();
            let has_next_page = (page * PAGE_SIZE) < data.mangas.total_count;
            return Ok(MangaPageResult { entries, has_next_page });
        }

        match listing.id.as_str() {
            "library" => {
                let data = graphql::fetch_mangas(
                    &base_url,
                    &username,
                    &password,
                    None,
                    None,
                    SORT_IN_LIBRARY_AT,
                    false,
                    page,
                )?;
                let entries: Vec<Manga> = data
                    .mangas
                    .nodes
                    .iter()
                    .map(|n| helpers::manga_from_node(n, &base_url))
                    .collect();
                let has_next_page = (page * PAGE_SIZE) < data.mangas.total_count;
                Ok(MangaPageResult { entries, has_next_page })
            }
            "recent" => {
                let data = graphql::fetch_recent_chapters(&base_url, &username, &password, page)?;
                // Deduplicate manga by id — show the manga entry once per recent chapter
                let entries: Vec<Manga> = data
                    .chapters
                    .nodes
                    .iter()
                    .map(|c| helpers::manga_from_node(&c.manga, &base_url))
                    .collect();
                // recent chapters are always limited to one page of 20
                Ok(MangaPageResult { entries, has_next_page: false })
            }
            _ => bail!("Unknown listing: {}", listing.id),
        }
    }
}

impl Home for SuwayomiSource {
    fn get_home(&self) -> Result<HomeLayout> {
        let base_url = self.server_url()?;
        let username = self.username();
        let password = self.password();

        // Recently updated chapters section
        let recent_data =
            graphql::fetch_recent_chapters(&base_url, &username, &password, 1)?;

        let recent_entries: Vec<MangaWithChapter> = recent_data
            .chapters
            .nodes
            .iter()
            .map(|c| MangaWithChapter {
                manga: helpers::manga_from_node(&c.manga, &base_url),
                chapter: helpers::chapter_from_node_recent(c),
            })
            .collect();

        // Library overview section
        let library_data = graphql::fetch_mangas(
            &base_url,
            &username,
            &password,
            None,
            None,
            SORT_IN_LIBRARY_AT,
            false,
            1,
        )?;

        let library_entries: Vec<Manga> = library_data
            .mangas
            .nodes
            .iter()
            .map(|n| helpers::manga_from_node(n, &base_url))
            .collect();

        Ok(HomeLayout {
            components: Vec::from([
                HomeComponent {
                    title: Some(String::from("Recently Updated")),
                    subtitle: None,
                    value: HomeComponentValue::MangaChapterList {
                        page_size: None,
                        entries: recent_entries,
                        listing: Some(Listing {
                            id: String::from("recent"),
                            name: String::from("Recently Updated"),
                            kind: ListingKind::List,
                        }),
                    },
                },
                HomeComponent {
                    title: Some(String::from("Library")),
                    subtitle: None,
                    value: HomeComponentValue::Scroller {
                        entries: library_entries
                            .into_iter()
                            .map(|m| m.into())
                            .collect(),
                        listing: Some(Listing {
                            id: String::from("library"),
                            name: String::from("Library"),
                            kind: ListingKind::Default,
                        }),
                    },
                },
            ]),
        })
    }
}

impl DynamicFilters for SuwayomiSource {
    fn get_dynamic_filters(&self) -> Result<Vec<Filter>> {
        Ok(Vec::from([
            SortFilter {
                id: "sort".into(),
                title: Some("Sort By".into()),
                can_ascend: true,
                options: Vec::from(["Title".into(), "Last Updated".into(), "Date Added".into()]),
                default: Some(SortFilterDefault { index: 1, ascending: false }),
                ..Default::default()
            }
            .into(),
            SelectFilter {
                id: "status".into(),
                title: Some("Status".into()),
                options: Vec::from([
                    "Any".into(),
                    "Ongoing".into(),
                    "Completed".into(),
                    "Hiatus".into(),
                    "Cancelled".into(),
                ]),
                ..Default::default()
            }
            .into(),
        ]))
    }
}

impl DynamicSettings for SuwayomiSource {
    fn get_dynamic_settings(&self) -> Result<Vec<Setting>> {
        // Settings are defined statically in res/settings.json.
        // This implementation is a no-op but satisfies the trait.
        Ok(Vec::new())
    }
}

register_source!(SuwayomiSource, ListingProvider, Home, DynamicFilters, DynamicSettings);
