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

















































