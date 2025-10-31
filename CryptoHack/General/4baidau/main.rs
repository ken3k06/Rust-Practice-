use std::fmt::format;
use std::vec;
use std::{io, str::from_utf8}; 
use rand::Rng; 


use std::str;


use base64::*;
use base64::{engine::general_purpose::STANDARD, Engine as _}; 
use num_bigint::BigUint;
use std::cmp::Ordering; 
pub fn start()-> String{
    println!("Guess the number");
    println!("Please input your guess. ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed");
    println!("You guessed : {}",guess);
    guess.trim().to_string()
}

fn bytes_to_long(bytes :&[u8]) -> BigUint{
    BigUint::from_bytes_be(bytes)
}
fn long_to_bytes(n:&BigUint) -> Vec<u8>{
    n.to_bytes_be()
}
fn rot13(s: &str) -> String{
    s.chars()
    .map(|c| match c{
        'a'..='z' => (((c as u8 - b'a') + 13) % 26 + b'a') as char, 
        'A'..='Z' => (((c as u8 - b'A') + 13) % 26 +b'A') as char, 
        other => other, 
    }).collect()
}
fn main(){
    let sec_num = rand::thread_rng().gen_range(1..=100);
    println!("Sanity check !"); 
    println!("số bí mật  = {}", sec_num);
    let guess = start();
    let guess: u32 = guess.trim().parse().expect("Nhập vào số nguyên"); 

    match guess.cmp(&sec_num){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You Win!"),
    }


    // do cryptography task 
    // bai 1: ascii value to flag 
    
    let a: Vec<u8> = vec![99, 114, 121, 112, 116, 111, 123, 65, 83, 67, 73, 73, 95, 112, 114, 49, 110, 116, 52, 98, 108, 51, 125];
    let s: String = a.into_iter().map(|x| x as char).collect(); 
    println!("{}", s); 

    // bai 2: hex to bytes

    let hex_val = "63727970746f7b596f755f77696c6c5f62655f776f726b696e675f776974685f6865785f737472696e67735f615f6c6f747d";
    let bytes = hex::decode(hex_val).unwrap();
    let flag = String::from_utf8(bytes).unwrap();
    println!("{}",flag); 

    // bai 3 decode base64 chuyen tu hex -> byte -> b64encode(byte)
    let hex_str = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";
    let bytes = hex::decode(hex_str).unwrap(); 
    let flag = STANDARD.encode(bytes);
    println!("{}",flag);
    // bai 4 
    let n = BigUint::parse_bytes(b"11515195063862318899931685488813747395775516287289682636499965282714637259206269",10).unwrap();
    let flag_bytes = long_to_bytes(&n);
    let flag: String = flag_bytes.into_iter().map(|x| x as char).collect();
    println!("{}",flag);
    
}
