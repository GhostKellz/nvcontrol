pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub mod gpu;
pub mod display;
pub mod vibrance;
pub mod fan;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
