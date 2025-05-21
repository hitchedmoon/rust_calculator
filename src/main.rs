use std::io;

fn calculate_something(num1: f64, num2: f64, operator: &str) -> f64 {
    if operator == "*" {
        num1 * num2
    } else if operator == "/" {
        num1 / num2
    } else if operator == "-" {
        num1 - num2
    } else if operator == "+" {
        num1 + num2
    } else {
        num1 + num2 // otherwise the stupid compiler wont let me call it even if I delete the last stupid else if idk why :((
    }
}

fn main() {

    println!("Welcome to the best calculator in rust that you will ever see;) xD!!!!");

    let mut input1 = String::new();
    println!("Gib die erste zahl ein:");
    io::stdin().read_line(&mut input1).expect("Fehler beim Lesen");
    let zahl1: f64 = input1.trim().parse().expect("Keine Zahl du.....!!!!!!!!");

    let mut input2 = String::new();
    println!("Gib der Operator ein: * / + -");
    io::stdin().read_line(&mut input2).expect("Fehler beim Lesen");
    let operator = input2.trim();

    let mut input3 = String::new();
    println!("Gib die zweite Zahl ein:");
    io::stdin().read_line(&mut input3).expect("Fehler beim Lesen");
    let zahl2: f64 = input3.trim().parse().expect("Keine Zahl du......!!!!!!!");

    if operator == "*" || operator == "/" || operator == "+" || operator == "-" {
        let res = calculate_something(zahl1, zahl2, operator);
        println!("Das resultat ist: {}", res);
    } else {
        println!("Kein Valid Operator: {}", operator);
    }
}

