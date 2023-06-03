pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn fnname(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn check_if_fnname_Works() {
        // let result = fnname
    }
}
