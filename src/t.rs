pub fn te(a: i32) {
    let mut counter:i64 = 0;
    if a != 0 {
        println!("a khác 0, giá trị: {}", a);
        loop {
            print!("Chạy vô hạn! ");
            counter += 1;
            if counter == 10000{
                print!("Dừng lại đây! ");
                break;
            }
        }
    } else {
        println!("a bằng 0");
        loop {
            print!("chào thằng ku ");
            counter += 1;
            if counter == 10000{
                print!("Dừng lại đây! ");
                break;
            }
        }
    }
}
