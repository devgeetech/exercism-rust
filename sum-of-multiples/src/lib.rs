fn insert_into_array<'a>(element: u32, mut arr: Vec<u32>) -> Vec<u32> {
    let x = arr.binary_search(&element);
    match x {
        Ok(_i) => return arr,
        Err(_i) => {
            arr.push(element);
            return arr
        }
    }
}
 
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut v = Vec::new();
    for i in 0..factors.len() {
        if factors[i]==0 {
            continue;
        }
        let mut j = 1;
        let mut prod = factors[i]*j;
        while prod<limit {
            v = insert_into_array(prod, v);
            j = j + 1;
            prod = factors[i]*j;
        }
    }
    v.sort();
    v.dedup();
    let sum:u32 = v.iter().sum();
    sum  
}
