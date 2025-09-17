
#![no_std]
use aidoku::prelude::*;
use aidoku::{
    Chapter, DeepLinkHandler, DeepLinkResult, FilterValue, Home, HomeLayout, Listing,
    ListingProvider, Manga, MangaPageResult, Page, Result, Source, MangaStatus, ContentRating,
    alloc::{String, Vec, vec},
    register_source,
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
        mut manga: Manga,
        needs_details: bool,
        _needs_chapters: bool,
    ) -> Result<Manga> {
        if needs_details {
            manga.description = Some(String::from("Updated description for dummy manga."));
        }
        Ok(manga)
    }

    fn get_page_list(&self, _manga: Manga, _chapter: Chapter) -> Result<Vec<Page>> {
        Ok(vec![
            Page {
                content: aidoku::PageContent::Text(String::from("https://mangabuddy.com/page1.jpg")),
                ..Default::default()
            },
            Page {
                content: aidoku::PageContent::Text(String::from("https://mangabuddy.com/page2.jpg")),
                ..Default::default()
            }
        ])
    }
}

impl ListingProvider for Mangabuddy {
    fn get_manga_list(&self, _listing: Listing, _page: i32) -> Result<MangaPageResult> {
        self.get_search_manga_list(Some(String::from("")), 1, vec![])
    }
}

impl Home for Mangabuddy {
    fn get_home(&self) -> Result<HomeLayout> {
        Ok(HomeLayout { components: vec![] })
    }
}

impl DeepLinkHandler for Mangabuddy {
    fn handle_deep_link(&self, _url: String) -> Result<Option<DeepLinkResult>> {
        Ok(None)
    }
}

register_source!(Mangabuddy, ListingProvider, Home, DeepLinkHandler);

