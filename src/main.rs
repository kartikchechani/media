

mod content;

use content::media::Media;
use content::catalog::Catalog;





fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("The Rust Programming Language")
    };

    let book = Media::Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols")
    };

    let movie = Media::Movie {
        title: String::from("Inception"),
        director: String::from("Christopher Nolan")
    };

    let podcast = Media::Podcast(20);
    let placeholder = Media::Placeholder;

    // println!("{}", audiobook.description());
    // println!("{}", book.description());
    // println!("{}", movie.description());

    let mut catalog = Catalog::new();

    catalog.add_item(audiobook);
    catalog.add_item(book);
    catalog.add_item(movie);
    catalog.add_item(podcast);
    catalog.add_item(placeholder);
    

    match catalog.get_by_index(2) {
        Some(item) => {
            println!("Item {:#?}", item)
        }
        None => {
            println!("Item not found")
        }
    }



}
