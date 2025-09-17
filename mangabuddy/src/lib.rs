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

#[cfg(test)]
mod tests {
	use super::*;
	use aidoku::{Manga, MangaStatus, ContentRating, Chapter};
	use aidoku::alloc::{String, vec};

	#[test]
	fn test_search_manga_list() {
		let source = Mangabuddy;
		let result = source.get_search_manga_list(None, 1, vec![]);
		assert!(result.is_ok(), "get_search_manga_list should return Ok");
		let page = result.unwrap();
		assert!(!page.entries.is_empty(), "Should return at least one manga");
		assert_eq!(page.entries[0].title, "Dummy Manga");
	}

	#[test]
	fn test_get_manga_update() {
		let manga = Manga {
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
		};
		let source = Mangabuddy;
		let result = source.get_manga_update(manga.clone(), true, false);
		assert!(result.is_ok(), "get_manga_update should return Ok");
		let updated = result.unwrap();
		assert_eq!(updated.description, Some(String::from("Updated description for dummy manga.")));
	}

	#[test]
	fn test_get_page_list() {
		let manga = Manga {
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
		};
		let chapter = Chapter {
			key: String::from("dummy-chapter"),
			title: Some(String::from("Chapter 1")),
			url: Some(String::from("/manga/dummy/chapter-1")),
			chapter_number: Some(1.0),
			volume_number: None,
			date_uploaded: Some(0),
			scanlators: Some(vec![String::from("Test")]),
			..Default::default()
		};
		let source = Mangabuddy;
		let result = source.get_page_list(manga, chapter);
		assert!(result.is_ok(), "get_page_list should return Ok");
		let pages = result.unwrap();
		assert_eq!(pages.len(), 2);
		// The Page struct may not have a direct 'url' field; check content if needed
	}
}

