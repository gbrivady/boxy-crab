type Grid = Vec<Vec<bool>>;

fn build_hints(g: Grid) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut row_hints: Vec<Vec<i32>> = Vec::new();
    // let mut col_hints: Vec<Vec<i32>> = vec![Vec::new(); g[0].len()];
    let mut col_counts: Vec<i32> = vec![0; g[0].len()];
    for row in g.iter() {
        let mut cur_hint: Vec<i32> = Vec::new();
        let mut cur_count: i32 = 0;
        for cell in row.iter().enumerate() {
            match (cell, cur_count) {
                ((j, &true), _) => {
                    cur_count += 1;
                    col_counts[j] += 1;
                }
                ((j, &false), 0) => (),
                ((j, &false), _) => {
                    cur_hint.push(cur_count);
                    cur_count = 0;
                }
            }
        }
        if cur_count != 0 {
            cur_hint.push(cur_count)
        }
        row_hints.push(cur_hint);
    }
    return (row_hints, vec![vec![0]; 1]);
}

fn main() {
    println!(
        "{:#?}",
        build_hints(vec![
            vec![true, true, false, true, true, false],
            vec![true, true, false, false, false, true]
        ])
        .0
    )
}
