use std::env;
use std::fs::{self, File};
use std::io::Write;


fn main() {
 
    let args: Vec<String> = env::args().collect();
    let filename = &args[1]; 

    let contents = readFile(filename.to_string());
    let mut v: Vec<&str> = Vec::new();
    let y = parseFile(&contents, &mut v).to_vec();
    writeToFile(&y);
}

fn read_File(filename : String) -> String{
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents
}

fn parse_File<'a>(contents: &'a std::string::String, v:&'a mut Vec<&'a str> ) -> &'a Vec<&'a str>{
    
    let mut x:i32;
    for line in contents.lines(){
       x=  line.trim().parse().expect("File does not contain numbers");
       
       if x%15 == 0{
        v.push("FizzBuzz");
       }
       else if x%5 == 0{
        v.push("Buzz");
       }
       else if x%3==0 {
        v.push("Fizz");
       }
       else {
           v.push(line);
       }
    }
    v
}

fn write_To_File(v: & Vec<&str>){
    let mut file = File::create("foo.txt").expect("Unable to create file");
    for i in v{                                                                                                                                                                  
        file.write_all((*i).as_bytes()).expect("Unable to write data");       
        file.write_all("\n".as_bytes()).expect("Unable to write data");                                                                                                                       
    }
    
}