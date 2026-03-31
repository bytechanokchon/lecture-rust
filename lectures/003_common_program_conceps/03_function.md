### Function

*rust ใช้รูปแบบ snack_case เป็นมาตรฐานในการตั้งชื่อฟังก์ชันและตัวแปน*

    fn main() {
        println!("Hello, world!");

        another_function();
    }

    fn another_function() {
        println!("Another_function.");
    }

ภาษา rust สามารถสร้างฟังก์ชันโดยการใช้คำสั่ง `fn` ตามด้วยชื่อฟังก์ชันและวงเล็บปีกกา วงเล็บปีกกาจะบอกคอมไพเลอร์ว่าส่วนเนื้อหาของฟังก์ชันเริ่มต้นและสิ้นสุดที่ใด

เราสามารถเรียกใช้งานฟังก์ชันได้โดยการเรียกใช้งานชื่อฟังก์ชัน

    fn main() {
        println!("Hello, world!");

        another_function();
    }

โดยใน rust จะไม่สนใจว่าชื่อฟังก์ชันจะถูกสร้างก่อนหรือหลังตัวที่เรียกใช้งาน ขอแค่อยู่ในขอบเขตที่ผู้ใช้สามารถเรียกใช้งานได้ ก็จะสามารถใช้งานได้ทันที

**แบบ A**

    fn main() {
        println!("Hello, world!");

        another_function();
    }

    fn another_function() {
        println!("Another_function.");
    }

**แบบ B**

    fn another_function() {
        println!("Another_function.");
    }

    fn main() {
        println!("Hello, world!");

        another_function();
    }

    
### Parameters
เราสามารถให้ฟังก์ชันรับข้อมูลเข้ามาเพื่อทำการประมวลผลได้ โดยข้อมูลที่ฟังก์ชันรับเข้ามาจะเรียกว่า พารามีเตอร์ `(Parameter)`

และข้อมูลที่ฟังก์ชันอื่นส่งเข้ามา จะเรียกว่า อาร์กิวเมนต์ `(Argument)`

    fn main() {
        another_function(5); // ส่งข้อมูลเข้าไป เรียกว่า argument
    }

    fn another_function(x: i32) { // รับข้อมูลเข้ามา เรียกว่า parameter
        println!("The value of x is: {x}");
    }

### Statements and Expressions
`Statement` และ `Expression` แตกต่างกันอย่างไร

- `Statement` ชุดคำสั่งที่ดำเนินการโดยไม่ส่งข้อมูลกลับ
- `Expression` ชุดคำสั่งที่ดำเนินการโดยส่งข้อมูลกลับ

ตัวอย่าง ที่ผ่านมาเราได้ใช้ทั้ง `statement` และ `expression` มาแล้ว 

    let y = 6;

การใช้งาน `let` เพื่อสร้างตัวแปรถือเป็น `statement` 

การสร้างฟังก์ชันด้วยคำสั่ง `fn` คือเป็น `statement` เช่นกัน (แต่การเรียกใช้งานฟังก์ชันจะไม่ใช่ `statement`)

`Statement` จะไม่ส่งคืนข้อมูลกลับ ทำให้เมื่อเรานำ `statement` ไปกำหนดให้กับตัวแปรอื่น ๆ จะเกิดข้อผิดพลาด

    fn main() {
        let x = (let y = 6);
    }

ข้อผิดพลาด

    $ cargo run
    Compiling functions v0.1.0 (file:///projects/functions)
    error: expected expression, found `let` statement
     --> src/main.rs:2:14
      |
    2 |     let x = (let y = 6);
      |              ^^^
      |
      = note: only supported directly in conditions of `if` and `while` expressions

    warning: unnecessary parentheses around assigned value
     --> src/main.rs:2:13
      |
    2 |     let x = (let y = 6);
      |             ^         ^
      |
      = note: `#[warn(unused_parens)]` on by default
    help: remove these parentheses
      |
    2 -     let x = (let y = 6);
    2 +     let x = let y = 6;
      |

    warning: `functions` (bin "functions") generated 1 warning
    error: could not compile `functions` (bin "functions") due to 1 previous error; 1 warning emitted

`Expression` คือคำสั่งที่มีการส่งคืนค่ากลับมา เช่น 5 + 1 จะมีการส่งคืนข้อมูลกลับมาคือ 11 โดยจากตัวอย่าง `let x = (let y = 6);` เลข 6 ถือเป็น `expression` เพราะมีการส่งคืน 6 กลับมาที่ตัวแปร\

    let y = 6;

- `let y = ...` ส่วนนี้เป็น statement
- `6` = ส่วนนี้เป็น expression
- `6` ถูกประเมินค่าและนำไปเก็บไว้ใน y

*บล็อกโค้ด `{}` ก็เป็น `expression`*

    let y = {
        let x = 3; // statement
        x + 1 // expression (ไม่มี ;)
    }

- บล็อก `{}` เป็น `expression`
- ค่าสุดท้ายที่ไม่มี `;` คือค่าที่บล็อกนี้คืนออกมา

ดังนั้น `let y = 4`

#### ทำไมต้องไม่มีเซมิโคลอน ?
ใน rust หาก
- มี `;` จะหมายถึง `statement` หรือ ไม่คืนค่า (คืนเป็น () หรือ void)
- ไม่มี `;` จะหมายถึง `expression` หรือ คืนค่า

### Functions with Return Values
ฟังก์ชันสามารถส่งข้อมูลกลับไปยังชุดคำสั่งที่เรียกใช้งานได้ โดยการใช้คำสั่ง `->`

    fn five() -> i32 {
        5
    }

    fn main() {
        let x = five();

        println!("The value of x is: {x});
    }

ใน rust การส่งคืนค่ากลับ บรรทัดสุดท้ายเราสามารถเขียน `expression` ได้เลย โดยไม่ต้องมี `;` แต่หากต้องการส่งคืนค่ากลับก่อน สามารถใช้งาน `return` ได้