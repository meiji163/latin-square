use std::fs;
use std::io;

const SIZE : usize = 7;
type Square = [[i32; SIZE]; SIZE];

fn convert_char(c: char) -> i32 {
    (c as i32) - ('a' as i32)
}

fn parse_square(s: &str) -> Option<Square> {
    let rows : Vec<[i32; SIZE]> = s.lines().map(|l| {
        l
            .chars()
            .map(convert_char)
            .collect::<Vec<i32>>()
            .try_into().unwrap()
    }).collect();
    if rows.len() != SIZE {
        None
    } else {
        Some ( rows.try_into().unwrap() )
    }
}

fn parse_squares(s: &str) -> Vec<Square> {
    s
        .split("\n\n")
        .filter_map(parse_square)
        .collect()
}

fn factorial(n: usize) -> usize {
    (1..=n).reduce(|acc, x| acc*x).unwrap()
}

// Generate list of (n!-1) swaps such that successive
// applications of the swaps generates all n-permutations.
// Uses Heap's algorithm (https://en.wikipedia.org/wiki/Heap%27s_algorithm)
fn generate_swaps(n: usize) -> Vec<(usize,usize)> {
    let nfact = factorial(n);
    let mut digits = vec![0; n];
    digits[n-1] = n+1; // sentinel
    let mut swps = vec![];
    for _i in 0..nfact {
        let mut j = 0;
        while digits[j] == j+1 {
            digits[j] = 0;
            j += 1;
        }
        if j == n-1 {
            break;
        }
        let x = if j+1 % 2 == 0 {
            0
        } else {
            digits[j]
        };
        swps.push((j+1,x));
        digits[j] += 1;
    }
    swps
}

fn apply_swap(sqr: &mut Square, swap: &(usize, usize)) {
    let &(i,j) = swap;
    (sqr[j], sqr[i]) = (sqr[i], sqr[j]);
    for k in 0..SIZE {
        (sqr[k][i], sqr[k][j]) = (sqr[k][j], sqr[k][i]);
    }
}

// Find a stacking of two edge colorings by brute-forcing permutation of the vertices
fn find_stacking(swaps: &Vec<(usize,usize)>, sqr1: &Square, sqr2: &mut Square) -> bool {
    // let mut p = [0; SIZE];
    // for i in 0..SIZE {
    //     p[i] = i;
    // }
    for swap in swaps.iter() {
        let mut found = true;
        'outer: for i in 0..SIZE {
            for j in (i+1)..SIZE {
                if sqr1[i][j] == sqr2[i][j] {
                    found = false;
                    break 'outer;
                }
            }
        }
        if found {
            return true;
        }
        apply_swap(sqr2, swap);

        // let &(i,j) = swap;
        // (p[i],p[j]) = (p[j],p[i]);
        // println!("{:?}", p);
    }
    return false;
}

fn print_square(sqr: &Square) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            print!("{}", sqr[i][j]);
        }
        print!("\n");
    }
    print!("\n");
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("symLS7isom")?;
    let squares = parse_squares(&input);

    let swaps = generate_swaps(SIZE);
    let len = squares.len();

    // loop over pairs of symmetric latin squares and verify there is a rainbow stacking
    let num_iters = (len*(len-1))/2;
    let mut itr = 0;
    for i in 0..len {
        for j in (i+1)..len {
            if itr%500000 == 0 {
                let pct_done = 100.0 * (itr as f64) / (num_iters as f64);
                println!("{}/{}({:.2}%)", itr, num_iters, pct_done);
            }
            let mut sqr1 = squares[i];
            let mut sqr2 = squares[j];

            let found = find_stacking(&swaps, &mut sqr1, &mut sqr2);
            if !found {
                println!("NO STACKING: i={} j={}", i, j);
                print_square(&sqr1);
                print_square(&sqr2);
                return Ok(());
            }
            itr += 1;
        }
    }
    Ok(())
}

#[test]
fn test_swaps(){
    assert_eq!(generate_swaps(4).len(), factorial(4)-1);
    assert_eq!(generate_swaps(7).len(), factorial(7)-1);
}
