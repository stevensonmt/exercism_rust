const MY_CONST: &'static [&'static str] = &["Pling", "Plang", "Plong"];
const FACTORS: &'static [i32] = &[3, 5, 7];

pub fn raindrops(num: i32) -> String {
    let mut mydrops = "".to_string();
    for i in FACTORS {
        let ndx = FACTORS.iter().position({|&j| j == *i }).unwrap();
        if num % i == 0 {
            let drip = MY_CONST[ndx].to_string();
            mydrops.push_str(&drip);
        }
    }
    if mydrops.len() > 1 {
        mydrops
    } else {
        num.to_string()
    }
}
