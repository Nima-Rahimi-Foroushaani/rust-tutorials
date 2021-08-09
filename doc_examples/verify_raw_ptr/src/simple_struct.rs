#![allow(unused)]

pub struct Account {
    pub balance:i32,
}

pub fn read_ptr_from_shared(a: &Account) -> i32 {
    // Automatic production of guarantees
    
    // Active_Context: a: source(Account)
    // Symbolic_Store: a: _a
    // Symbolic_Mem_Stage: (_a, {balance: _balance})
    // Assumptions:
    let raw_ptr: *const Account = a; // coerce
    // Active_Context: a: source(Account), raw_ptr: *source(Account)
    // Symbolic_Store: a: _a, raw_ptr: _a
    // Symbolic_Mem_Stage: (_a, {balance: _balance})
    // Asuumptions:
    let result;
    // Unbound_Context: result: i32
    // Active_Context: a: source(Account), raw_ptr: *source(Account)
    // Symbolic_Store: a: _a, raw_ptr: _a
    // Symbolic_Mem_Stage: (_a, {balance: _balance})
    // Asuumptions:
    
    let b = a.balance;
    // Unbound_Context: result: i32
    // Active_Context: a: source(Account), raw_ptr: *source(Account), b: i32
    // Symbolic_Store: a: _a, raw_ptr: _a, b: _balance
    // Symbolic_Mem_Stage: (_a, {balance: _balance})
    // Asuumptions:
    
    unsafe {
        // below field access needs an approperiate chunk to be verified
        // generating desired chunks using current information:
        
        // @open a;
        // Active_Context(a, source(Account)) and Symbolic_Store(a, _a) ==>
        //      consume (_a, {balance: ?balance})
        //      produce [p]_a->balance |-> balance &*& 0<p &*& p=<1
        
        // Unbound_Context: result: i32
        // Active_Context: a: source(Account), raw_ptr: *source(Account), b: i32
        // Symbolic_Store: a: _a, raw_ptr: _a, b: _balance, p: _p, balance: _balance
        // Symbolic_Mem_Stage: [_p]Account_balance(_a, _balance)
        // Asuumptions: _p > 0, _p =< 1
        
        result = (*raw_ptr).balance;
        // Unbound_Context:
        // Active_Context: a: source(Account), raw_ptr: *source(Account), b: i32, result: i32
        // Symbolic_Store: a: _a, raw_ptr: _a, b: _balance, p: _p, balance: _balance, result: _balance
        // Symbolic_Mem_Stage: [_p]Account_balance(_a, _balance)
        // Asuumptions: _p > 0, _p =< 1
        
        
        // close a;
        // Active_Context(a, source(Account)) and Symbolic_Store(a, _a)
        //      consume [p]_a->balance |-> ?balance
        //      produce (_a, {balance: balance})
        
        // Unbound_Context:
        // Active_Context: a: source(Account), raw_ptr: *source(Account), b:i32, result: i32
        // Symbolic_Store: a: _a, raw_ptr: _a, b: _balance, p: _p, balance: _balance, result: _balance
        // Symbolic_Mem_Stage: (_a, {balance: _balance})
        // Asuumptions: _p > 0, _p =< 1
        
        // nothing goes out of scope here
    }
    // clean up check:
    // forall r: source(T) => Exists(_r, v:T)
    // forall r: sink+source(T) => Unique(_r, v:T)
    
    assert_eq!(b, result); // _balance == _balance
    
    // return statement
    result
    
}
