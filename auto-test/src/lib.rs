#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true, "custom message with {}", String::from("place holder"));
        assert_eq!(
            2 + 2,
            4,
            "custom message with {}",
            String::from("place holder")
        );
        assert_ne!(1, 2, "custom message with {}", String::from("place holder"));
    }
    
    #[test]
    #[should_panic(expected="wise panic...")]
    fn this_one_too() {
        panic!("I am a wise panic...!")
    }
    
    #[test]
    #[ignore]
    fn it_does_not() {
        panic!("Panic and burn!");
    }
    
    #[test]
    #[ignore]
    fn test_return() -> Result<(), String> {
//        Ok(())
        Err(String::from("I Fail with return"))
    }
}
