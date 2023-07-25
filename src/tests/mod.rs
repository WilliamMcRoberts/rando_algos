#[cfg(test)]
mod tests {
    use crate::run_length_encoding::{run_length_decode, run_length_encode};
    use crate::{check_for_duplicates::*, depth_first_search::*};

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

    #[test]
    fn find_1_fail() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 99;

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            None
        );
    }

    #[test]
    fn find_1_success() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objective = 7;

        let correct_path = vec![1, 2, 4, 5, 3, 6, 7];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            Some(correct_path)
        );
    }

    #[test]
    fn find_2_success() {
        let vertices = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let edges = vec![
            (0, 1),
            (1, 3),
            (3, 2),
            (2, 1),
            (3, 4),
            (4, 5),
            (5, 7),
            (7, 6),
            (6, 4),
        ];

        let root = 0;
        let objective = 6;

        let correct_path = vec![0, 1, 3, 2, 4, 5, 7, 6];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            Some(correct_path)
        );
    }

    #[test]
    fn find_3_success() {
        let vertices = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let edges = vec![
            (0, 1),
            (1, 3),
            (3, 2),
            (2, 1),
            (3, 4),
            (4, 5),
            (5, 7),
            (7, 6),
            (6, 4),
        ];

        let root = 0;
        let objective = 4;

        let correct_path = vec![0, 1, 3, 2, 4];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search(&graph, root.into(), objective.into()),
            Some(correct_path)
        );
    }

    #[test]
    fn test_run_length_decode() {
        let res = run_length_decode(&[('A', 0)]);
        assert_eq!(res, "");
        let res = run_length_decode(&[('B', 1)]);
        assert_eq!(res, "B");
        let res = run_length_decode(&[('A', 5), ('z', 3), ('B', 1)]);
        assert_eq!(res, "AAAAAzzzB");
    }

    #[test]
    fn test_run_length_encode() {
        let res = run_length_encode("");
        assert_eq!(res, []);

        let res = run_length_encode("A");
        assert_eq!(res, [('A', 1)]);

        let res = run_length_encode("AA");
        assert_eq!(res, [('A', 2)]);

        let res = run_length_encode("AAAABBBCCDAA");
        assert_eq!(res, [('A', 4), ('B', 3), ('C', 2), ('D', 1), ('A', 2)]);

        let res = run_length_encode("Rust-Trends");
        assert_eq!(
            res,
            [
                ('R', 1),
                ('u', 1),
                ('s', 1),
                ('t', 1),
                ('-', 1),
                ('T', 1),
                ('r', 1),
                ('e', 1),
                ('n', 1),
                ('d', 1),
                ('s', 1)
            ]
        );
    }
}
