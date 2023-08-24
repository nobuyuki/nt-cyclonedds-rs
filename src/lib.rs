///
///
///
///
///
///
///
///
pub mod core;
pub mod domain;
pub mod idl;
pub mod pub_;
pub mod sub;
pub mod topic;

mod internal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
