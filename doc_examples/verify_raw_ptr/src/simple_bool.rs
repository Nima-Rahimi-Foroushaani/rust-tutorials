#![allow(unused)]

use crate::access_simulate_fn;

pub fn simple_bool_shared() {
    let x: bool;
    x = true;
    let shared_x: &bool;
    shared_x = &x;
    
    access_simulate_fn::read_access(shared_x);
}

pub fn simple_bool_shared_interleaved() {
    
    let x: bool;
    x = true;
    
    let r1_x: &bool;
    
    {
        let r2_x: &bool;
        r2_x = &x;
        
        r1_x = &x;
        
        access_simulate_fn::read_access(r2_x);
    }
    
    access_simulate_fn::read_access(r1_x);
}