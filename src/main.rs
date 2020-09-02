#[macro_use]
extern crate clap;
extern crate mime;
extern crate reqwest;

use dynfmt::{Format, SimpleCurlyFormat};

mod mangadex_api;
// mod h4x;

use mime::Mime;
use reqwest::header::{CONTENT_TYPE, CONNECTION, ACCEPT, USER_AGENT, REFERER}; // COOKIE 4 poor cases
use reqwest::header::HeaderMap;
use clap::App;
use std::fs;
use std::fs::File;
use std::io; // Was thread(4 offthreading file ops) and time(not needed anymo)


fn main() -> Result<(), reqwest::Error> { // We have Result type with relevant Error alt. U can use '?' operator, if rustc taunts u about doing unwrap() - do mem::transmute to normal type of your meth output.
    // command line arguments
    let yaml = load_yaml!("cli.yml");
    let args = App::from_yaml(yaml).get_matches();

    if args.is_present("chapter") && args.is_present("volume") {
        println!("Both chapter and volume cannot be used at the same time");
        std::process::exit(1);
    }

    let mut map = HeaderMap::with_capacity(4); // Number of herders in a default header map. Btw, put ur headers, common 4 API & image requests, here.
    map.insert(CONNECTION, reqwest::header::HeaderValue::from_static("keep-alive")); // Use same connection 4 entire chapter. Maybe will work with their API(comment this out if it complains bout FRAME_SIZE blah).
    map.insert(USER_AGENT, reqwest::header::HeaderValue::from_static("Brave (like Chrome, but better)")); // Brave is better than Chrome, just 2! skip initial configuratio or ull be stuck with Bitcoin scam backgrounds and Google instead of search engine.
    map.insert(REFERER, reqwest::header::HeaderValue::from_static("https://mangadex.org/user/2")); // Зе-мля ему стекловатой. Change it, when their xcuse of admin will notice immut referer 4 100+ series.
    map.insert("X-Requested-With", reqwest::header::HeaderValue::from_static("XMLHttpRequest")); // They will fink it's Google client 4 clearnet.
    let client = reqwest::blocking::Client::builder().user_agent("mangadex-full-api").cookie_store(false).referer(false) // Masking 4 their API and disabling COOKIE and REFERER headers autogen(will be manual).
    .default_headers(map).build()?; // Wild specimen of '?' operator. Hold on to it, mem::transmute 2 left type in Result<T, E> if Rustc will complain about line with this.

    if args.is_present("chapter") {
        let chapter_data = mangadex_api::get_chapter_data(&client, args.value_of("id").unwrap());
        let manga_data = mangadex_api::get_manga_data(&client, &chapter_data.manga_id.to_string());
        let data = manga_data
            .chapter
            .get(&chapter_data.id.to_string())
            .unwrap();
        println!(
            "Scraping '{} Vol. {} Ch. {} in {} from {}'",
            manga_data.manga.title, data.volume, data.chapter, data.lang_code, data.group_name
        );
        download_chapter(&client, chapter_data.id.to_string(), data, &manga_data);
    } else if args.is_present("volume") {
        let volume = args.value_of("volume").unwrap();
        let manga_data = mangadex_api::get_manga_data(&client, args.value_of("id").unwrap());
        println!("Scraping '{} Vol. {}'", manga_data.manga.title, volume);
        for (name, data) in &manga_data.chapter {
            if data.volume != volume {
                continue;
            }
            if args.is_present("lang") {
                if data.lang_code != args.value_of("lang").unwrap() {
                    continue;
                }
            }

            download_chapter(&client, name.to_string(), &data, &manga_data);
        }
    } else {
        let manga_data = mangadex_api::get_manga_data(&client, args.value_of("id").unwrap());


        println!(
            "Scraping '{}' in {}",
            manga_data.manga.title,
            if !args.is_present("lang") {
                "All"
            } else {
                args.value_of("lang").unwrap()
            }
        );

        for (name, data) in &manga_data.chapter {
            if args.is_present("lang") {
                if data.lang_code != args.value_of("lang").unwrap() {
                    continue;
                }
            }

            download_chapter(&client, name.to_string(), data, &manga_data);
        }
    }
    Ok(())
}

#[inline(always)]
fn pagenum_extractor(page : String, _mime : Mime) -> String // BRITTLE
{
    // let ext : String = match (sm_club.type_(), mime.subtype()) {
    //               (mime::IMAGE, mime::PNG)  => ".png",
    //              (mime::IMAGE, mime::GIF)    => ".gif",
    //              (mime::IMAGE, mime::JPEG) => ".jpg",
    //              (_, _)                                         => ".svg", // because it is visible only by extension. MIME sniffing doesn't work rel on it.
    //            }; // Replace by .enumerate() iter(that's with page in it) and whatever. Problem is that serde'd page_array is unsorted, "but it is".
    page.clone().chars().skip(1).take_while(|&c| c != '-').chain(extension_bzn(page).chars()).collect()
}

#[inline(always)]
fn extension_bzn(page : String) -> String
{
  page.chars().take_while(|&c| c != '.').collect()
}

#[inline(always)]
fn extension(page : String) -> Mime {
  match extension_bzn(page).as_str() {
        ".png"               => "image/png".parse::<Mime>().expect("Thatstwatihavesaid"),
        ".gif"                  => "image/gif".parse::<Mime>().expect("Thatstwatihavesaid"),
        ".jpg" | ".jpeg" => "image/jpeg".parse::<Mime>().expect("Thatstwatihavesaid"),
        ".svg"                => "image/svg+xml".parse::<Mime>().expect("Thatstwatihavesaid"),
        _                       => "application/json".parse::<Mime>().expect("Thatstwatihavesaid"),
      }
}

