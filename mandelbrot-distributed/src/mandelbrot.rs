pub fn compute_block(
    start_row: u32,
    end_row: u32,
    width: u32,
    height: u32,
    max_iter: u32,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
) -> Vec<u32> {
    let mut pixels = Vec::new();

    for row in start_row..=end_row {
        for col in 0..width {
            let x0 = x_min + (col as f64 / width as f64) * (x_max - x_min);
            let y0 = y_min + (row as f64 / height as f64) * (y_max - y_min);

            let mut x = 0.0;
            let mut y = 0.0;
            let mut iter = 0;

            while x * x + y * y <= 4.0 && iter < max_iter {
                let xtemp = x * x - y * y + x0;
                y = 2.0 * x * y + y0;
                x = xtemp;
                iter += 1;
            }

            pixels.push(iter);
        }
    }

    pixels
}