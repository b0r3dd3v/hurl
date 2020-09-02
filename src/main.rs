                    reqwest::Url::parse(&*format!(
489
                        "https://mangadex.org/data/{}/{}",
490
                        chapter_data.hash, page
491
                    ))
492
                    .unwrap()
493
                } else {
494
                    reqwest::Url::parse(&*format!(
495
                        "{}{}/{}",
496
                        chapter_data.server, chapter_data.hash, page
497
                    ))
498
                    .unwrap()
499
                };
500
                //println!("downloading {}", &url);
501
               let mut resp = client.get(url).header(ACCEPT, "image/apng,image/*").send().unwrap();
502
​
503
                fs::create_dir_all(strip_characters(
504
                    &*format!(
505
                        "{} Vol. {} Ch. {} - {} ({})",
506
                        manga_data.manga.title,
507
                        format!("{:0>4}", data.volume),
508
                        format!("{:0>4}", data.chapter),
509
                        data.group_name,
510
                        data.lang_code
511
                    ),
512
                    "/",
513
                ))
514
                .unwrap();
515
                let mut out = File::create(
516
                    std::path::Path::new(&*strip_characters(
517
                        &*format!(
518
                            "{} Vol. {} Ch. {} - {} ({})",
519
                            manga_data.manga.title,
520
                            format!("{:0>4}", data.volume),
521
                            format!("{:0>4}", data.chapter),
522
                            data.group_name,
523
                            data.lang_code,
524
                        ),
525
                        "/",
526
                    ))
527
                    .join(&page_name),
528
                )
529
                .expect("failure to create image");
530
                io::copy(&mut resp, &mut out).expect("failure to copy to image a second time");
531
                0
532
            }
533
        };
534
    }
535
​
536
    println!(
537
        "Downloaded '{} Vol. {} Ch. {} in {} from {}'",
538
        manga_data.manga.title, data.volume, data.chapter, data.lang_code, data.group_name
539
    );
540
}
541
