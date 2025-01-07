use std::env;

mod problems;
use problems::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let arg = args.get(1).map(String::as_str).unwrap_or("-1");

    let problem_id = arg.parse::<i32>();

    let result = match problem_id {
        Ok(1) => problem1::problem1(),
        Ok(2) => problem2::problem2(),
        Ok(3) => problem3::problem3(),
        Ok(4) => problem4::problem4(),
        Ok(5) => problem5::problem5(),
        Ok(6) => problem6::problem6(),
        Ok(7) => problem7::problem7(),
        Ok(8) => problem8::problem8(),
        Ok(9) => problem9::problem9(),
        Ok(10) => problem10::problem10(),
        Ok(11) => problem11::problem11(),
        Ok(12) => problem12::problem12(),
        Ok(13) => problem13::problem13(),
        Ok(14) => problem14::problem14(),
        Ok(16) => problem16::problem16(),
        Ok(17) => problem17::problem17(),
        Ok(18) => problem18::problem18(),
        Ok(19) => problem19::problem19(),
        Ok(20) => problem20::problem20(),
        Ok(21) => problem21::problem21(),
        Ok(22) => problem22::problem22(),
        Ok(23) => problem23::problem23(),
        Ok(25) => problem25::problem25(),
        Ok(26) => problem26::problem26(),
        Ok(27) => problem27::problem27(),
        Ok(28) => problem28::problem28(),
        Ok(29) => problem29::problem29(),
        Ok(30) => problem30::problem30(),
        Ok(31) => problem31::problem31(),
        Ok(32) => problem32::problem32(),
        Ok(33) => problem33::problem33(),
        Ok(34) => problem34::problem34(),
        Ok(35) => problem35::problem35(),
        Ok(36) => problem36::problem36(),
        Ok(37) => problem37::problem37(),
        Ok(38) => problem38::problem38(),
        Ok(39) => problem39::problem39(),
        Ok(40) => problem40::problem40(),
        Ok(41) => problem41::problem41(),
        Ok(42) => problem42::problem42(),
        Ok(43) => problem43::problem43(),
        Ok(44) => problem44::problem44(),
        Ok(45) => problem45::problem45(),
        Ok(46) => problem46::problem46(),
        Ok(48) => problem48::problem48(),
        Ok(49) => problem49::problem49(),
        Ok(50) => problem50::problem50(),

        _ => panic!("Invalid problem ID"),
    };

    println!("Problem {}: {}", problem_id.unwrap(), result);
}
