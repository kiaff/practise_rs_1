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


















































