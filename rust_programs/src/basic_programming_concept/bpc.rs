pub struct BPC;

impl BPC {
    pub fn do_basic_prorams() {
        basic_operations();
        let add = another_function(5);
        println!("additon of on number and next number is :{}", add);
        control_flow();
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
