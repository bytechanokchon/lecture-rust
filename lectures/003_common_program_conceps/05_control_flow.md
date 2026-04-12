# Control Flow
เป็นการควบคุมการทำงานของชุดคำสั่งตามเงื่อนไขที่กำหนด

## `if` Expression
คำสั่ง `if` ช่วยให้เราสามารถเขียนเงื่อนไขในการทำงานของชุดคำสั่งได้ โดย *ถ้าตรงเงื่อนไขที่กำหนด ชุดคำสั่งจึงจะทำงาน* ถ้าไม่ตรงกับเงื่อนไข *จะรันชุดคำสั่งอื่น*

    fn main() {
        let number: i32 = 3;

        if number < 5 {
            println!("Condition was true");
        } else {
            println!("Condition was false");
        }
    }

ชุดคำสั่งดังกล่าว
- ตรวจสอบว่า `number` มีค่าน้อยกว่า 5 หรือไม่
- หากเป็น `true` จะแสดงข้อความ *Condition was true*
- หากเป็น `false` จะแสดงข้อความ *Condition was false*

*บางครั้ง เงื่อนไขที่เราตั้งไว้ใน `if` จะถูกเรียกว่า `arms` (แบบเดียวกับ match)*

*เงื่อนไขจะต้องเป็นชนิดข้อมูลแบบ `boolean`*

## `else` Expression
เป็นคำสั่งที่จะทำงาน เมื่อเงื่อนไขของ `if` เป็นเท็จทั้งหมด

## จัดการหลายเงื่อนไขด้วย `if` และ `else`
เราสามารถใช้งานหลายเงื่อนไขได้ด้วยการรวม `if` และ `else` เข้าด้วยกันด้วยคำสั่ง `else if` expression

    let number_two: i32 = 6;

    if number_two % 4 == 0 {
        println!("number is divisible by 4");
    } else if number_two % 3 == 0 {
        println!("number os dibisible by 3");
    } else if number_two % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

เมื่อชุดคำสั่งทำงาน มันจะตรวจสอบเงื่อนไขของ `if` ทีละอัน และจะดำเนินการชุดคำสั่งด้านในเฉพาะอันที่เงื่อนไขเป็นจริงอันแรก

*การใช้งาน `else if` ที่มากเกินไปอาจทำให้โปรแกรมของเรารก หากมีเงื่อนไขเยอะ แนะนำให้ใช้ `match`*

## การใช้ `if` ใน `let` statement
เนื่องจาก `if` เป็น statement เราจึงสามารถใช้งานร่วมกับ `let` เพื่อจัดเก็บผลลัพธ์ของเงื่อนไขไว้ในตัวแปรได้

    let condition = true;
    let number_three = if condition { 5 } else { 6 };
    println!("The value of number is: {number_three}");

ตัวแปร `number_three` จะถูกกำหนดค่าตามเงื่อนไขที่กำหนด

**สำคัญ** ข้อมูลที่อยู่ในแต่ละส่วนของ `if` จะต้องเป็นชนิดเดียวกัน หากเป็นคนละชนิดจะเกิดข้อผิดพลาด

    let condition = true;
    let number = if condition { 5 } else { "six" };
    println!("The value of number is: {number}");

ชุดคำสั่งดังกล่าว หากเงื่อไขเป็นจริงจะส่งคืน 5 ซึ่งเป็น `i32` แต่หากเป็นเท็จจะส่งคืน "six" ซึ่งเป็น `string` ทำให้เกิดข้อผิดพลาด

    Compiling branches v0.1.0 (file:///projects/branches)
    error[E0308]: `if` and `else` have incompatible types
     --> src/main.rs:4:44
      |
    4 |     let number = if condition { 5 } else { "six" };
      |                                 -          ^^^^^ expected integer, found `&str`
      |                                 |
      |                                 expected because of this

## Repetition with Loops
การวนซ้ำเพื่อทำงานชุดคำสั่งเดิม สามารถทำได้ด้วยการใช้คำสั่ง `loop`, `while`, `for`

### `loop` Expression
เป็นการสั่งวนซ้ำชุดคำสั่งไปเรื่อย ๆ อาจวนซ้ำ **ตลอดไป** หรือ **จนกว่าเราจะสั่งให้หยุด**

    loop {
        println!("again!");
    }

เมื่อเรารันคำสั่ง จะเห็นข้อความ "again!" พิมพ์ซ้ำไปเรื่อย ๆ จนกว่าเราจะสั่งหยุด โดยเราสามารถสั่งหยุดได้ด้วยการกดปุ่ม `Ctrl` + `C`

นอกจากนี้ เราสามารถใช้งานคำสั่ง `break` เพื่อสั่งให้หยุดการทำงานของ `loop` และสามารถใช้งานคำสั่ง `continue` เพื่อข้ามการวนซ้ำไปรอบถัดไป

### การรับค่าคืนจาก `loop`
ในบางครั้ง เราอาจใช้ลูปเพื่อทำซ้ำการทำงานที่อาจล้มเหลว เช่น การตรวจสอบว่าเธรดทำงานเสร็จแล้วหรือยัง โดยอาจต้องการผลลัพธ์ของการทำงานนั้นออกจากลูปไปยังส่วนที่เหลือของโปรแกรม โดยสามารถส่งคืนผ่าน `break` ที่เราใช้เพื่อหยุดการวนซ้ำ

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    }

    println!("The result is {result}");

สามารถใช้งานคำสั่ง `return` ได้เช่นกัน แต่จะเป็นการออกจากฟังก์ชันที่ลูปนี้กำลังทำงาน

### Disambiguating with Loop Labels