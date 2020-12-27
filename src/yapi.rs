extern crate reqwest;
extern crate serde;

use serde::Deserialize;
use std::collections::HashMap;

// Пих себе - доступ во все поля старых типов устранить... с особой жестокостью. Буфера для главы, JSON расфасовать по SoA коллекции(v)
// Serialize entire chapter data fields. Pack hash, strings should be dedup.
// ChapterData: Hash, language(ext), pages[_array], server. volume(ext)
// MangaData: mainCover, tags(index mangas, dl mainCovers).

// Language has no likelies - steer manga JSON(after sorting & enum). Into lang caches.
let manga_scrape = try_json_manga_scrape(); // while loops 4 ~reliability, multipools 4 reqwest with proxies. Tries are run until Booing resulting stride is saturated(check number of zeroes wo constraining ranges).
let (ru_cache, cn_cache, en_cache) = alternate_lang(manga_scrape);

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
type Tusk = Vec<i64>; // больше StringBuilder не примет.
type Booing = [i128 ; 0x40000 ]; // 4M / 16b = 256k = 65536 x 4. 22: Trait with tuples. Assembly kernels 4 tracing.
type Bitmap = [i128 ; 0x40000];  // Convertible pools.

unsafe impl Booing for Booing { // Бой с хренью.
    #[inline(always)]
    fn trace_2(stride1: Booing, stride2: Booing) {} // VLRU load, trace 2 strides with 32/2 cache wo xlanes.
    #[inline(always)]
    fn trace_4(stride1: Booing, stride2: Booing, stride3: Booing, stride4: Booing) {} // 32/4 cache instances.
    #[inline(always)]
    fn trace_8(stride1: Booing, stride2: Booing, stride3: Booing, stride4: Booing, stride5: Booing, stride6: Booing, stride7: Booing, stride8: Booing) {} // 32/8.
    // Без генри, статическая раздача приключений.
}

unsafe impl Bitmap for Bitmap {
    fn ahobaka() -> i64 // Easy conversion 2 usize for BufSlices with usize arith(no uchar cz no JSON).
    { // NOT TMP; TEST TMP, TMP; IFNZ INC CTR. // CTR = 0, prefetch next, fetch VLRU, tile loop.
}

// Generate likelies on return(4M array), saturate state bitmap until ur done.
// Write retries 2 BufSlice 4 images(use their size limits 4 BufSlice limits) & write() files on caching(write state 2 Booing & Tusk likelies, off-thread disk writes).

#[inline(always)]
fn try_json_manga_scrape(q: Tusk) {} // 

#[inline(always)]
fn try_json_chapter_scrape(q: Tusk) //

#[inline(always)]
fn try_cover_scrape() {} //

fn try_chapter_scrape() {} //


// Booing Yatsude.
fn intersect_tags() {} //

fn intersect_tusk_likelies() {} //

// No mo reasons 4 single client(multithreaded design with many socket collections). Need hel of bots.
