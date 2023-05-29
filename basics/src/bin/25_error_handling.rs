fn main(){
    // [[1]]
    // panic macro quits the program immediately
    // used when - we can't "handle an error gracefully"
    //           - program fails in a way that is unrecoverable

    // panic!("crash and burn");


    // [[2]]
    // use backtracing to find out what went wrong and 
    // HOW DID IT LEAD TO THAT. 
    // We check the call stack and figure out what went wrong

    // run this command to see the call stack
    // RUST_BACKTRACE=1 cargo run -q --bin 25_error_handling
    let _num = a();
    println!("{:?}", _num);


    /*            CALL STACK 
    
    thread 'main' panicked at 'code must crash and burn. DO NOT PASS 32!!', src/bin/25_error_handling.rs:36:15
    stack backtrace:
   0: rust_begin_unwind
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/std/src/panicking.rs:579:5
   1: core::panicking::panic_fmt
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/panicking.rs:64:14
   2: _25_error_handling::d
             at ./src/bin/25_error_handling.rs:36:15
   3: _25_error_handling::c
             at ./src/bin/25_error_handling.rs:30:5
   4: _25_error_handling::b
             at ./src/bin/25_error_handling.rs:26:5
   5: _25_error_handling::a
             at ./src/bin/25_error_handling.rs:22:5
   6: _25_error_handling::main
             at ./src/bin/25_error_handling.rs:16:16
   7: core::ops::function::FnOnce::call_once
             at /rustc/84c898d65adf2f39a5a98507f1fe0ce10a2b8dbc/library/core/src/ops/function.rs:250:5
    note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace


     */

}

fn a(){
    b()
}

fn b(){
    c()
}

fn c(){
    d(32)
}


fn d(num: usize) {
    match num {
        32 => panic!("code must crash and burn. DO NOT PASS 32!!"),
        other => {
            println!("{other}")
        }
    }

}