use std::io;
use rand::{Rng};
use std::cmp::Ordering;
pub struct Guess;

impl Guess{
    pub fn do_guess(){

        println!("Guess the NUmber !" );
       
        let secrete_number=rand::thread_rng().gen_range(1,101);
      //  println!("Your Secrete number is {}",secrete_number);
        loop {
            println!("PLease enter the your guess no : ");
            let mut guess=String::new();

        io::stdin().read_line(&mut guess)
        .expect("failed to read line");
      

       

        let guess:u32=match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=> continue,
        };

        println!("You guessed {}",guess);

        match guess.cmp(&secrete_number) {

            Ordering::Less => println!("To small!"),
            Ordering:: Greater=> println!("To Large !"),
            Ordering::Equal=>{
                println!("You Win !");
                break;
            }

        }
        




        }
            }

}
