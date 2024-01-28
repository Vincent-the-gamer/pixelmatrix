use crate::{bluetooth, types::{IDMClockMode, IDMColor, IDMCommand, IDMPixel}};
use colors_transform::Color;
use tokio::time;

pub fn parse_clock_mode_string(clock_mode: &str) -> Result<IDMClockMode, String> {
    let mut args = clock_mode.split(",");
    let style = args.next().unwrap().parse::<u8>().unwrap();
    let r = args.next().unwrap().parse::<i32>().unwrap();
    let g = args.next().unwrap().parse::<i32>().unwrap();
    let b = args.next().unwrap().parse::<i32>().unwrap();
    Ok(IDMClockMode {
        style,
        hour24: true,
        visible_date: false,
        r: Some(r),
        g: Some(g),
        b: Some(b)
    })
}

pub fn parse_color_string(color: &str) -> Result<IDMColor, String> {
    let color = color.trim_start_matches('#');
    let r = u8::from_str_radix(&color[0..2], 16).unwrap();
    let g = u8::from_str_radix(&color[2..4], 16).unwrap();
    let b = u8::from_str_radix(&color[4..6], 16).unwrap();
    Ok(IDMColor { r, g, b })
}

pub fn parse_pixel_string(pixel: &str) -> Result<IDMPixel, String> {
    let mut pixel = pixel.split(',');
    let x = pixel.next().unwrap().parse::<u8>().unwrap();
    let y = pixel.next().unwrap().parse::<u8>().unwrap();
    let color = parse_color_string(pixel.next().unwrap()).unwrap();
    Ok(IDMPixel { x, y, color })
}

pub async fn color_hue(bluetooth: &bluetooth::BluetoothWrapper) {
    let mut hue = 180.0;

    let mut last_loop = time::Instant::now();
    loop {
        for y in 0..32 {
            for x in 0..32 {
                let color = colors_transform::Hsl::from(hue, 100.0, 50.0).to_rgb();
                hue += 0.2;
                hue %= 360.0;
                let color_u8 = color.as_tuple();
                let color_u8 = (color_u8.0 as u8, color_u8.1 as u8, color_u8.2 as u8);

                let command = IDMCommand::SetPixel(IDMPixel {
                    x: x as u8,
                    y: y as u8,
                    color: IDMColor {
                        r: color_u8.0,
                        g: color_u8.1,
                        b: color_u8.2,
                    },
                });
                bluetooth.send_command(&command).await;
            }
        }

        let now = time::Instant::now();
        let duration = now.duration_since(last_loop);
        println!("Loop duration: {:?}", duration);
        last_loop = now;
    }
}