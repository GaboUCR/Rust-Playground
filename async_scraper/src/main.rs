use std::io::Read;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

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
    resp.into_reader()
        .take(5_000_000)
        .read_to_end(&mut bytes)?;

    create_file(bytes, link);
    Ok(())
}

fn main()  {
    download_image("https://upload.wikimedia.org/wikipedia/commons/thumb/5/5b/The_Felidae.jpg/245px-The_Felidae.jpg".to_string());

}
