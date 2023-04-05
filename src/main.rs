use raster::Color;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_image = raster::open(
        args.get(1).expect("Not enough arguments provided")
    ).expect("Could not open input image");

    if input_image.width != 64 && input_image.height != 64 {
        panic!("Invalid image dimensions. Expected 64x64 but got {}x{}", input_image.width, input_image.height);
    }

    let palette = fs::read_to_string(
        args.get(2).expect("Not enough arguments provided")
    ).expect("Could not open color palette");

    let mut output = "#Put these colors into the LED color window\n#".to_string();
    
    let palette = {
        let mut vec = Vec::new();
        let colors = palette.trim().split('\n').collect::<Vec<&str>>();
        if colors.len() > 16 {
            panic!("Too many colors in provided palette");
        }
        for s in colors.iter() {
            vec.push(Color::hex(*s).expect("invalid color in provided palette"));
            output.push_str(&s[1..]);
            output.push_str(", ");
        }
        vec
    };
    output.push_str("\n#Actual Image:");
    for y8 in 0..8 {
        for x in 0..64 {
            output.push_str("\n0x");
            for y1 in 0..8 {
                let pixel = input_image.get_pixel(x, y1 + y8*8).unwrap();
                let (color, _) = {
                    let mut iter = palette.iter();
                    let first = iter.next().unwrap();
                    let out = iter.fold({
                        let dr: u32 = if pixel.r > first.r {pixel.r - first.r} else {first.r - pixel.r} as u32;
                        let dg: u32 = if pixel.g > first.g {pixel.g - first.g} else {first.g - pixel.g} as u32;
                        let db: u32 = if pixel.b > first.b {pixel.b - first.b} else {first.b - pixel.b} as u32;
                        (first, dr*dr + dg*dg + db*db)
                    },
                        |(closest, dist), c| {
                            let dr: u32 = if pixel.r > c.r {pixel.r - c.r} else {c.r - pixel.r} as u32;
                            let dg: u32 = if pixel.g > c.g {pixel.g - c.g} else {c.g - pixel.g} as u32;
                            let db: u32 = if pixel.b > c.b {pixel.b - c.b} else {c.b - pixel.b} as u32;
                            let d = dr*dr + dg*dg + db*db;
                            if dist < d {
                                (closest, dist)
                            } else {
                                (c, d)
                            }
                        }
                    );
                    out
                };
                let num = palette.iter().position(|c| color.r == c.r && color.g == c.g && color.b == c.b).unwrap();
                output.push_str(&format!("{:X}", num));
            }
        }
    }

    fs::write(
        args.get(3).expect("Not enough arguments provided"),
        output
    ).expect("Failed to write output file");
}
