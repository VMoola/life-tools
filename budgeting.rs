fn main()
{
    //values
    let mut line = String::new();
    println!("Enter your salary :");
    let _err = std::io::stdin().read_line(&mut line);

    let salary : f64 = line.trim().parse().unwrap();

    //percentages (ceil-ed for simplicity)
    let espp = 10.0;
    let pre_tax = 20.0;
    let post_tax = 20.0;

    let fed_tax = 13.0;
    let social_security = 0.0;
    let medicare = 3.0;
    let ca_tax = 5.0;
    let ca_disability = 1.0;

    let cost = espp + pre_tax + post_tax + fed_tax + social_security + medicare + ca_tax + ca_disability;
    let to_hand = salary * (1.0 - cost / 100.0);


    println!("You use {cost}% of your paycheck on payroll deductions, the remaining ${to_hand} is yours to spend");
}
