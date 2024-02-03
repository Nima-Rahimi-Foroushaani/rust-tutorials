pub fn case1_1(mut v: Vec<usize>) {
    v.push(v.len())
}
/*
fn case1_1(_1: std::vec::Vec<usize, std::alloc::Global>) -> () {
    debug v => _1;                       // in scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:1:16: 1:21
    let mut _0: ();                      // return place in scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:1:35: 1:35
    let mut _2: &'_#5r mut std::vec::Vec<usize, std::alloc::Global>; // in scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:5: 2:20
    let mut _3: usize;                   // in scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:12: 2:19
    let mut _4: &'_#6r std::vec::Vec<usize, std::alloc::Global>; // in scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:12: 2:19

    bb0: {
        StorageLive(_2);                 // bb0[0]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:5: 2:20
        _2 = &'_#3r mut _1;              // bb0[1]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:5: 2:20
        StorageLive(_3);                 // bb0[2]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:12: 2:19
        StorageLive(_4);                 // bb0[3]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:12: 2:19
        _4 = &'_#4r _1;                  // bb0[4]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:12: 2:19
        _3 = Const(Value(Scalar(<ZST>)): for<'r> fn(&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::vec::Vec<usize, std::alloc::Global>) -> usize {std::vec::Vec::<usize, std::alloc::Global>::len})(move _4) -> [return: bb1, unwind: bb4]; // bb0[5]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:12: 2:19
                                         // mir::Constant
                                         // + span: /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:14: 2:17
                                         // + literal: Const { ty: for<'r> fn(&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) std::vec::Vec<usize, std::alloc::Global>) -> usize {std::vec::Vec::<usize, std::alloc::Global>::len}, val: Value(Scalar(<ZST>)) }
    }

    bb1: {
        StorageDead(_4);                 // bb1[0]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:18: 2:19
        _0 = Const(Value(Scalar(<ZST>)): for<'r> fn(&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) mut std::vec::Vec<usize, std::alloc::Global>, usize) {std::vec::Vec::<usize, std::alloc::Global>::push})(move _2, move _3) -> [return: bb2, unwind: bb4]; // bb1[1]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:5: 2:20
                                         // mir::Constant
                                         // + span: /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:7: 2:11
                                         // + literal: Const { ty: for<'r> fn(&ReLateBound(DebruijnIndex(0), BoundRegion { var: 0, kind: BrNamed('r) }) mut std::vec::Vec<usize, std::alloc::Global>, usize) {std::vec::Vec::<usize, std::alloc::Global>::push}, val: Value(Scalar(<ZST>)) }
    }

    bb2: {
        StorageDead(_3);                 // bb2[0]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:19: 2:20
        StorageDead(_2);                 // bb2[1]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:2:19: 2:20
        drop(_1) -> [return: bb3, unwind: bb5]; // bb2[2]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:3:1: 3:2
    }

    bb3: {
        return;                          // bb3[0]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:3:2: 3:2
    }

    bb4 (cleanup): {
        drop(_1) -> bb5;                 // bb4[0]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:3:1: 3:2
    }

    bb5 (cleanup): {
        resume;                          // bb5[0]: scope 0 at /home/nima/Projects/rust-tutorials/two_phase_bor/src/lib.rs:1:1: 3:2
    }
}*/

pub fn case1_2(r: &mut Vec<usize>) {
    r.push(r.len())
}

pub fn case2(r: &mut Vec<usize>) {
    let _ = std::mem::replace(r, vec![r.len()]);
}

pub fn case3() {
    let mut x = std::num::Wrapping(2);
    x += x;
}
