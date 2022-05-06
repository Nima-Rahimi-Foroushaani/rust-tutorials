pub fn fun_a() { fun_b() }

pub fn fun_b() { unsafe{ fun_c() } }

pub unsafe fn fun_c() { /* I'm using superpower */ }