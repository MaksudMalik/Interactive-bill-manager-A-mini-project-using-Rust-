use std::collections::HashMap;
use std::io;

pub fn add_bill(bills:&mut HashMap<String, f64>) {

    println!("Name : ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    println!("Amount : ");
    loop{
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Failed to read line");
        let amount:Result<f64,_> = amount.trim().parse();
        match amount {
            Ok(n)=>{
                bills.insert(name,n);
                break;
            }
            Err(_) => {
                println!("Enter a number");
            }
        }
    }
    println!("\nBill added!\n");
}

pub fn view_bill(bills:&mut HashMap<String, f64>){

    if bills.is_empty(){
        println!("\nNo bills for now");
    }
    else{
        println!("\nAvailable bills :");
        for (name, amount) in bills{
            println!("{}: {}",name, amount);
        }
    }
    println!("\n");
}

pub fn remove_bill(bills:&mut HashMap<String, f64>){

    println!("Enter the name : ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    if bills.get(&name)==None{
        println!("\nNo bill found");
    }
    else{
        bills.remove(&name);
        println!("\nBill removed!");
    }
    println!("\n");
}

pub fn update_bill(bills:&mut HashMap<String, f64>) {

    println!("Enter the bill name : ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name = name.trim().to_string();

    if bills.get(&name)==None{
        println!("\nNo bill found");
    }
    else{
        println!("Enter the new amount :");

        loop{
            let mut amount = String::new();
            io::stdin().read_line(&mut amount).expect("Failed to read line");
            let amount:Result<f64,_> = amount.trim().parse();
            match amount {
                Ok(n)=>{
                    bills.insert(name,n);
                    println!("\nBill updated!\n");
                    break;
                }
                Err(_) => {
                    println!("Enter a number");
                }
            }
        }
    }
}