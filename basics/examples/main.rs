fn main() {
    println!("SO FROM THE FOLLOWING TWO LINKS I UNDERSTAND CARGO'S PACKAGE LAYOUT");
    
    
    /*  TILL NOW , I WAS STRUGGLING TO CREATE FOLDERS INSIDE WHICH I CAN GET RUST ANALYSER 
    
    NOW WE KNOW!!!!!!

    
    THIS REALLY HELPED ME 
    https://users.rust-lang.org/t/hints-work-in-main-rs-but-not-other-files-why/81800/2 

    RUST LANG BOOK IS INSANE
    https://doc.rust-lang.org/cargo/guide/project-layout.html

    
    .
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── main.rs
│   └── bin/
│       ├── named-executable.rs
│       ├── another-executable.rs
│       └── multi-file-executable/
│           ├── main.rs
│           └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
    
    
    
     */


    // <------------------------------------------>
    println!("THIS EXAMPLE MODULES IS GONNA BE USED AS MY FOLDER FOR PYTHON -> RUST EXAMPLES")

}