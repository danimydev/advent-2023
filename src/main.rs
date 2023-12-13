mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let problem_1_1 = day_1::problem_1();
    println!("1_1: {}", problem_1_1);

    let problem_1_2 = day_1::problem_2();
    println!("1_2: {}", problem_1_2);

    let problem_2_1 = day_2::problem_1();
    println!("2_1: {}", problem_2_1);

    let problem_2_2 = day_2::problem_2();
    println!("2_2: {}", problem_2_2);

    let problem_3_1 = day_3::problem_1();
    println!("3_1: {}", problem_3_1);

    let problem_3_2 = day_3::problem_2();
    println!("3_2: {}", problem_3_2);

    let problem_4_1 = day_4::problem_1();
    println!("4_1: {}", problem_4_1);

    let problem_4_2 = day_4::problem_2();
    println!("4_2: {}", problem_4_2);
}
