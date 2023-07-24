#[cfg(test)]
mod tests {
    use crate::check_for_duplicates::{has_duplicates_map, has_duplicates_set};

    #[test]
    fn test_has_duplicates_set_with_integers() {
        let arr1 = [1, 2, 3, 4, 5];
        let arr2 = [1, 2, 3, 4, 2];
        let arr3 = [4, 4, 4, 4, 4];
        let arr4: [i32; 0] = [];

        assert_eq!(has_duplicates_set(&arr1), false);
        assert_eq!(has_duplicates_set(&arr2), true);
        assert_eq!(has_duplicates_set(&arr3), true);
        assert_eq!(has_duplicates_set(&arr4), false);
    }

    #[test]
    fn test_has_duplicates_set_with_strings() {
        let arr1 = ["apple", "banana", "orange"];
        let arr2 = ["apple", "banana", "apple", "orange"];
        let arr3 = ["apple", "apple", "apple", "apple"];
        let arr4: [&str; 0] = [];

        assert_eq!(has_duplicates_set(&arr1), false);
        assert_eq!(has_duplicates_set(&arr2), true);
        assert_eq!(has_duplicates_set(&arr3), true);
        assert_eq!(has_duplicates_set(&arr4), false);
    }

    #[test]
    fn test_has_duplicates_map_with_integers() {
        let arr1 = [1, 2, 3, 4, 5];
        let arr2 = [1, 2, 3, 4, 2];
        let arr3 = [4, 4, 4, 4, 4];
        let arr4: [i32; 0] = [];

        assert_eq!(has_duplicates_map(&arr1), false);
        assert_eq!(has_duplicates_map(&arr2), true);
        assert_eq!(has_duplicates_map(&arr3), true);
        assert_eq!(has_duplicates_map(&arr4), false);
    }

    #[test]
    fn test_has_duplicates_map_with_strings() {
        let arr1 = ["apple", "banana", "orange"];
        let arr2 = ["apple", "banana", "apple", "orange"];
        let arr3 = ["apple", "apple", "apple", "apple"];
        let arr4: [&str; 0] = [];

        assert_eq!(has_duplicates_map(&arr1), false);
        assert_eq!(has_duplicates_map(&arr2), true);
        assert_eq!(has_duplicates_map(&arr3), true);
        assert_eq!(has_duplicates_map(&arr4), false);
    }
}
