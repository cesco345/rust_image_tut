fn main() {
    let _x1: i8 = 0; // [-128, 127]
    let _x2: i16 = 0;
    let _x3: i32 = 0;
    let _x4: i64 = 0;
    let _x5: i128 = i128::MAX;

    println!("{_x1}, {_x2}, {_x3}, {_x4}, {_x5}");

    let _x1: u8 = u8::MAX; // [0, 255]
    let _x2: u16 = u16::MAX;
    let _x3: u32 = u32::MAX;
    let _x4: u64 = u64::MAX;
    let _x5: u128 = u128::MAX;

    println!("{_x1}, {_x2}, {_x3}, {_x4}, {_x5}");

    let _dec = 255;
    println!("dec: {_dec}");
    let _hex = 0xff;
    println!("hex: {_hex}");
    let _bin = 0b11111111;
    println!("bin: {_bin}");

    let _oct = 0o377;
    println!("oct: {_oct}");

    let _f1 = 2.0;
    let _f2: f64 = 0.0;
    let _f3: f32 = 0.0;

    println!("{_f1}, {_f2}, {_f3}");

    let _f4 = 2.0_f32; // float
    let _f4 = 2.0_f64; // double

    println!("{_f4}");

    let _flag1 = true;
    let _flag2 = false;

    println!("{_flag1}, {_flag2}");

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation

    println!("{c}, {z}");
}
