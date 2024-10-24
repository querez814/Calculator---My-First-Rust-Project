mod operations;
use std::io;
use std::result;

use operations::add;

use operations::sub;
use operations::mult;
use operations::div;
use operations::sub::subtract;

fn main() {
    let mut x = String::new();
    println!("Input the first number!");
    io::stdin().read_line(&mut x).expect("Not a valid input");

    let x: f64 = x.trim().parse().expect("A number, cmon idiot...");

    println!("The number is {}", x);

    let mut y: String = String::new();
    println!("Input the second number!");
    
    io::stdin().read_line(&mut y).expect("Please put a damned number in this is a calculator");

    let y:f64 = y.trim().parse().expect("DUDE IT IS A CALCULATOR U MESSED UP WTFFFFF");

    println!("The number is (inp 2) {}", y );

    let mut choice = String::new();
    println!("Pick an operation: \n1. Add \n2. Subtract \n3. Multiply \n4. Divide");

    io::stdin().read_line(&mut choice).expect("Please Enter a valid selection");

    let user_choice: i32 = choice.trim().parse().expect("Please enter a valid selection");

    /* This is the creation of our "Menu" for lack of better word */
    match user_choice{
        1=>{
            let result = add::addition(x, y);
            println!("The result of the performed addition operation on your inputs is {}", result);
        },

        2=>{
            let result:f64 = sub::subtract(x, y);
            println!("The result of the performed subtraction operation on your inputs is {}", result);
        },

        3=>{
            let result = mult::multiply(x, y);
            println!("The result of the performed multiplication operation on your inputs is {}", result);
        },

        4=>{
            let result = div::divide(x, y);
            println!("The result of the performed division operation on your inputs is {}", result);
            
        },

        _ => {
            println!( "Invalid input, try once more and read the damn insturctions u idiot im the idiot i cant even spell instructions"

            );
        }


    }


}





   


