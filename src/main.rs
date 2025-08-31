use std::collections::btree_map::Values;



#[derive(Debug)]
enum Media {
    Book {  title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast ( u32 ),
    Placeholder


}


impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book {title, author} => {
                format!("Book - Title: {}, Author: {}", title, author)
            },
            Media::Movie {title, director} => {
                format!("Movie - Title: {}, Director: {}", title, director)
            },
            Media::Audiobook {title} => {
                format!("Audiobook - Title: {}", title)
            },
            Media::Podcast(episode) => {
                format!("Podcast - Episode: {}", episode)
            },
            Media::Placeholder => {
                String::from("Placeholder Media")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>
}

enum ValusPresentOrNot<'a> {
    ValuePresent (&'a Media),
    ValueNotPresent
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: vec![]}
    }

    fn add_item(&mut self, media: Media) {
        self.items.push(media);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {

        if(self.items.len() > index) {
            Some(&self.items[index])
        }else {
            None
        }
    }
}


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
    

    let item = catalog.get_by_index(200);

    println!("{:#?}", item.unwrap());


}
