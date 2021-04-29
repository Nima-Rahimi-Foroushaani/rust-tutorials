#![allow(unused)]

pub struct Account {
    pub balance:i32,
}

pub fn read_unsafe(a: &Account) -> i32 {
    // Active_Context: a: source(Account)
    // Symbolic_Store: a: _a
    // Symbolic_Heap:
    // Assumptions:
    let raw_ptr: *const Account = a; // coerce
    // Active_Context: a: source(Account), raw_ptr: *source(Account)
    // Symbolic_Store: a: _a, raw_ptr: _a
    // Symbolic_Heap:
    // Asuumptions:
    let result;
    // Unbound_Context: result: i32
    // Active_Context: a: source(Account), raw_ptr: *source(Account)
    // Symbolic_Store: a: _a, raw_ptr: _a
    // Symbolic_Heap:
    // Asuumptions:
    
    unsafe {
        // below field access needs an approperiate chunk to be verified
        // generating desired chunks using current information:
        // Active_Context(a, source(Account)) and Symbolic_Store(a, _a) ==> produce [?p]_a->balance |-> ?balance &*& 0<p &*& p=<1;
        
        // Unbound_Context: result: i32
        // Active_Context: a: source(Account), raw_ptr: *source(Account)
        // Symbolic_Store: a: _a, raw_ptr: _a, p: _p, balance: _balance
        // Symbolic_Heap: [_p]Account_balance(_a, _balance)
        // Asuumptions: _p > 0, _p =< 1
        
        result = (*raw_ptr).balance;
        // Unbound_Context:
        // Active_Context: a: source(Account), raw_ptr: *source(Account)
        // Symbolic_Store: a: _a, raw_ptr: _a, p: _p, balance: _balance, result: _balance
        // Symbolic_Heap: [_p]Account_balance(_a, _balance)
        // Asuumptions: _p > 0, _p =< 1
        
        // nothing goes out of scope here
    }
    // From the end of unsafe block to the end of safe abstraction we should check accesses to realized references
    
    // return statement
    result
    
    // clean up: this is a safe abstraction and the consistency of heap should be checked
    // Active_Context(a, source(Account)) and Symbolic_Store(a, _a) ==> consume [?p]_a->balance |-> ?balance &*& 0<p &*& p=<1;
}