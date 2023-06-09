

trait Cipher {
    fn encrypt(&self) -> String;
}

impl<T> Cipher for T 
where T: ToString {
    fn encrypt(&self) -> String {
        let mut encrypted_string = String::new();
        let data = self.to_string();
        for ch in data.chars() {
            encrypted_string.push((ch as u8 + 1) as char);
        }

        encrypted_string
    }
}

struct DataFrame {
    len: usize,
    data: Vec<u8>,
}

impl DataFrame {
    fn new(message: String) -> Self {
       let data = message.as_bytes();
       let len = data.len();
        DataFrame{
            len,
            data: data.to_vec()
        } 
    }
}

impl ToString for DataFrame {
    fn to_string(&self) -> String {
        if self.len == 0 {
            return String::from("");
        }
        return String::from_utf8(self.data.clone()).unwrap();
    }
}

fn main() {
    let message: String = String::from("Hello, World!");
    // let e_message = message.encrypt();
    let df = DataFrame::new(message);
    println!("{}", df.encrypt());
}

