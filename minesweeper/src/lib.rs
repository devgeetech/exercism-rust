pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    match minefield.len() {
        0 => return vec![],
        height => {
            let width = minefield[0].len();
            if width == 0 {
                return vec![("".to_string())]
            }
            for i in 0..height {
                output.push("".to_string());
                minefield[i].chars().enumerate().for_each(|(j, elem)| {
                    if elem == '*' {
                        output[i].push(elem);
                    } else {
                        let mut mine_count: u8 = 0;

                        // Arrays defining which positions to be scanned for mines
                        let mut pop_rows: Vec<i8> = vec![-1, 0, 1];
                        let mut pop_cols: Vec<i8> = vec![-1, 0, 1];

                        if i==0 { // Don't scan top positions
                            pop_rows.remove(pop_rows.iter().position(|x| *x == -1).expect("not found"));
                        }
                        if i==height-1 { // Don't scan bottom positions
                            pop_rows.remove(pop_rows.iter().position(|x| *x == 1).expect("not found"));
                        }
                        if j==0 { // Don't scan left positions
                            pop_cols.remove(pop_cols.iter().position(|x| *x == -1).expect("not found"));
                        }
                        if j==width-1 { // Don't scan right positions
                            pop_cols.remove(pop_cols.iter().position(|x| *x == 1).expect("not found"));
                        }

                        // Counting of mines
                        for x in pop_rows {
                            let row_val = (i as i8 + x) as usize;
                            for  y in &pop_cols {
                                let col_val = (j as i8 + *y) as usize ;
                                if is_mine(minefield[row_val].chars().nth(col_val).unwrap()) {
                                    mine_count += 1;
                                }
                            }
                        }

                        match mine_count {
                            0 => output[i].push(' '),
                            not_zero => {
                                let not_zero_str = not_zero.to_string();
                                output[i].push_str(&not_zero_str[..])
                            }
                        }
                    }
                });
            }
        }
    };
    
    output
}

fn is_mine(symbol: char) -> bool {
    symbol=='*'
}