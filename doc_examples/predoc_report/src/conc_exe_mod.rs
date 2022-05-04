pub fn fun_a() {}

pub fn fun_b() { unsafe{ fun_c() } }

pub unsafe fn fun_c() { /* Do somthing */ }