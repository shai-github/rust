fn main() {
    // Integers
    // numbers without a fractional component
    // every integer has a length and can be signed or unsigned
    // rust defaults to signed 32-bit integers
    let _a = 98_222; // Decimal
    let _b = 0xff; // Hex
    let _c = 0o77; // Octal
    let _d = 0b1111_0000; // Binary
    let _e = b'A'; // Byte (u8 only)

    // this is an example of integer overflow
    // if you set to a value over 255, Rust will wrap around to 0
    // this means that 256 becomes 0, 257 becomes 1, etc.
    let _f: u8 = 255;

    // Floating-point numbers
    // numbers with decimal points
    // defaults to a 64 double precision floating point number
    let _f = 2.0; // f64
    let _g: f32 = 3.0; // f32

    let _sum = 5 + 10; // addition
    let _difference = 95.5 - 4.3; // subtraction
    let _product = 4 * 30; // multiplication
    let _quotient = 56.7 / 32.2; // division
    let _remainder = 43 % 5; // remainder

    // Booleans
    // true or false declarations
    let _t = true;
    let _f: bool = false; 

    // Characters
    // represent unicode characters
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
}
