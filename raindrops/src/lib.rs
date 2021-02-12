pub fn raindrops(n: u32) -> String {
    let mut s = String::new();
    let mut boot = 0;
    if n%3==0 {
        s += "Pling";
        boot = 1;
    }
    if n%5==0 {
        s += "Plang";
        boot = 1;
    }
    if n%7==0 {
        s += "Plong";
        boot = 1;
    }
    if boot==0 {
        s = n.to_string();
    }
    
    s
}
