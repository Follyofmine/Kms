// no_std is set at the crate root (lib.rs)
use aidoku::prelude::*;
use aidoku::{
    alloc::{String, Vec, vec},
    Manga, MangaPageResult, FilterValue, Result, Source, Chapter, Page,
    ContentRating, MangaStatus
};

pub struct Mangabuddy;

impl Source for Mangabuddy {
    fn new() -> Self {
        Self
    }

    fn get_search_manga_list(
        &self,
        _query: Option<String>,
        _page: i32,
        _filters: Vec<FilterValue>,
    ) -> Result<MangaPageResult> {
        Ok(MangaPageResult {
            entries: vec![Manga {
                key: String::from("en.mangabuddy.dummy"),
                cover: Some(String::from("https://mangabuddy.com/cover.jpg")),
                title: String::from("Dummy Manga"),
                authors: Some(vec![String::from("Follyofmine")]),
                artists: Some(vec![String::from("Follyofmine")]),
                tags: Some(vec![String::from("Test")]),
                content_rating: ContentRating::Safe,
                status: MangaStatus::Ongoing,
                description: Some(String::from("This is a test manga for Aidoku.")),
                url: Some(String::from("/manga/dummy")),
                ..Default::default()
            }],
            has_next_page: false,
        })
    }

    fn get_manga_update(
        &self,
        manga: Manga,
        _needs_details: bool,
        _needs_chapters: bool,
    ) -> Result<Manga> {
        Ok(manga)
    }

    fn get_page_list(&self, _manga: Manga, _chapter: Chapter) -> Result<Vec<Page>> {
        Ok(vec![])
    }
}