fn strip_characters(original: &str, to_strip: &str) -> String {
    original
        .chars()
        .filter(|&c| !to_strip.contains(c))
        .collect()
}

fn download_chapter(
    client: &reqwest::blocking::Client,
    name: String,
    data: &mangadex_api::Chapter,
    manga_data: &mangadex_api::MangaData,
) {
    let chapter_data = mangadex_api::get_chapter_data(&client, &name);

    for page in chapter_data.page_array { // .iter().enumerate() and (num, page) 4 sorted array. Fix serde inst of sort.
        let page_name = SimpleCurlyFormat.format("{:0>8}", &[pagenum_extractor(page.clone(), extension(page.clone()))]).expect("something?").to_string(); // Consume .extension -> strict MIME-like enum(PNG, JPG, GIF). Then collect forward stride after '-'.
        // Late MIME sniff. Tends 2b buggy, so use contracts and ACCEPT. Note: can recog extension 4m page_array string tailmatch.

        let url = if chapter_data.server == "/data/" { // Old 4mat. Newer MDNet 4mat has a long string b4 hash
            reqwest::Url::parse(&*format!(
                "https://mangadex.org/data/{}/{}",
                chapter_data.hash, page
            ))
            .unwrap()
        } else {
            reqwest::Url::parse(&*format!( // Yep, I like it.
                "{}{}/{}",
                chapter_data.server, chapter_data.hash, page
            ))
            .unwrap()
        };
        //println!("downloading {}", &url);
        let mut resp = client.get(url).header(ACCEPT, "image/apng,image/*").send().unwrap();
        let headers = resp.headers_mut();
        let _ = headers.get(CONTENT_TYPE).expect("scraptor 2 die horribly if MD servo got the weed in the root shell");
        // let mut mime_container : Mime = "application/json".parse::<Mime>().expect("Just bring ur Options; how can this ever fail?");
        // mime_container = match baka1 {
        //  Some(mime) => mime.clone().to_str().parse::<Mime>().expect("mime parser 22 it`s humble work"),
        //  None => extension(page.clone()),
        //};
        if cfg!(debug_assertions) { println!("Got CONTENT_TYPE : {}", extension_bzn(page.clone()) as String); }


        fs::create_dir_all(strip_characters(
            &*format!(
                "{} Vol. {} Ch. {} - {} ({})",
                manga_data.manga.title,
                format!("{:0>4}", data.volume),
                format!("{:0>4}", data.chapter),
                data.group_name,
                data.lang_code
            ),
            "/",
        ))
        .unwrap();
        let mut out = File::create(
            std::path::Path::new(&*strip_characters(
                &*format!(
                    "{} Vol. {} Ch. {} - {} ({})",
                    manga_data.manga.title,
                    format!("{:0>4}", data.volume),
                    format!("{:0>4}", data.chapter),
                    data.group_name,
                    data.lang_code,
                ),
                "/",
            ))
            .join(&page_name),
        )
        .expect("failure to create image");
        let _copy = io::copy(&mut resp, &mut out);
        let _copy = match _copy {
            Ok(file) => file,
            Err(error) => {
                println!("Error Copying to File, trying again: {}", error);
                std::fs::remove_file(
                    std::path::Path::new(&*strip_characters(
                        &*format!(
                            "{} Vol. {} Ch. {} - {} ({})",
                            manga_data.manga.title,
                            format!("{:0>4}", data.volume),
                            format!("{:0>4}", data.chapter),
                            data.group_name,
                            data.lang_code,
                        ),
                        "/",
                    ))
                    .join(&page_name),
                )
                .unwrap();
                let url = if chapter_data.server == "/data/" {
                    reqwest::Url::parse(&*format!(
                        "https://mangadex.org/data/{}/{}",
                        chapter_data.hash, page
                    ))
                    .unwrap()
                } else {
                    reqwest::Url::parse(&*format!(
                        "{}{}/{}",
                        chapter_data.server, chapter_data.hash, page
                    ))
                    .unwrap()
                };
                //println!("downloading {}", &url);
               let mut resp = client.get(url).header(ACCEPT, "image/apng,image/*").send().unwrap();

                fs::create_dir_all(strip_characters(
                    &*format!(
                        "{} Vol. {} Ch. {} - {} ({})",
                        manga_data.manga.title,
                        format!("{:0>4}", data.volume),
                        format!("{:0>4}", data.chapter),
                        data.group_name,
                        data.lang_code
                    ),
                    "/",
                ))
                .unwrap();
                let mut out = File::create(
                    std::path::Path::new(&*strip_characters(
                        &*format!(
                            "{} Vol. {} Ch. {} - {} ({})",
                            manga_data.manga.title,
                            format!("{:0>4}", data.volume),
                            format!("{:0>4}", data.chapter),
                            data.group_name,
                            data.lang_code,
                        ),
                        "/",
                    ))
                    .join(&page_name),
                )
                .expect("failure to create image");
                io::copy(&mut resp, &mut out).expect("failure to copy to image a second time");
                0
            }
        };
    }

    println!(
        "Downloaded '{} Vol. {} Ch. {} in {} from {}'",
        manga_data.manga.title, data.volume, data.chapter, data.lang_code, data.group_name
    );
}
