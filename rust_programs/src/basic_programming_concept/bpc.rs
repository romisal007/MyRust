use std::io;
pub struct BPC;

impl BPC {
    pub fn do_basic_prorams() {
        basic_operations();
        let add = another_function(5);
        println!("additon of on number and next number is :{}", add);
        control_flow();

        do_temerature_convrsion();
    }
}

fn basic_operations() {
    //variable
    let x = 5;
    let x = x + 1;
    println!("variable x is {}", x);

    //Tuple
    let tup = (500, 4, 1);

    println!("tuple values are {:#?}", tup);
    //array
    let arr = [10, 24, 35, 44];
    let first = arr[0];
    println!("array first value is  {}", first);
}

fn another_function(x: i32) -> i32 {
    println!("Another Function value is {}", x);
    let y = { x + 1 };
    let z = x + y;
    z
}
fn control_flow() {

    let num=5;
    
    if num!=0{
        println!("number is {}",num );
    }
    else {
        println!("number is zero " );
    }

    let a=[10,20,3,40,50];
    let mut i=0;
    while i<5 {
        println!("in while loop arrrays value {} at index {}",a[i],i );
        i=i+1;
    }

    for element in a.iter(){
        println!("For loop array elenments are {}",element );
    }



}

fn do_temerature_convrsion(){

    println!("Choose Operation ");

    println!("Press 1 to convert Fahrnite to celsius");
    println!("Press 2 to convert celsius to fahrnite");

    let mut  opt=String ::new();
    io::stdin().read_line(&mut opt).expect("failed to read line");

   let  opt :u32=  opt.trim().parse().expect("failed to parse Please enter valid oprion");

    if opt==1{
        do_fahrneite_to_celeious();
    }
    else if opt==2{
        do_celcious_to_fahrneite();
    }
    else {
        println!("pLease enter valid option");
    }

}
fn do_fahrneite_to_celeious(){

 loop {
     
 let mut temp_in_f = String :: new();
 
 println!("Please enter temp in F : " );
    io::stdin().read_line(&mut temp_in_f)
    .expect("failed to read line");

  let degree_f:f32 = match temp_in_f.trim().parse(){
      Ok(degree_f)=>degree_f,
      Err(_)=>continue,
  };
  
  let degree_temp:f32= (degree_f - 32.0)*(5.0/9.0);
  println!("temp of fahrneite {} in degree celcious is {}",temp_in_f,degree_temp );
  break;
 }

   

}

fn do_celcious_to_fahrneite(){

    loop {
        let mut temp_in_c=String::new();
        println!("Please enter the temperature in C :" );
    
        io::stdin().read_line(& mut temp_in_c).expect("failed to read line");
        let degree_c:f32=match temp_in_c.trim().parse(){
            Ok(degree_c)=>degree_c,
            Err(_)=>continue,
        };
    
        let degree_in_f:f32=(degree_c*9.0/5.0) + 32.0;
        println!("Temperature of celsius {} in fahrnite is {}",temp_in_c,degree_in_f );
        break;
    }
  
}