use crate::{Author, Book};

pub fn books_db() -> Vec<Book> {
    vec![
        Book {
            id: 1,
            name: String::from("A very interesting title"),
            published: 2023,
            author_id: 1,
            author: None,
        },
        Book {
            id: 2,
            name: String::from("A not interesting title"),
            published: 2012,
            author_id: 2,
            author: None,
        },
        Book {
            id: 3,
            name: String::from("A somewhat interesting title"),
            published: 1995,
            author_id: 3,
            author: None,
        },
        Book {
            id: 1,
            name: String::from("A fascinating title"),
            published: 2018,
            author_id: 2,
            author: None,
        },
        Book {
            id: 2,
            name: String::from("A scintillating title"),
            published: 2001,
            author_id: 1,
            author: None,
        },
    ]
}

pub fn authors_db() -> Vec<Author> {
    vec![
        Author {
            id: 1,
            name: String::from("John Smith"),
        },
        Author {
            id: 2,
            name: String::from("Bob Billboard"),
        },
        Author {
            id: 3,
            name: String::from("Abraham Lincoln"),
        },
    ]
}
