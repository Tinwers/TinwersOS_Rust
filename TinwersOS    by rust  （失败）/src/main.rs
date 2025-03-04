use std::io;
use std::io::Write;
fn main() {
    println!("Tinwers OS-Rust v0.1.0");
    println!("(c) SunnyBlank 拥有所有权。");	
    println!("");
    let mut i=0;
    loop{
        if i == 100 {
            break;
        }
        print!("C:\\Users\\RunData>");
        io::stdout().flush();
        let mut hd = String::new();
        let gn1 = "version";
        io::stdin().read_line(&mut hd).expect("Failed to read line");
        println!("rust版本已停用，请更换成C++版本！");
        i+=1;
    }
}
