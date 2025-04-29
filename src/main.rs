use std::io::{self, Write}; // thêm Write để flush
fn main() {
    let mut input = String::new();

    println!("Nhập tên của bạn");

    io::stdin()
        .read_line(&mut input)
        .expect("Lỗi khi đọc dòng");
    println!("Chào bạn {} \n ",input.trim());
     // Dừng lại đợi Enter
     println!("Nhấn Enter để thoát...");
     let mut dummy = String::new();
     io::stdin().read_line(&mut dummy).unwrap();
}