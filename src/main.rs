mod To {           // Khai báo module To mà không cần file mod.rs
    pub mod test;  // Khai báo module test trong thư mục To
    pub mod array;
}

use std::{io, iter::Scan};

use To::test;      // Sử dụng module test từ To
use To::array;
mod t;
fn main() {
    let tup  = (1,2,3.3);
    let hu = tup.2;
    let (x,y,z) = tup;
    // print!("Các tup lần lượt là {x}, {y}, {hu}");
    let a = [1,2,3,4,5];
    let b = a[0];
    // let mut input = String::new();
    // println!("Nhập giá trị vào");
    // io::stdin().read_line(&mut input).expect("không thể đọc");
    // let a = input.trim().parse().expect("không thể đọc");
    // t::te(a);
    array::ar();
}