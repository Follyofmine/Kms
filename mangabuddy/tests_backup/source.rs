use aidoku::{Manga, MangaStatus, ContentRating, Chapter, Page};
use aidoku::alloc::{String, vec};
use crate::mangabuddy;

#[test]
fn test_search_manga_list() {
    let source = mangabuddy;
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
    let source = mangabuddy;
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
    let source = mangabuddy;
    let result = source.get_page_list(manga, chapter);
    assert!(result.is_ok(), "get_page_list should return Ok");
    let pages = result.unwrap();
    assert_eq!(pages.len(), 2);
    // The Page struct may not have a direct 'url' field; check content if needed
}
