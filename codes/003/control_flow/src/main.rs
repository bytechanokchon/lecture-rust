fn main() {
    let mut count: i32 = 0;

    'counting_up: loop {
        println!("Count: {count}");

        let mut remaining: i32 = 10;

        loop {
            println!("Remaining: {remaining}");

            if remaining == 9 {
                break; // หยุดการทำงานลูปปัจจุบัน
            }

            if count == 2 {
                break 'counting_up; // หยุดการทำงานลูปชื่อ countung_up               
            }

            remaining -= 1;
        }

        count += 1;
    }

    println!("End count: {count}");
}
