use crate::token::{InvertedIndex, InvertedIndexHash, PostingsList};

pub fn update_postings(p: InvertedIndex) {
    println!("update inverted index");
}

pub fn merge_inverted_index_hash(base: &mut InvertedIndexHash, to_be_added: InvertedIndexHash) {
    for (token_id, inverted_index) in to_be_added {
        if let Some(base_inverted_index) = base.get_mut(&token_id) {
            base_inverted_index.docs_count += inverted_index.docs_count;
            base_inverted_index.positions_count += inverted_index.positions_count;
            base_inverted_index.postings_list = merged_postings(
                base_inverted_index.postings_list.clone(),
                inverted_index.postings_list,
            );
        } else {
            base.insert(token_id, inverted_index);
        }
    }
}

fn merged_postings(base: PostingsList, to_be_added: PostingsList) -> PostingsList {
    let mut head: Option<Box<PostingsList>> = None;
    let mut tail = &mut head;
    let mut pa = Some(Box::new(base));
    let mut pb = Some(Box::new(to_be_added));
    while pa.is_some() || pb.is_some() {
        let (next_node, next_pa, next_pb) = match (pa.take(), pb.take()) {
            (Some(mut a), Some(mut b)) => {
                if a.document_id <= b.document_id {
                    let next_a = a.next.take();
                    pb = Some(b);
                    (a, next_a, pb)
                } else {
                    let next_b = b.next.take();
                    pa = Some(a);
                    (b, pa, next_b)
                }
            }
            (Some(mut a), None) => {
                let next_a = a.next.take();
                (a, next_a, None)
            }
            (None, Some(mut b)) => {
                let next_b = b.next.take();
                (b, None, next_b)
            }
            (None, None) => unreachable!(),
        };

        if let Some(t) = tail {
            t.next = Some(next_node);
            tail = &mut t.next;
        } else {
            *tail = Some(next_node);
        }

        pa = next_pa;
        pb = next_pb;
    }
    *head.unwrap()
}

mod tests {
    use crate::token::InvertedIndex;

    use super::*;

    #[test]
    fn test_merged_inverted_index_hash() {
        let mut base = InvertedIndexHash::new();
        base.insert(
            1,
            InvertedIndex {
                token_id: 1,
                postings_list: PostingsList {
                    document_id: 1,
                    positions: vec![1, 2, 3],
                    positions_count: 3,
                    next: None,
                },
                docs_count: 1,
                positions_count: 3,
            },
        );
        let mut to_be_added = InvertedIndexHash::new();
        to_be_added.insert(
            2,
            InvertedIndex {
                token_id: 1,
                postings_list: PostingsList {
                    document_id: 1,
                    positions: vec![10, 20, 30],
                    positions_count: 3,
                    next: None,
                },
                docs_count: 1,
                positions_count: 3,
            },
        );
        to_be_added.insert(
            2,
            InvertedIndex {
                token_id: 2,
                postings_list: PostingsList {
                    document_id: 2,
                    positions: vec![100, 200, 300],
                    positions_count: 3,
                    next: None,
                },
                docs_count: 1,
                positions_count: 3,
            },
        );
        merge_inverted_index_hash(&mut base, to_be_added);
        println!("{:?}", base);
        assert_eq!(base.len(), 2);
    }

    #[test]
    fn test_merge_postings() {
        let base = PostingsList {
            document_id: 1,
            positions: vec![1, 2, 3],
            positions_count: 3,
            next: Some(Box::new(PostingsList {
                document_id: 2,
                positions: vec![4],
                positions_count: 1,
                next: None,
            })),
        };
        let to_be_added = PostingsList {
            document_id: 10,
            positions: vec![10, 11, 12, 13],
            positions_count: 4,
            next: None,
        };
        let merged = merged_postings(base, to_be_added);
        assert_eq!(merged.document_id, 1);
        assert_eq!(merged.positions, vec![1, 2, 3]);
        assert_eq!(merged.positions_count, 3);
        let next = merged.next.unwrap();
        assert_eq!(next.document_id, 2);
        assert_eq!(next.positions, vec![4]);
        assert_eq!(next.positions_count, 1);
        let next = next.next.unwrap();
        assert_eq!(next.document_id, 10);
        assert_eq!(next.positions, vec![10, 11, 12, 13]);
        assert_eq!(next.positions_count, 4);
    }
}
