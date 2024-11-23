fn main() {
    println!("Hello, world!");

    let mut n: i32 = 10;
    let Pi = 3.14;
    println!("the number is {}",n);
    println!("Value of Pi is {}",Pi);

    let char = "N";
    println!("Character is {}",char);

    let float: f64 = 5.5;
    println!("float value is {}",float);

    let my_bool:bool = true;
    println!("value of bool is {}", my_bool); 
    
    const cs: i32 = 10;
    println!("value of const is {}",cs);
    let mut my_name: String = String::from("ABCD");
    let friend_name = "Second Way To Define String".to_string();
    println!("my name is {}",my_name); 
    println!("my name in lowerCase {}",friend_name);

    let mut my_vec: Vec<u64> = Vec::new();
    my_vec.push(1);
    println!("my vector/array is {:?}", my_vec); 
    my_vec.push(2);
    println!("my vector/array is {:?}", my_vec); 
    my_vec.push(my_vec[0]+my_vec[1]);

    // let vec_length: i32 = my_vec.len():
    println!("my vector/array is {:?} and length is {}", my_vec, my_vec.len()); 
    

    struct Rectangle {
        length :u32,
        width: u32
    };
    let my_rectangle : Rectangle = Rectangle{
        length: 10,
        width: 15,
    };
    println!(" lenght and width of my rectangle is {} and {}", my_rectangle.length,my_rectangle.width);

    let a:u32 = 10;
    let b:u32 = 20;
    println!("a={},b={}",a,b);
    let my_sum: u32 = sum(10,20);
    println!("sum of {}+{}={}",a,b,my_sum);
    let my_sub: u32 = sub(20,10);
    println!("sub of {}-{}={}",b,a,my_sub);
    let my_mul:u32 = mul(5,5);
    println!("mul of {}*{}={}",a,b,my_mul);
    println!("div is {}/{}={}",b,a,div(10,2));
    println!("mod of {} and {} is {}",b,a,modulo(b,a));

    let old_str: String = String::from("Example of ownership.");
    let new_str: String = old_str;
    // println!("old string is -> {}",old_str); //Will trow Error
    println!("new string is -> {}",new_str);

    if(a>b){
        println!("a is greater");
    }else{
        println!("b is greater");
    }
}

fn sum(a: u32, b: u32) -> u32 {
    // return a+b;
    let c = a+b;
    return c;
}
fn sub(a:u32, b:u32) -> u32{
    return a-b;
}
fn mul(a:u32, b:u32) -> u32{
    return a*b;
}
fn div(a:u32,b:u32) -> u32{
    a/b
}
fn modulo(a:u32,b:u32)-> u32{
    a%b
}