fn main () {
    let mut m = [[8.0, -8.0, -9.0, -1.0], [-10.0, 15.0, -9.0, -25.0], [-9.0, -1.0, 7.0, 3.0]];
    
    if m[0][0] == 0.0 {
	    for i in 0..m[0].len() {
	        m[0][i] += 1.0;
	    }
    }
    
    for c in 0..m.len() {
	    if m[c][c] != 1.0 {
		    for e in 0..m[c].len() {
		        m[c][e] = m[c][e] / m[c][c];
		    }
	    }
        for r in 0..m.len() {
    		if r != c {
			for e in 0..m[r].len() {
			    m[r][e] -= m[c][e] * m[r][c]
			}
        	}
        }
    }
    
    for element in m {
        for element_ in element {
            print!("{}", element_);
        }
        print!("\n");
    }
}
