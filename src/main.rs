use std::ops::Deref;
use std::time::Duration;

const WIDTH: usize = 64;

type ScreenBuff = [[char; WIDTH]; WIDTH];

struct Buffer {
    buffer: ScreenBuff,
}

impl Buffer {
    fn init() -> Self {
        Buffer {
            buffer: [['-'; WIDTH]; WIDTH],
        }
    }
}

impl Deref for Buffer {
    type Target = ScreenBuff;

    fn deref(&self) -> &Self::Target {
        &self.buffer
    }
}



fn main() {
    let mut buffer = Buffer::init();

    loop {
        if !std::process::Command::new("clear").status().unwrap().success() { break }
        buffer.iter().for_each(|row| {
            row.iter().for_each(|col| {
                print!("{}", col);
            });
            print!("\n");
        });

        std::thread::sleep(Duration::from_millis(24));
    }
}
