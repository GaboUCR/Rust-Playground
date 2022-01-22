use scraper::{Html, Selector};
use url::{Url, Host, Position};
use std::{thread, time};

mod downloads;
use downloads::{create_file, download_image, Downloader, sleep};


fn get_links(link:String) -> Result<Vec<Vec<String>>, ureq::Error> {

    let mut links:Vec<String> = Vec::new();
    let parse = Url::parse(&link)?;

    let domain = format!("{}://{}", parse.scheme(), parse.host_str().unwrap()) ;

    let html: String = ureq::get(&link).call()?.into_string()?;
    let document = Html::parse_document(&html);
    let mut selector = Selector::parse("a").unwrap();

    // Get the links
    for element in document.select(&selector) {

        match element.value().attr("href") {
            Some(x) => {
                let first_char = x.chars().next().unwrap();

                if first_char == "/".chars().next().unwrap() {
                    links.push(format!("{}{}", domain, x));
                    continue;
                }
                if x.contains("wikipedia") {
                    links.push(x.to_string());
                }
            },
            None => continue,
        }
    }
    selector = Selector::parse("img").unwrap();
    let mut img_links:Vec<String> = Vec::new();
    // Get the images
    for element in document.select(&selector) {

        match element.value().attr("src") {
            Some(x) => {
                let first_char = x.chars().next().unwrap();

                if first_char == "/".chars().next().unwrap() {
                    img_links.push(format!("{}:{}", parse.scheme(), x));
                    continue;
                }
                img_links.push(x.to_string());
            },
            None => continue,
        }
    }

    Ok((Vec::from([links, img_links])))
}

fn main()  {

    let downloader1 = Downloader::new();
    let downloader2 = Downloader::new();

    let tx1 = downloader1.tx.clone();
    let tx2 = downloader2.tx.clone();

    thread::spawn(move || downloader1.handle_images());
    thread::spawn(move || downloader2.handle_images());

    let mut links:Vec<String> = Vec::from(["https://es.wikipedia.org/wiki/Felis_silvestris_catus".to_string()]);
    let mut img_links:Vec<String> = Vec::from([]);
    let mut scraped:Vec<String> = Vec::from([]);

    loop{
        let link = links.remove(0);
        let mut skip = false;

        for e in &scraped {
            if e == &link {
                skip = true;
                continue;
            }
        }

        if skip {
            continue;
        }
        println!("scraping {}", link);

        let req = get_links(link.clone());
        scraped.push(link);

        for e in &req.as_ref().unwrap()[0] {
            links.push(e.to_string());

        }

        for e in &req.unwrap()[1] {
            img_links.push(e.to_string());
        }

        loop {
            let l = img_links.remove(0);
            let mut is_in = true;

            for e in &scraped {
                if e == &l {
                    is_in = false;
                    break;
                }
            }
            if is_in {
                tx1.send(l).unwrap();
                break;
            }

        }
        loop {
            let l = img_links.remove(0);
            let mut is_in = true;

            for e in &scraped {
                if e == &l {
                    is_in = false;
                    break;
                }
            }
            if is_in {
                tx2.send(l).unwrap();
                break;
            }

        }
        sleep(5);
    }
}
