mod cell;
mod rust_access_type;
#[cfg(test)]
mod tests {
    #[test]
    fn cell_test() {
        let c = crate::cell::Cell::new(42);
        let cr = &c;
        assert_eq!(cr.get(), 42);
        cr.set(43);
        assert_eq!(cr.get(), 43);
    }
}
