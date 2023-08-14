use std::time::Duration;

const HEIGHT: f32 = 24_f32;
const WIDTH: f32 = HEIGHT * 3_f32;

fn main() {
    let mut count = 0_f32;
    let mut inc_flag = true;
    // Origin
    // The origins affect their opposite axes
    let (x_origin, y_origin) = ((HEIGHT / 2_f32) + 2_f32, WIDTH / 2_f32);

    // Minor axis
    let radius2 = (WIDTH / 2_f32) as f32;

    // Pupil
    let radius3 = WIDTH / 10_f32;
    let (x_iris_origin, y_iris_origin) = (x_origin, y_origin);

    loop {
        std::process::Command::new("clear").status().unwrap();

        // Counting
        if inc_flag {
            count += 0.5_f32;
        } else {
            count -= 0.5_f32;
        }

        // This is for the max before we have to reverse the count
        // Adjusts the bounds for radius expansion
        if count >= HEIGHT / 2.7_f32 {
            inc_flag = false;
        }
        if count <= 0_f32 {
            inc_flag = true;
        }

        // Major axis
        let radius1 = count as f32;

       // Draw screen
        for x_point in 0..(HEIGHT as usize) {
            let mut is_next = true;

            for y_point in 0..(WIDTH as usize) {
                let x_point = x_point as f32;
                let y_point = y_point as f32;

                if (x_origin - 2_f32..x_origin + 3_f32).contains(&x_point) {
                    is_next = false;
                    break
                }

                // For circle
                {
                    let x_diff = x_point - x_iris_origin;
                    let y_diff = y_point - y_iris_origin;
                    let x_diff_sqr = x_diff * x_diff;
                    let y_diff_sqr = y_diff * y_diff;

                    if x_diff_sqr + (y_diff_sqr / 4_f32) <= radius3.powi(2) {
                        print!(" ");
                        continue
                    }
                }

                let x_diff = x_point - x_origin;
                let y_diff = y_point - y_origin;
                let x_diff_sqr = x_diff * x_diff;
                let y_diff_sqr = y_diff * y_diff;
                let x_part = x_diff_sqr / radius1.powi(2);
                let y_part = y_diff_sqr / radius2.powi(2);

                if x_part + y_part < 1_f32 {
                    print!("█");
                    continue
                }

                print!(" ");
            }

            if is_next { print!("\n") }
        }

        std::thread::sleep(Duration::from_millis(42));
    }
}
