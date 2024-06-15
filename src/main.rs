use crate::token::InvertedIndexHash;

mod database;
mod ngram;
mod postings;
mod token;

fn main() {
    println!("main started");

    let db = database::Storage::new();

    let mut inverted_index_hash = InvertedIndexHash::new();
    add_document("title1", "Apple pencil", &db, &mut inverted_index_hash);
    add_document("title2", "Apple pencil next", &db, &mut inverted_index_hash);
    for (_, inverted_index) in inverted_index_hash.iter() {
        println!("{:?}", inverted_index);
    }
    println!("db: {:?}", db);

    println!("main terminated");
}

const N_GRAM: usize = 2;
const INVERTED_INDEX_UPDATE_THRESHOLD: usize = 1;

/// 転置インデックスを作成する
fn add_document(
    title: &str,
    body: &str,
    db: &database::Storage,
    inverted_index_hash: &mut InvertedIndexHash,
) {
    let title_size = title.len();
    let body_size = body.len();
    db.add_document(title, title_size, body, body_size);
    let document_id = db.get_document_id(title);
    token::text_to_postings_lists(
        document_id,
        body,
        body_size,
        N_GRAM,
        db,
        inverted_index_hash,
    );

    if inverted_index_hash.len() > INVERTED_INDEX_UPDATE_THRESHOLD {
        println!("update inverted index");
    }
}
