use inline_colorization::*;

enum Publication<'a> {
    Book(Book<'a>),
    Magazine(Magazine<'a>),
}

struct Book<'a> {
    title: &'a str,
    author: &'a str,
    page_count: u32,
}

struct Magazine<'a> {
    title: &'a str,
    topic: &'a str,
    issue: u32,
}

fn print_publicaion(publications: &Vec<Publication>) {
    for publication in publications {
        print!("\n");
        match publication {
            Publication::Book(book) => {
                print!(
                    "{color_blue}Book: '{}', by '{}', {} pages{color_reset}",
                    book.title, book.author, book.page_count
                );
            }
            Publication::Magazine(magazine) => {
                print!(
                    "{color_blue}Magazine: '{}', topic: '{}', issue: {} {color_reset}",
                    magazine.title, magazine.topic, magazine.issue
                );
            }
        }
        print!("\n")
    }
}

fn main() {
    let book = Book {
        title: "The Catcher in the Rye",
        author: "J.D. Salinger",
        page_count: 234,
    };
    let magazine = Magazine {
        title: "The Economist",
        topic: "Economics",
        issue: 123,
    };

    let publication = vec![Publication::Book(book), Publication::Magazine(magazine)];

    print_publicaion(&publication);
}