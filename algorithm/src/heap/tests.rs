use super::PriorityQueue;

#[test]
fn test_new() {
    let pq: PriorityQueue<i32> = PriorityQueue::new(1);

    assert_eq!(pq.size(), 0);
    assert!(pq.is_empty());
}

#[test]
fn test_push() {
    let mut pq = PriorityQueue::new(1);
    pq.push(1);

    assert_eq!(pq.size(), 1);
    assert!(pq.is_full());
}

#[test]
fn test_clear() {
    let mut pq = PriorityQueue::new(3);

    pq.push(1);
    pq.push(2);
    pq.push(3);
    assert_eq!(pq.size(), 3);
    assert!(pq.is_full());

    pq.clear();
    assert_eq!(pq.size(), 0);
    assert!(pq.is_empty());
}

#[test]
fn test_pop() {
    let mut pq = PriorityQueue::new(3);

    pq.push(1);
    pq.push(3);
    pq.push(2);

    assert_eq!(pq.size(), 3);
    assert!(pq.is_full());

    assert_eq!(pq.pop(), 3);
    assert_eq!(pq.pop(), 2);
    assert_eq!(pq.pop(), 1);

    assert_eq!(pq.size(), 0);
    assert!(pq.is_empty());
}

#[test]
fn test_custom_compartor() {
    let compartor = |i1: &i32, i2: &i32| i2.partial_cmp(i1);
    let mut pq = PriorityQueue::with_compare(3, compartor);

    pq.push(1);
    pq.push(3);
    pq.push(2);

    assert_eq!(pq.size(), 3);
    assert!(pq.is_full());

    assert_eq!(pq.pop(), 1);
    assert_eq!(pq.pop(), 2);
    assert_eq!(pq.pop(), 3);

    assert_eq!(pq.size(), 0);
    assert!(pq.is_empty());
}
