struct Person<'a> {
    name: &'a str,
    age: usize,
}

struct Book<'a> {
    title: String,
    author: String,
    is_available: bool,
    rented_by: Option<&'a Person<'a>>,
}

impl<'a> Default for &Person<'a> {
    fn default() -> Self {
        &Person {
            name: "None",
            age: 0,
        }
    }
}

struct Library<'a> {
    books: Vec<Book<'a>>,
}

impl<'a> Library<'a> {
    fn checkout_book(&mut self, title: &str, person: &'a Person) {
        match self.books.iter_mut().find(|a| a.title == title) {
            Some(book) => {
                if book.is_available {
                    book.is_available = false;
                    book.rented_by = Option::from(person);
                    println!(
                        "Librarian: Hey {}, Thank you for checking out the book: {}.",
                        person.name, title
                    );
                } else {
                    println!(
                        "Sorry, the book: {} is not available for rent. Try again next time!",
                        title
                    );
                    return;
                }
            }
            None => {
                println!("Sorry, couldn't find the book: {}", title);
                return;
            }
        };
    }

    fn return_book(&mut self, title: &str, person: &Person) {
        match self.books.iter_mut().find(|a| a.title == title) {
            Some(book) => {
                if !book.is_available
                    && book.rented_by.is_some()
                    && book.rented_by.unwrap().name == person.name
                {
                    book.is_available = true;
                    book.rented_by = None;
                    println!(
                        "Librarian: Hey {}, Thank you for returning the book: {}.",
                        person.name, title
                    );
                } else {
                    println!("Sorry, the book: {} cannot be returned.", title);
                    return;
                }
            }
            None => {
                println!("Sorry, couldn't find the book: {}", title);
                return;
            }
        };
    }

    fn list_books(&self) {
        println!("--- Library Logs ---");
        self.books.iter().for_each(|book| {
            let rented_by = book.rented_by.unwrap_or_default();

            println!(
                "Author: {} | Title: {} | Available: {} | Rented By: {}",
                book.author, book.title, book.is_available, rented_by.name
            )
        });
        println!("---")
    }
}

fn display_person(person: &Person) {
    println!(
        "{} just visited the library and is {} years old",
        person.name, person.age
    )
}

fn main() {
    println!(" -- assignment 01 --");
    let john = Person {
        name: "John",
        age: 25,
    };
    let jane = Person {
        name: "Jane",
        age: 28,
    };

    let sherlock_holmes = Book {
        title: "The Adventures of Sherlock Holmes".to_string(),
        author: "Arthur Conan Doyle".to_string(),
        is_available: true,
        rented_by: None,
    };

    let cosmos = Book {
        title: "Cosmos".to_string(),
        author: "Carl Sagan".to_string(),
        is_available: true,
        rented_by: None,
    };

    println!("Arranging books on the shelves. Please wait..");
    let mut library = Library {
        books: vec![sherlock_holmes, cosmos],
    };
    library.list_books();
    display_person(&john);
    library.checkout_book("The Adventures of Sherlock Holmes", &john);

    display_person(&jane);
    library.checkout_book("Cosmos", &jane);

    library.list_books();

    println!("One eternity later...");
    display_person(&jane);
    library.return_book("Cosmos", &jane);

    println!(
        "Librarian: Looks like {} ran away with the book. Time to make a call..",
        &john.name
    );

    print!(
        "Narrator: {} went missing, and Sherlock is investigating the case.",
        &john.name
    )
}
