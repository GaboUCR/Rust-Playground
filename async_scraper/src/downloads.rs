use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::{thread, time};

pub struct Downloader {
    pub tx: Sender<String>,
    rx: Receiver<String>,
}

impl Downloader {
    pub fn new () -> Downloader {
        let (tx, rx) = channel::<String>();

        Downloader {
            tx,
            rx,
        }
    }
    pub fn handle_images(self) {

        loop {
            let link = self.rx.recv().unwrap();
            println!("Getting image from: {}", link);
            download_image(link.clone());
            sleep(4)
        }
    }
}

pub fn sleep(sec:u64){
    let ten_millis = time::Duration::from_millis(sec*1000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);
}


pub fn create_file(bytes:Vec<u8>, link:String){

    let mut new_file_path = String::from("images");
    let name = &link[link.rfind("/").unwrap()..link.len()].to_string();
    new_file_path.push_str(name);

    let path = Path::new(&new_file_path);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => println!("couldn't create {}: {}", display, why),
        Ok(mut file) => {
            match file.write_all(&bytes) {
                Err(why) => println!("couldn't write to {}: {}", display, why),
                Ok(_) => println!("successfully wrote to {}", display),
            };
        },
    };
}

pub fn download_image(link:String) -> Result<(),  ureq::Error> {
    let resp = ureq::get(&link).call()?;

    let mut bytes: Vec<u8> = Vec::with_capacity(5000000);
    resp.into_reader().take(5_000_000).read_to_end(&mut bytes)?;

    create_file(bytes, link);
    Ok(())
}
