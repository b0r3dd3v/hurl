extern crate reqwest;
extern crate serde;

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Manga {
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct Chapter {
    pub volume: String,
    pub chapter: String,
    pub lang_code: String,
    pub group_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MangaData {
    pub manga: Manga,
    pub chapter: HashMap<String, Chapter>,
}

#[derive(Debug, Deserialize)]
pub struct ChapterData {
    pub id: u32,
    pub hash: String,
    pub manga_id: u32,
    pub server: String,
    pub page_array: Vec<String>,
}

// https://mangadex.org/api/?id=42035&type=manga

pub fn get_manga_data(client: &reqwest::blocking::Client, manga: &str) -> MangaData {
    let base_url = reqwest::Url::parse("https://mangadex.org/api/").unwrap();
    // let url = base_url.join(manga).unwrap();

    let json: MangaData = client.get(base_url).query(&[("id", manga), ("type", "manga")]).send().unwrap().json().unwrap();
    json
}

// https://mangadex.org/api/?id=725671&server=na&saver=0&type=chapter
// And change file naming 2 enumerate.

pub fn get_chapter_data(client: &reqwest::blocking::Client, chapter: &str) -> ChapterData {
    let base_url = reqwest::Url::parse("https://mangadex.org/api/").unwrap();
    // let url = base_url.join(chapter).unwrap();

    let json: ChapterData = client
        .get(base_url).query(&[("id", chapter), ("server", "na"), ("saver", "0"), ("type", "chapter")])
        .send()
        .expect("something went wrong with sending")
        .json()
        .expect("something went wrong with json parsing");
    json
}
