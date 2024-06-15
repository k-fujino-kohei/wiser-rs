use crate::{
    database::Storage,
    ngram::{Ngram, TokinizedChars},
    postings::merge_inverted_index_hash,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct PostingsList {
    pub document_id: usize,
    /// 特定文書中の位置情報配列
    pub positions: Vec<usize>,
    /// 特定文書中の位置情報の数
    pub positions_count: usize,
    pub next: Option<Box<PostingsList>>,
}
type TokenID = usize;
#[derive(Debug)]
pub struct InvertedIndex {
    pub token_id: TokenID,
    pub postings_list: PostingsList,
    /// トークンを含む文書数
    pub docs_count: usize,
    /// 全文書内でのトークン出現数
    pub positions_count: usize,
}
pub type InvertedIndexHash = HashMap<TokenID, InvertedIndex>;

impl InvertedIndex {
    pub fn new(token_id: usize, postings_list: PostingsList) -> Self {
        Self {
            token_id,
            postings_list,
            docs_count: 1,
            positions_count: 1,
        }
    }
}

pub fn text_to_postings_lists(
    document_id: usize,
    text: &str,
    text_size: usize,
    n: usize,
    db: &Storage,
    inverted_index_hash: &mut InvertedIndexHash,
) {
    let mut buffer = InvertedIndexHash::new();
    let chars = TokinizedChars::from(text);
    for (index, ngram) in chars.ngrams(n).enumerate() {
        let position = index;
        let token: String = ngram.iter().collect();
        let token_id = db.get_token_id(&token);
        token_to_postings_list(document_id, token_id, position, &mut buffer);
    }

    merge_inverted_index_hash(inverted_index_hash, buffer);
}

fn token_to_postings_list(
    document_id: usize,
    token_id: usize,
    position: usize,
    inverted_index_hash: &mut InvertedIndexHash,
) {
    // 構築済のミニ転置インデックスを取得する
    let postings = inverted_index_hash.get_mut(&token_id);
    if let Some(postings) = postings {
        // 同一トークンが出現したので、位置情報を更新する
        postings.positions_count += 1;
        postings.postings_list.positions.push(position);
        postings.postings_list.positions_count += 1;
        return;
    }

    // ミニ転置インデックスを新規作成する
    let inverted_index = InvertedIndex::new(
        token_id,
        PostingsList {
            document_id,
            positions: vec![position],
            positions_count: 1,
            next: None,
        },
    );
    inverted_index_hash.insert(token_id, inverted_index);
}
