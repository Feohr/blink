use std::time::Duration;

const WIDTH: i32 = 24;

fn main() {
    let mut count = 0;
    let mut inc_flag = true;
    // Origin
    // The origins affect their opposite axes
    let (x_origin, y_origin) = ((WIDTH / 2) + 2, WIDTH / 2);

    loop {
        if !std::process::Command::new("clear").status().unwrap().success() { break }

        // Counting
        if inc_flag {
            count += 1;
        } else {
            count -= 1;
        }
        // This is for the max before we have to reverse the count
        // Adjusts the bounds for radius expansion
        if count >= WIDTH / 4 {
            inc_flag = false;
        }
        if count <= 0 {
            inc_flag = true;
        }

        // A
        let radius1 = count;
        // B
        let radius2 = WIDTH / 4;

        // Draw screen
        for x_point in 0..WIDTH {
            for y_point in 0..WIDTH {
                let x_diff = x_point - x_origin;
                let y_diff = y_point - y_origin;

                let x_diff_sqr = x_diff * x_diff;
                let y_diff_sqr = y_diff * y_diff;

                let x_part = x_diff_sqr.checked_div(radius1.pow(2)).unwrap_or(2);
                let y_part = y_diff_sqr.checked_div(radius2.pow(2)).unwrap_or(2);

                if x_part + y_part > 1 {
                    print!("-");
                    continue
                }

                print!("#");
            }
            print!("\n");
        }

        std::thread::sleep(Duration::from_millis(84));
    }
}
