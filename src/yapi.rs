extern crate reqwest;
extern crate serde;

use serde::Deserialize;
use std::collections::HashMap;

// Пих себе - доступ во все поля старых типов устранить... с особой жестокостью. Буфера для главы, JSON расфасовать по SoA коллекции(v)
// Serialize entire chapter data fields. Pack hash, strings should be dedup.
// ChapterData: Hash, language(ext), pages[_array], server. volume(ext)
// MangaData: mainCover, tags(index mangas, dl mainCovers).

// Left it 4 comparison. Iterating over JSON fields will be hard wo rewrite.

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

// Наркомат:

#[derive(Debug, Deserialize)]
pub struct MangaData {
    pub manga: Manga,
    pub chapter: HashMap<String, Chapter>,
}

pub struct Chapter {
code: i32, // Useless
data: ChapterData,
status: String,
}

pub struct Manga {
code: i32, // Useless
data: ChapterData,
status: String,
}

pub struct GroupTuple { // Skip this struct. Still there is dead code.
    id: i32,
    name: String
}

pub struct LinksStruct {
    al: String,
    amz: String,
    ap: String,
    bw: String,
    ebj: String,
    engtl: String,
    kt: String,
    mal: String,
    mu: String
}

pub struct ChapterData {
    #[serde(skip)]
    chapter: i32, // Chapter num. Useless.
    #[serde(skip)]
    comments: i32, // Shitposts num. Useless.
    #[serde(skip)]
    groups: Vec<GroupTuple>, // Ded kode ensues. No Derive of GroupTuple type.
    hash: String, // Well, it is used in String builder, so leave it immut until use.           11111111111111111111111111111111111111111111
    #[serde(skip)]
    id: i32, // In case if you dindu know wut r u reqwesting.
    language: String, // Convert it 2 Enum and get rid of that allocation stuff. 11111111111111111111111111111
    mangaId: i32, // Same.
    mangaTitle: String, // Same 4 alt-heimers
    pages: Vec<String>, // Handy.                                                                         11111111111111111111111111111111
    server: String, // Check 4 bing not a data saver, but orig mangadex or mdnet. 111111111111111111111111111111111111
    #[serde(skip)]
    status: String
    #[serde(skip)],
    timestamp: i64,
    #[serde(skip)]
    title:String,
    #[serde(skip)]
    uploader: i32,
    #[serde(skip)]
    views: i32,
    volume: String, // Used in String builder 4 filtering. 1111111111111111111111111111
}

pub struct MangaData {
    altTitles: Vec<String>,
    artist: String,
    author: String
    #[serde(skip)],
    comments: i32,
    #[serde(skip)]
    description: String,
    #[serde(skip)]
    follows: i32,
    #[serde(skip)]
    id: i32,
    #[serde(skip)]
    isHentai: bool, // Lemme care of ur flyin butts.
    #[serde(skip)]
    lastChapter: !,
    #[serde(skip)]
    lastUploaded: i64,
    #[serde(skip)]
    lastVolume: !,
    #[serde(skip)]
    links: LinksStruct,
    mainCover: String, //                                            1111111111111111111111111111111111111111111
    #[serde(skip)]
    publication : Blication,
    #[serde(skip)]
    rating: Blication, // Gold out of the ass.
    #[serde(skip)]
    relations: Vec<Elations>,
    tags: Vec<i32>, //                                                     111111111111111111111111111111111111111111111
    title: String,
    #[serde(skip)]
    views: i32,
}

pub struct Elations {
    id: i32,
    isHentai: bool,
    title: String,
    #[serde(rename = "type")]
    strain: i32, // New Thames rhabies virus. Gotta pwn yall.
}

// Crossdressing cooking gyarus.
pub struct Blication {
    demographic: i32,
    language: String,
    status: i32,
}

// Award winning teriyaki facepalm comedy.
pub struct Outofgold {
    bayesian: f32,
    mean: f32, // hidoi
    users: i32, //Миллионы mux
    
}

// Жестъ ябучая. Сколько срердёдок придётся ограничивать.


// Compare query API:
// https://mangadex.org/api/?id=42035&type=manga
// With yapi:
// https://mangadex.org/api/v2/manga/666?saver=0
// https://mangadex.org/api/v2/chapter/152530?saver=0

pub fn get_manga_data(client: &reqwest::blocking::Client, manga: &str) -> MangaData {
    let base_url = reqwest::Url::parse("https://mangadex.org/api/v2/{manga,chapter}").unwrap(); // Ну, вы подшейте его.
    // let url = base_url.join(manga).unwrap();

    // let json: MangaData = client.get(base_url).query(&[("id", manga), ("type", "{manga,chapter}")]).send().unwrap().json().unwrap(); Oyasumi, однострочник на фуллстыке.
    json // Бида. Поля во все поля, хотя нужен только инлайн гетман/сетман[/хитман].
}

// https://mangadex.org/api/?id=725671&server=na&saver=0&type=chapter
// And change file naming 2 enumerate. Охблъть, наступил в момент.

pub fn get_chapter_data(client: &reqwest::blocking::Client, chapter: &str) -> ChapterData {
    // let base_url = reqwest::Url::parse("https://mangadex.org/api/").unwrap(); // Чёрт, вшил бы эти юники, как u32 массив и притянул бы тип за уши.
    // let url = base_url.join(chapter).unwrap(); Сначала будем патчить древнекод... а потом - ещё древнее.

    let json: ChapterData = client
        .get(base_url) // .query(&[("id", chapter), ("server", "na"), ("saver", "0"), ("type", "chapter")]) // Вроде  как просто, только прокси нужен. А то дебилы на линке надорвутся.
        .send()
        .expect("something went wrong with sending")
        .json()
        .expect("something went wrong with json parsing");
    json
}

// Прошу ненавидеть и бояться.
unsafe fn roll_and_cover()

unsafe fn convert_tags()

#[inline(always)]
unsafe fn chuck_yapi_manga_json() // B4 generating state clone.

#[inline(always)]
unsafe fn chuck_yapi_chapter_json()

// JSON теперь придётся вытягивать у CDNа из задницы. Прокси нужен, чтобы хуелионом онанимных линков(без печенек + генератор паранойи) насытить 1024k-битный канал.
// Ныжен модуль для хостового ARQ, птшо поди разберись, где серву моча в голову ударила.
