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
____________________________________
enum Status{
    Active ,
    Inactive ,
}
impl Status{
    fn print_status_1(&self){
        match self{
            Status::Active => println!("He is active !") ,
            Status::Inactive => println!("He is incative !") ,
        }
    }
    fn print_status_2(&self){
        match self{
            Status::Active => println!("He is active !") ,
            Status::Inactive => println!("He is not active !") ,
        }
    }
}

fn main(){
    let status_1 : Status = Status::Active ;
    let status_2 : Status = Status::Inactive ;
    status_1.print_status_1() ;
    status_2.print_status_2() ;
}


___________________________________________________________
enum Data{
    Data1(i32 , i32) ,
    Data2(String , String) ,
}
fn main(){
    let data1 : Data = Data::Data1(12 , 22) ;
    let data2 : Data = Data::Data2(String::from("Arch") , String::from("Linux")) ;
    match data1{
        Data::Data1(value1 , value2) => println!("First int{:?} and second int is {:?}" , value1 , value2) ,
        Data::Data2(value1 , value2) => println!("First string is {:?} and second string is {:?}" , value1 , value2) ,
    }
    match data2{
        Data::Data1(value1 , value2) => println!("First int is {:?} and second int is {:?}" , value1 , value2) ,
        Data::Data2(value1 , value2) => println!("First string is {:?} and second string is {:?}" , value1 , value2) ,
    }
}
//maybe its a touple enum ?
__________________________________________________________
#[derive(Debug)]
struct Counter{
    value : i32 ,
}
impl Counter{
    fn inc(&mut self)-> i32{
        self.value = self.value + 1 ;
        println!("Value is now increased as {:?}" , self.value) ;
        return 0 as i32 ;
    }
    fn dec(&mut self)-> i32{
        self.value = self.value - 1 ;
        println!("Value is now decreased as {:?}" , self.value) ;
        return 0 as i32 ;
    }
    fn print_value(&self){
        println!("the value is {:?}" , self.value) ;
    }
}
fn main(){
    let mut value1 : Counter = Counter{
        value : 22 ,
    } ;
    value1.inc() ;
    value1.dec() ;
    value1.print_value() ;
    println!("Value 1 is {:?}" , value1) ;
}

__________________________________________________________________
fn main(){
    let mut word : String = String::from("Hello Noob") ;
    let big_word : &str = give_ref_not_own(&mut word) ;
    println!("The big word is {:?}" , big_word) ;
}
fn give_ref_not_own(value : &mut String)-> &str{
    value.push_str(" Again Hello world") ;
    return value.as_str() ;
}
//simple but very much tricky ; 
______________________________________________________
fn main(){
    let string_data : String = String::from("Hello Arch") ;
    let str_data : &str = "hello noob";
    print_string(&string_data) ;
    print_str(&str_data) ;
    //both string_data and str_data also alive because we just gave the reference only ; 
}
fn print_string(value : &String){
    println!("The string value is {:?}" , value) ;
}
fn print_str(value : &str){
    println!("The &str value is {:?}" , value) ;
}
_______________________________________________________











































