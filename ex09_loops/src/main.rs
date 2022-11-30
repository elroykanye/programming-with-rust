fn main() {
    // conditioned loops - while
    let mut i = 0;
    while i < 10 {
        
        
        i += 1;

        if i > 5 && i < 7 {
            continue;
        }

        // println!("{}", i);
    }

    // infinite loops - loop

    let mut j = 0;
    loop {
        if j > 10 {
            break;
        }

        // println!("{}", j);
        j += 1;
    }

    // counting loops - for
    for i in 0..11 { // 0 <= i < 10
        println!("{}", i * 14);
    }
}

/*
for (int i = 0; i < 10; i++) {}
for (let i = 0; i < 10; i++) {}
 */