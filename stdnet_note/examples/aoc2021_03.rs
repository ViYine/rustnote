



fn main () {
    let input = include_str!("fixtures/aoc2021_03.txt");
    let all_vec = input
    .lines()
    .map(|s|{
        s.chars().map(|c| {
            if c == '1' {
                1
            } else {
                0
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let one_len = all_vec[0].len();
    let mut e = vec![];
    let mut g = vec![];
    for i in 0..one_len {
        let mut c_nums = vec![];
        for j in all_vec.iter() {
            let cur = j[i];
            c_nums.push(cur);
        }
        let c1 = c_nums.iter().filter(|&x| *x == 1).count();
        let c0 = c_nums.iter().filter(|&x| *x == 0).count();
        if c1 > c0 {
            e.push(1);
            g.push(0);
        } else {
            e.push(0);
            g.push(1);
        }
    }
    // convert to string
    let e_str = e.iter().map(|x| x.to_string()).collect::<String>();
    let g_str = g.iter().map(|x| x.to_string()).collect::<String>();
    println!("e: {:?}", e_str);
    println!("g: {:?}", g_str);
    // convert to i32
    let e_i32 = i32::from_str_radix(&e_str, 2).unwrap();
    let g_i32 = i32::from_str_radix(&g_str, 2).unwrap();
    println!("e: {:?}", e_i32);
    println!("g: {:?}", g_i32);
    println!("Part1: {}", e_i32 * g_i32);
}
