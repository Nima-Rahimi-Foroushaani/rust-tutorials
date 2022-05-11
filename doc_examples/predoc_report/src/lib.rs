mod cell;
mod conc_exe_mod;
mod deque;
mod deque_annot;
mod panic_safety;
mod rust_access_type;
mod unsound_unsafe;

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

    // #[test]
    fn unsound_stupid_test() {
        crate::unsound_unsafe::stupid();
    }

    // #[test]
    fn unsound_worse_test() {
        let mut v = crate::unsound_unsafe::Vector {
            ptr: 0x0 as *mut i32,
            len: 0,
        };

        let r: &mut i32 = v.at(10);
        println!("The tenth element is {}", r);
    }

    #[test]
    fn bheap_test() {
        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        struct Token {
            text: String,
        }

        let mut bh = crate::panic_safety::BinaryHeap {
            data: vec![
                Token {
                    text: "Hello".to_string(),
                },
                Token {
                    text: "world".to_string(),
                },
                Token {
                    text: "!".to_string(),
                },
            ],
        };
        bh.sift_up(0, 1);
    }

    #[test]
    fn deque_test() {
        crate::deque::caller();
    }
}
