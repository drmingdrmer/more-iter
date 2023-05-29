use std::collections::BTreeMap;

use maplit::btreemap;

use crate::MergePair;

fn choose_left<V>(_left: V, _right: V) -> V {
    _left
}

#[test]
fn test_empty_input() {
    let left: Vec<(i32, i32)> = vec![];
    let right: Vec<(i32, i32)> = vec![];

    let merger = MergePair::merge(left, right, choose_left);
    let result: Vec<(i32, i32)> = merger.collect();
    assert_eq!(result, vec![]);
}

#[test]
fn test_no_overlap() {
    let left = [(1, 10), (3, 30)];
    let right = [(2, 20), (4, 40)];

    let merger = MergePair::merge(left, right, choose_left);
    let result: Vec<(i32, i32)> = merger.collect();
    assert_eq!(vec![(1, 10), (2, 20), (3, 30), (4, 40)], result);
}

#[test]
fn test_with_overlap_choose_left() {
    let left = [(1, 10), (3, 30), (4, 40)];
    let right = [(2, 20), (3, 300), (4, 400)];

    let merge = MergePair::merge(left, right, choose_left);
    let result: Vec<(i32, i32)> = merge.collect();
    assert_eq!(vec![(1, 10), (2, 20), (3, 30), (4, 40)], result);
}

#[test]
fn test_with_overlap_choose_right() {
    fn choose_right<V>(_left: V, _right: V) -> V {
        _right
    }

    let left = [(1, 10), (3, 30), (4, 40)];
    let right = [(2, 20), (3, 300), (4, 400)];

    let merge = MergePair::merge(left, right, choose_right);
    let result: Vec<(i32, i32)> = merge.collect();
    assert_eq!(vec![(1, 10), (2, 20), (3, 300), (4, 400)], result);
}

#[test]
fn test_with_overlap_choose_greater() {
    let left = [(1, 10), (3, 30), (4, 40)];
    let right = [(2, 200), (3, 300)];

    let merged = MergePair::merge(left, right, std::cmp::max).collect::<Vec<_>>();
    assert_eq!(vec![(1, 10), (2, 200), (3, 300), (4, 40)], merged);
}

#[test]
fn test_with_overlap_choose_greater_seq() {
    let a = btreemap! {
        10 => (1,1),
        15 => (2,2),
        20 => (3,3),
    };
    let b = btreemap! {
        11 => (1,1),
        15 => (3,3),
        21 => (4,4),
    };

    let merge = MergePair::merge(a, b, |l, r| if l.0 > r.0 { l } else { r });
    let result = merge.collect::<Vec<_>>();

    assert_eq!(
        vec![
            //
            (10, (1, 1)),
            (11, (1, 1)),
            (15, (3, 3)),
            (20, (3, 3)),
            (21, (4, 4)),
        ],
        result
    );
}

#[test]
fn test_with_overlap_combine_values() {
    let a: BTreeMap<u64, (u64, u64)> = btreemap! {
        15 => (2,2),
        20 => (3,3),
    };
    let b: BTreeMap<u64, (u64, u64)> = btreemap! {
        11 => (1,1),
        15 => (3,3),
    };

    let merge = MergePair::merge(a, b, |(lseq, ldata), (rseq, rdata)| (lseq + rseq, ldata + rdata));
    let result = merge.collect::<Vec<_>>();

    assert_eq!(
        vec![
            //
            (11, (1, 1)),
            (15, (5, 5)),
            (20, (3, 3)),
        ],
        result
    );
}
