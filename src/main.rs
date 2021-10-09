use std::time::Instant;
use std::process::{Command, Stdio};
use std::env::current_exe;
use pyo3::PyTryInto;
use uuid::Uuid;
use threadpool::ThreadPool;
use clap::{Arg, App};
use pyo3::{Python, types::{PyLong, PyModule}};


fn spawn_child() {
    // lazily for this project, but just run another instance of the same executable but using the "worker" runtype.
    let exe_path = current_exe().unwrap();
    let mut child_process = Command::new(exe_path)
        .arg("worker")
        .stdout(Stdio::inherit()) // use the broker process' stdout and stderr
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    // run child process and debug print exit status.
    let status = child_process.wait();
    match status {
        Ok(es) => println!("Finished with status: {}", es),
        Err(e) => println!("!!! Error: {}", e)
    }
}


fn worker_main() {
    // run 8 jobs in rust, then 8 jobs using Python code from Rust
    rust_task();
    python_task();
}


fn rust_task() -> usize {
    let start_time = Instant::now();
    let uuid = Uuid::new_v4();
    let mut num_found = 0;
    for num in 0..25_000_000 {
        if num % 2 == 0 || num % 3 == 0 || num % 5 == 0 {
            let bitthing = num ^ 0x42;
            let isthree = bitthing & 3;
            if isthree == 3 {
                num_found += 1;
            }
            if num_found % 250_000 == 0 {
                println!("RUST {} found {} so far in {}", uuid, num_found, num);
            }
        }
    }
    let runtime = start_time.elapsed();
    println!("Finished in: {}.{}", runtime.as_secs(), runtime.subsec_nanos());
    println!("Worker result: {}", num_found);
    num_found
}


fn python_task() -> usize {
    // This task just loads, then runs the python code.
    let result = Python::with_gil(|py_interpreter| {
        // read the source code file into a string, then parse it into a Python module
        let source_path = "test_python_code.py";
        let testcode_str = std::fs::read_to_string(&source_path).unwrap();

        let module = PyModule::from_code(
            py_interpreter,
            &testcode_str,
            "test_python_code.py",
            "test"
        ).unwrap();

        // get and run entrypoint method in python code
        let function_return_val: &PyLong = module.getattr("test_function").unwrap().call0().unwrap().try_into().unwrap();
        println!("Worker result: {}", function_return_val);
        // return python int (PyLong w/ pyo3) to Rusts' usize
        function_return_val.extract().unwrap()
    });
    result
}

fn main() {
    // use clap since we need to run multiple processes. Rather than make two seperate binaries, use the same one with two runtypes based on positional arg.
    let args = App::new("Testing program")
        .arg(Arg::with_name("ptype")
            .possible_values(&["worker", "broker"])
            .index(1))
        .get_matches();
    
    // if broker, spawn threads that each will execute and manage a new worker process
    if args.value_of("ptype").unwrap() == "broker" {
        let n_workers = 16;
        let pool = ThreadPool::new(n_workers);
        for _ in 0..n_workers {
            pool.execute(|| spawn_child());
        }
        pool.join();
    } else {
        // if worker, prepare Python and then run code
        pyo3::prepare_freethreaded_python();
        worker_main();
    }        
}
