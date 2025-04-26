
use rust_rsa::bignum::BigNum; // Adjust path if needed

fn main() {
    println!("Testing BigNum struct...");

    // Test new()
    let n0 = BigNum::new();
    println!("new() -> {:?}", n0);

    // Test from_u32()
    let n1 = BigNum::from_u32(12345);
    println!("from_u32(12345) -> {:?}", n1);

    // Test from_i32()
    let n2 = BigNum::from_i32(-54321);
    println!("from_i32(-54321) -> {:?}", n2);

    let n3 = BigNum::from_i32(0);
    println!("from_i32(0) -> {:?}", n3);

    // Test from_u64()
    let n4 = BigNum::from_u64(0x1_0000_0001); // 2^32 + 1
    println!("from_u64(0x1_0000_0001) -> {:?}", n4);

    // Test from_i64()
    let n5 = BigNum::from_i64(-0x2_0000_0001); // -(2^33 + 1)
    println!("from_i64(-0x2_0000_0001) -> {:?}", n5);
}
