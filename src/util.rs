use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

pub fn parse_grid_rows<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn parse_grid_cols<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr + Clone,
    <T as FromStr>::Err: Debug,
{
    let mut grid = vec![];
    let mut first_line = true;
    for line in input.lines() {
        let elems: Vec<T> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        if first_line {
            grid = vec![vec![]; elems.len()];
            first_line = false;
        }
        for (i, elem) in elems.into_iter().enumerate() {
            grid[i].push(elem);
        }
    }
    grid
}

pub fn get_counter<T: Eq + std::hash::Hash + Clone>(list: &[T]) -> HashMap<T, i32> {
    let mut counter = HashMap::new();
    for item in list {
        *counter.entry(item.clone()).or_insert(0) += 1;
    }
    counter
}

#[allow(dead_code)]
pub fn cached<Arg: Eq + std::hash::Hash + Clone, Ret: Clone>(
    mut f: impl FnMut(Arg) -> Ret,
) -> impl FnMut(Arg) -> Ret {
    let mut cache: HashMap<Arg, Ret> = HashMap::new();
    move |arg| {
        if let Some(ret) = cache.get(&arg) {
            ret.clone()
        } else {
            let ret = f(arg.clone());
            cache.insert(arg, ret.clone());
            ret
        }
    }
}

macro_rules! vec_to_tuple {
    ($vec:expr, $n:literal) => {
        seq_macro::seq!(_ in 0..$n {
            {
                let mut it = $vec.into_iter();
                (#(it.next().unwrap(),)*)
            }
        })
    }
}
pub(crate) use vec_to_tuple;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid_rows() {
        let input = "1 2 3\n4 5 6\n7 8 9";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        assert_eq!(parse_grid_rows::<i32>(input), expected);
    }

    #[test]
    fn test_parse_grid_cols() {
        let input = "1 2 3\n4 5 6\n7 8 9";
        let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(parse_grid_cols::<i32>(input), expected);
    }

    #[test]
    fn test_get_counter() {
        let input = vec![1, 2, 3, 1, 2, 3, 1, 2, 3];
        let expected = vec![(1, 3), (2, 3), (3, 3)]
            .into_iter()
            .collect::<HashMap<_, _>>();
        assert_eq!(get_counter(&input), expected);
    }

    #[test]
    fn test_cached() {
        fn fib(n: i32) -> i32 {
            if n <= 1 {
                n
            } else {
                fib(n - 1) + fib(n - 2)
            }
        }

        let mut cached_fib = cached(fib);
        assert_eq!(cached_fib(10), 55);
        assert_eq!(cached_fib(40), 102334155);

        let mut called_once = false;
        let f = move |_: i32| -> bool {
            if called_once {
                return false;
            }
            called_once = true;
            return true;
        };

        let mut cached_f = cached(f);
        assert!(cached_f(0));
        assert!(cached_f(0));
        assert!(!cached_f(1));
    }

    #[test]
    fn test_vec_to_tuple() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let (a, b, c) = vec_to_tuple!(input, 3);
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);

        let input = vec![1, 2, 3, 4, 5, 6];
        let (a, b, c, d, e, f) = vec_to_tuple!(input, 6);
        assert_eq!(a, 1);
        assert_eq!(b, 2);
        assert_eq!(c, 3);
        assert_eq!(d, 4);
        assert_eq!(e, 5);
        assert_eq!(f, 6);
    }
}
