mod dictionary;

use dictionary::Dictionary;

/* 
 * Creates the dictionary
 * {
 * 1: 6,
 * 2: 7,
 * 3: 8,
 * 4: 9,
 * 5: 0
 * }
 */
#[allow(dead_code)]
fn create_dict() -> Dictionary<u8, u8> {
    let tuples: Vec<(u8, u8)> = vec![(1, 6), (2, 7), (3, 8),
                             (4, 9), (5, 0)];
    Dictionary::from_tuples(tuples)
}

#[allow(dead_code)]
fn has_same_elements<T: PartialEq>(vec1: &Vec<T>, vec2: &Vec<T>) -> bool {
    for i in vec1 {
        if vec2.contains(i) {
            continue;
        }
        return false;
    }
    true
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn make_dict() {
        let _d: Dictionary<u8, u8> = Dictionary::new();
        
        assert_eq!(_d.capacity(), 8);
    }

    #[test]
    fn create_sized() {
        let _d: Dictionary<u8, u8> = Dictionary::with_capacity(16); 
        assert_eq!(_d.capacity(), 16);
    }

    #[test]
    #[should_panic]
    fn zero_sized_dict() {
        let _d: Dictionary<u8, u8> = Dictionary::with_capacity(0);
    }

    #[test]
    fn create_from_vecs() {
        let vec1: Vec<usize> = vec![1, 2, 3, 4, 5];
        let vec2: Vec<usize> = vec![6, 7, 8, 9, 0];
        let _d: Dictionary<usize, usize> = Dictionary::from_vecs(vec1, vec2);
       
        assert_eq!(_d.size(), 5); 
    }

    #[test]
    fn create_from_tuples() {
        let tuples: Vec<(u8, u8)> = vec![(1, 2), (3, 4)];
        let _d: Dictionary<u8, u8> = Dictionary::from_tuples(tuples);

        assert_eq!(_d.get(&1).unwrap(), 2);
    }

    #[test]
    #[should_panic]
    fn zero_sized_tuple_dict() {
        let tuples: Vec<(u8, u8)> = Vec::new();
        let _d: Dictionary<u8, u8> = Dictionary::from_tuples(tuples);
    }
    
    #[test]
    #[should_panic]
    fn paniced_from_vecs() {
        let vec1: Vec<usize> = vec![1, 2, 3, 4];
        let vec2: Vec<usize> = vec![5, 6, 7];
        let _d = Dictionary::from_vecs(vec1, vec2);
    }

    #[test]
    #[should_panic]
    fn zero_sized_vecs() {
        let vec1: Vec<u8> = Vec::new();
        let vec2: Vec<u8> = Vec::new();
        let _d = Dictionary::from_vecs(vec1, vec2);
    }

    #[test]
    fn lookup() {
        let _d = create_dict();

        assert_eq!(_d.get(&1).unwrap(), 6);
    }

    #[test]
    fn insert() {
        let mut _d: Dictionary<u8, u8> = Dictionary::new();
        _d.insert(1, 2);

        assert_eq!(_d.get(&1).unwrap(), 2);
    }

    #[test]
    fn size() {
        let _d = create_dict();
        assert_eq!(_d.size(), 5);
    }

    #[test]
    fn resize() {
        let mut _d: Dictionary<u8, u8> = Dictionary::with_capacity(4);
        assert_eq!(_d.capacity(), 4);
        for i in 0..4{
            _d.insert(i, i);
        }

        assert_eq!(_d.capacity(), 8);
    }

    #[test]
    fn contains() {
        let mut _d = create_dict();
        
        assert!(_d.contains(&1));
    }

    #[test]
    fn remove() {
        let mut _d = create_dict();
        let _r = _d.remove(&1);
        
        assert!((!_d.contains(&1)) &&
        _r.is_some() &&
        _r.unwrap() == (1, 6) &&
        _d.size() == 4);
    }
    
    #[test]
    fn down_size() {
        let mut _d = create_dict();
        
        _d.remove(&1);
        _d.remove(&2);

        assert_eq!(_d.capacity(), 5);
    }

    #[test]
    fn remove_panic() { 
        let mut _d: Dictionary<u8, u8> = Dictionary::new();
        _d.remove(&1);
    }

    #[test]
    fn keys() {
        let _d = create_dict();
        let expected_keys: Vec<u8> = vec![1, 2, 3, 4, 5];
        let keys = _d.keys().into_iter().map(|x| *x).collect(); 
        assert!(has_same_elements(&keys, &expected_keys));
    }
    
    #[test]
    fn values() {
        let _d = create_dict();
        let expected_values: Vec<u8> = vec![6, 7, 8, 9, 0];
        let values = _d.values().into_iter().map(|x| *x).collect();
        assert!(has_same_elements(&values, &expected_values));
    }

    #[test]
    fn items() {
        let tuples: Vec<(u8, u8)> = vec![(1, 6), (2, 7), (3, 8), (4, 9), (5, 0)];
        let _t = tuples.clone();
        let _d: Dictionary<u8, u8> = Dictionary::from_tuples(_t);
        let expected_items = _d.items().into_iter().map(|x| (*x.0, *x.1)).collect();
        assert!(has_same_elements(&expected_items, &tuples));
    }
}
