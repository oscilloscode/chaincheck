use chaincheck_macro::chaincheck;

pub fn add(left: u64, right: u64) -> u64 {
    let v = vec![1, 2, 3];

    chaincheck!(v.iter().map(|x| x + 1););

    8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
