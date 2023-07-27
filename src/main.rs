use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("{}", "Input 3 arguments!")
    } else {
        let start: i64 = args[1].parse().unwrap();
        let end: i64 = args[2].parse().unwrap();
        let gap: i64 = args[3].parse().unwrap();
        let mut current_fhar;
        let mut current_celc: f64;
        if end - start > 0 {
            let times: i64 = (end - start) / gap + 1;
            println!("{}", "\tFahr\tCelcius");
            for loops in 0..times {
                current_fhar = start + gap * loops;
                current_celc = ((current_fhar as f64) - 32.0) * (5.0 / 9.0);
                println!("\t{}\t{:.1}", current_fhar, current_celc);
            }
        } else {
            let times: i64 = (start - end) / gap + 1;
            println!("{}", "\tFahr\tCelcius");
            for loops in 0..times {
                current_fhar = start - gap * loops;
                current_celc = ((current_fhar as f64) - 32.0) * (5.0 / 9.0);
                println!("\t{}\t{:.1}", current_fhar, current_celc);
            }
        }
    }
}
