fn main() {}

#[cfg(test)]
mod tests {
    #[test]
    fn test_iterator_next() {
        let v1 = vec![1, 2, 3];

        let mut v_itr = v1.iter();
        assert_eq!(v_itr.next(), Some(&1));
        assert_eq!(v_itr.next(), Some(&2));
        assert_eq!(v_itr.next(), Some(&3));
        assert_eq!(v_itr.next(), None);
    }
}
