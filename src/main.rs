// Project 1: Interactive bill manager

use std::collections::HashMap;
use std::io;
mod functions;
use functions::*;
fn main() {

    let mut bills:HashMap<String, f64> = HashMap::new();

    loop{    
        println!("==Bill Manager==");
        println!("1. Add Bills");
        println!("2. View Bills");
        println!("3. Remove Bills");
        println!("4. Update Bills\n");
        println!("Enter Selection");

        loop {
            
            let mut selection = String::new();
            io::stdin().read_line(&mut selection).expect("Failed to read the line");
            let mut selection:Result<u32, _> = selection.trim().parse();
            match selection{
                Ok(n)=>selection=Ok(n),
                Err(_)=>{
                    println!("Enter a number");
                    continue;
                }
            }

            match selection {
                Ok(1) =>{
                    add_bill(&mut bills);
                    break;}

                Ok(2)=>{
                    view_bill(&mut bills);
                    break;}

                Ok(3)=>{
                    remove_bill(&mut bills);
                    break;}

                Ok(4)=>{
                    update_bill(&mut bills);
                    break;}

                _=>{
                    println!("Enter a valid selection");
                    continue;
                }
            }
        }
    }
}
