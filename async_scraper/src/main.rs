use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use scraper::{Html, Selector};
use url::{Url, Host, Position};

fn create_file(bytes:Vec<u8>, link:String){

    let mut new_file_path = String::from("images");
    let name = &link[link.rfind("/").unwrap()..link.len()].to_string();
    new_file_path.push_str(name);

    let path = Path::new(&new_file_path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(&bytes) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    };
}

fn download_image(link:String) -> Result<(),  ureq::Error> {
    let resp = ureq::get(&link).call()?;

    let mut bytes: Vec<u8> = Vec::with_capacity(5000000);
    resp.into_reader().take(5_000_000).read_to_end(&mut bytes)?;

    create_file(bytes, link);
    Ok(())
}

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
                links.push(x.to_string());
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
    // download_image("https://upload.wikimedia.org/wikipedia/commons/thumb/5/5b/The_Felidae.jpg/245px-The_Felidae.jpg".to_string());
    let t = get_links("https://es.wikipedia.org/wiki/Felis_silvestris_catus".to_string());
    println!("{:?}", t.unwrap()[0]);
}
