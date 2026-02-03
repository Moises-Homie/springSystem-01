fn double(x:i32) -> i32 {
    //return x*2;
    {
        let y = 10;
        x*2*y
    }
}

//fn say_hi(x:i32){
    //println!("Hi John!My fav num {}",x);
//}

fn main() {

    println!("Double {} equals to {} ", 5,double(5));

    //say_hi(5);

    /*let x = 5;
    {
        let x = x + 10;
        println!("x:{}", x);
    }

    println!("x: {}", x); */
    // Shadowing
    //let x = 5;
    //let x = x + 1;  // Creates a new variable
    
    // Mutation
    //let mut y = 5;
    //y = y + 1;  // Modifies the existing variable
    
    //println!("x: {}, y: {}", x, y);


    //let mut x:i32 = 5;
    //x = 1.012;// you can't

    //let x:f32 = x as f32 + 1.012;
    //println!("{}", x);




    //let mut result : f32 = 0.0; //int
    //let x:i32 = 5; //float
    //result = result + x as f32; //no implicit conversion

    //println!("{}", result);


    /*//let mut x = 10; // int8, int16, int32, int64
    //x += 10;

    //println!("5*2 + 10 = {}", x); */
}
