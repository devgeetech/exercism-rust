fn bottle_no(no: i32, is_first: bool) -> String {
    match no {
        0 => { 
            return if is_first {String::from("No more")} else {String::from("no more")};
        },
        -1 => {
            return String::from("99");
        }
        _ => { 
            return no.to_string()
        }
    };
}

fn plural_check(no: i32) -> String {
    match no {
        1 => {
            return String::from("");
        },
        _ => {
            return String::from("s");
        }
    }
}

fn what_to_do(no: i32) -> String {

    match no {
        0 => { 
            return String::from("Go to the store and buy some more")
        },
        1 => {
            return String::from("Take it down and pass it around")
        },
        _ => {
            return String::from("Take one down and pass it around")
        }
    }
}

pub fn verse(n: u32) -> String {
    let n_as_int = n as i32;

    return format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\n{}, {} bottle{} of beer on the wall.\n",
            bottle_no(n_as_int, true), 
            plural_check(n_as_int), 
            bottle_no(n_as_int, false), 
            plural_check(n_as_int), 
            what_to_do(n_as_int), 
            bottle_no(n_as_int-1, false), 
            plural_check(n_as_int-1), 
        )
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::from("");
    let mut i: i32 = start as i32;
    let j: i32 = end as i32;
    while i>=j {
        let i_min: i32 = i as i32;
        let i_min = i_min - 1;
        s = format!("{}{} bottle{} of beer on the wall, {} bottle{} of beer.\n{}, {} bottle{} of beer on the wall.",
            s,
            bottle_no(i as i32, true), 
            plural_check(i as i32), 
            bottle_no(i as i32, false), 
            plural_check(i as i32), 
            what_to_do(i as i32), 
            bottle_no(i_min, false), 
            plural_check(i_min), 
        );
        if i!=j {
            s = format!("{}\n\n", s);
        } else {
            s = format!("{}\n", s);
        }
        i=i-1;
    }
    
    return s
}
