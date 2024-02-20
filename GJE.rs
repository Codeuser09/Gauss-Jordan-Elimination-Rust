fn main () {
    let mut m = [[8.0, -8.0, -9.0, -1.0], [-10.0, 15.0, -9.0, -25.0], [-9.0, -1.0, 7.0, 3.0]];
    
    if m[0][0] == 0.0 {
	    for i in 0..m[0].len() {
	        m[0][i] += 1.0;
	    }
    }
    
    for c in 0..m.len() {
        let row_divider = m[c][c];
	    if row_divider != 1.0 {
		    for e in 0..m[c].len() {
		        m[c][e] = m[c][e] / row_divider;
		    }
	    }
        for r in 0..m.len() {
        let row_subtraction_multiplier = m[r][c];
    		if r != c {
    			for e in 0..m[r].len() {
    			    m[r][e] -= m[c][e] * row_subtraction_multiplier
    			}
        	}
        }
    }
    
    let last_index = m[0].len()-1;
    for i in 0..m[0].len() {
        println!("{}", m[i][last_index])
    }
}
