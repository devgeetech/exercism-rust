/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    vec![]
}

pub fn create_buffer(count: usize) -> Vec<u8> {
    vec![0; count]
}

pub fn fibonacci() -> Vec<u8> {
    let mut fib = vec![1, 1];

    for i in 2..5 {
        fib.push(fib[i-2]+fib[i-1]);
    }

    fib
}
