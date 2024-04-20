fn arrays() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3];

    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];

    let arrays = [one, two, blank1, blank2];

    for a in &arrays {
        print!("{:?}: ", a);

        for n in a.iter() {
            print!("\t {} + 10 = {}", n, n + 10);
        }

        let mut sum = 0;

        for i in 0..a.len() {
            sum += a[i];
        }

        println!("\t(sum{:?} = {})", a, sum);
    }
}

fn vec() {
    let ctx_lines = 2;
    let needle = "oo";
    let haystack = "\
Every face, every shop, bedroom window, public-house, and dark square is a picture feverishly turned--in search of what? It is the same with books. What do we seek through millions of pages?";

    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in haystack.lines().enumerate() {
        if line.contains(needle) {
            tags.push(i);

            let v = Vec::with_capacity(2 * ctx_lines + 1);
            ctx.push(v);
        }
    }

    if tags.is_empty() {
        return;
    }

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                ctx[j].push((i, line.to_string()));
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", i, line);
        }
    }
}

fn main() {
    arrays();
}
