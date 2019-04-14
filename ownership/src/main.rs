
fn main() 

{
    // it's an example of transfering ownership in intiger
    println!("copying Ownership in Intiger.");
    let x = 5;
    let y = x;
    println!("the Value of {}",x);
    println!("the Value of {}",y);
    
    // it's an example of Transfering Ownership in String
    println!("Transfering Ownership in String.");
    let s1 = String::from("hello");
    let s2 = s1;
    println!("the Value of {}",s2);
    
    println!("Transfering Ownership in String using Clone(),");
    let a = String::from("Hello");
    let b = a.clone();
    println!("a = {}, b = {}",a , b);

}