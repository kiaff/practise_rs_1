//This practice set covers some of the basic example of enum , struct , impl , Strings , intigers and so on 
______________________________________Example of enum 
enum Dir{
    Left ,
    Right ,
}
fn main(){
    let dir_1 : Dir = Dir::Left ;
    match dir_1{
        Dir::Left => println!("This is left !") ,
        Dir::Right => println!("This is right !") ,
    }
    let dir_2 : Dir = Dir::Right ;
    match dir_2{
        Dir::Left => println!("This is left !") ,
        Dir::Right => println!("This is right !") ,
    }
}

_______________________________________
//learning another enum code example with value 
enum DataSet{
    Data1(String) ,
    Data2(i32) ,
}
fn main(){
    let value_1 : DataSet = DataSet::Data1(String::from("Hello Kiaff")) ;
    let value_2 : DataSet = DataSet::Data2(22 as i32) ;
    match value_1{
        DataSet::Data1(value) => println!("The first data is a String :{:?}" , value) ,
        DataSet::Data2(value) => println!("The second data is a int :{:?}" , value) ,
    }
    match value_2{
        DataSet::Data1(value) => println!("The first data is a string is {:?}" , value) ,
        DataSet::Data2(value) => println!("The second data is a 32 bit intiger is {:?}" , value) ,
    }
}
________________
string and str

fn main(){
    let mut string :  String =  String::from("Hello ") ;
    string.push_str(" world") ;
    println!("the string is {:?}" , string) ;
}
______________________
fn main(){
    let word : &'static str = "hello world" ;
    println!("This is a borrowed string slice type {:?}" , word) ;
}
//the word veriable will be valid till the end of the programme .....
___________
fn main() {
    let s: &str = "Hello World";

    println!("{}", s);
}


the simple thing is here s is a &str means that s is borrowing the string ...ok but form where does it borrowing ? 
simply s is just taking the reference of the words "hello world" ..and explicit type is &'static str 

__________________________________________________________
#[derive(Debug)]
struct Rec{
    len  : i32 ,
    wid  : i32 ,
}
impl Rec{
    fn area(&self)-> i32{
        if self.wid <= 0{
            eprintln!("Please try with a valid width !") ;
            return 0 as i32 ;
        }else{
            return self.len * self.wid  ;
        }
    }
}
fn main(){
    let rec1 : Rec = Rec{
        len : 32 ,
        wid : 342 ,
    } ;
    println!("Rec1 is {:?}" , rec1) ;
    let area : i32 = rec1.area() ;
    println!("The area of the  Rectangle is {:?}" , area) ;
}
















































