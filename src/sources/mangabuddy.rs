#![no_std]
use aidoku::{
    AidokuError, Chapter, DeepLinkHandler, DeepLinkResult, FilterValue, Home, HomeLayout, Listing,
    ListingProvider, Manga, MangaPageResult, Page, Result, Source,
    alloc::{String, Vec},
    prelude::*,
};

pub struct mangabuddy;

impl Source for mangabuddy {
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
            manga: vec![Manga {
                id: String::from("dummy-1"),
                cover: String::from("https://mangabuddy.com/cover.jpg"),
                title: String::from("Dummy Manga"),
                author: String::from("Follyofmine"),
                artist: String::from("Follyofmine"),
                description: String::from("This is a test manga for Aidoku."),
                url: String::from("/manga/dummy-1"),
                categories: vec![String::from("Test")],
                status: 1,
                nsfw: false,
            }],
            has_more: false,
        })
    }

    fn get_manga_update(
        &self,
        manga: Manga,
        needs_details: bool,
        needs_chapters: bool,
    ) -> Result<Manga> {
        let mut updated = manga;
        if needs_details {
            updated.description = String::from("Updated description for dummy manga.");
        }
        if needs_chapters {
            // Normally you would fetch chapters here
        }
        Ok(updated)
    }

    fn get_page_list(&self, _manga: Manga, _chapter: Chapter) -> Result<Vec<Page>> {
        Ok(vec![Page {
            index: 0,
            url: String::from("https://mangabuddy.com/page1.jpg"),
        },
        Page {
            index: 1,
            url: String::from("https://mangabuddy.com/page2.jpg"),
        }])
    }
}

impl ListingProvider for mangabuddy {
    fn get_manga_list(&self, _listing: Listing, _page: i32) -> Result<MangaPageResult> {
        self.get_search_manga_list(Some(String::from("")), 1, vec![])
    }
}

impl Home for mangabuddy {
    fn get_home(&self) -> Result<HomeLayout> {
        Ok(HomeLayout {
            sections: vec![],
        })
    }
}

impl DeepLinkHandler for mangabuddy {
    fn handle_deep_link(&self, _url: String) -> Result<Option<DeepLinkResult>> {
        Ok(None)
    }
}

register_source!(mangabuddy, ListingProvider, Home, DeepLinkHandler);