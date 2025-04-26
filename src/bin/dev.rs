
use rust_rsa::bignum::BigNum; // Adjust path if needed
use std::ops::Add;

fn test_abs() {
    println!("Testing BigNum abs method...");

    let n1 = BigNum::from_i32(12345);
    let n2 = BigNum::from_i32(-12345);
    let n3 = BigNum::from_i32(0);
    let n4 = BigNum::from_i64(-0x1_0000_0001); // -(2^32 + 1)

    let n1_abs = n1.abs();
    let n2_abs = n2.abs();
    let n3_abs = n3.abs();
    let n4_abs = n4.abs();

    println!("n1 = {:?}, abs(n1) = {:?}", n1, n1_abs);
    println!("n2 = {:?}, abs(n2) = {:?}", n2, n2_abs);
    println!("n3 = {:?}, abs(n3) = {:?}", n3, n3_abs);
    println!("n4 = {:?}, abs(n4) = {:?}", n4, n4_abs);
}

fn test_constructor() {
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

fn test_add_digits() {
    println!("Testing add_digits intensively...");

    // Helper function to print vectors nicely
    fn show(v: &Vec<u32>) {
        println!("{:?}", v);
    }

    // 1. Simple addition without carry
    let a = vec![1, 2, 3];
    let b = vec![4, 5, 6];
    let result = BigNum::add_digits(&a, &b);
    println!("Case 1:");
    show(&result);
    assert_eq!(result, vec![5, 7, 9]);

    // 2. Addition with carry within digits
    let a = vec![0xFFFF_FFFF, 0x0000_0001];
    let b = vec![1];
    let result = BigNum::add_digits(&a, &b);
    println!("Case 2:");
    show(&result);
    assert_eq!(result, vec![0, 2]);

    // 3. Final carry causing an extra digit
    let a = vec![0xFFFF_FFFF, 0xFFFF_FFFF];
    let b = vec![1];
    let result = BigNum::add_digits(&a, &b);
    println!("Case 3:");
    show(&result);
    assert_eq!(result, vec![0, 0, 1]);

    // 4. Different lengths (a shorter)
    let a = vec![1, 2];
    let b = vec![3, 4, 5];
    let result = BigNum::add_digits(&a, &b);
    println!("Case 4:");
    show(&result);
    assert_eq!(result, vec![4, 6, 5]);

    // 5. Different lengths (b shorter)
    let a = vec![7, 8, 9];
    let b = vec![1];
    let result = BigNum::add_digits(&a, &b);
    println!("Case 5:");
    show(&result);
    assert_eq!(result, vec![8, 8, 9]);

    // 6. All zero vectors
    let a = vec![0, 0, 0];
    let b = vec![0, 0, 0];
    let result = BigNum::add_digits(&a, &b);
    println!("Case 6:");
    show(&result);
    assert_eq!(result, vec![0, 0, 0]);

    // 7. Stress test: Large random numbers
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let a: Vec<u32> = (0..1000).map(|_| rng.gen()).collect();
    let b: Vec<u32> = (0..800).map(|_| rng.gen()).collect();
    let result = BigNum::add_digits(&a, &b);
    println!("Case 7: Large random addition done ({} digits)", result.len());
}


fn test_sub_digits() {
    println!("Testing sub_digits intensively...");

    fn show(v: &Vec<u32>) {
        println!("{:?}", v);
    }

    // 1. Simple subtraction without borrow
    let a = vec![5, 6, 7];
    let b = vec![1, 2, 3];
    let result = BigNum::sub_digits(&a, &b);
    println!("Case 1:");
    show(&result);
    assert_eq!(result, vec![4, 4, 4]);

    // 2. Subtraction with borrow across one digit
    let a = vec![0, 1];
    let b = vec![1];
    let result = BigNum::sub_digits(&a, &b);
    println!("Case 2:");
    show(&result);
    assert_eq!(result, vec![u32::MAX]);

    // 3. Multiple borrows across multiple digits
    let a = vec![0, 0, 1];
    let b = vec![1];
    let result = BigNum::sub_digits(&a, &b);
    println!("Case 3:");
    show(&result);
    assert_eq!(result, vec![u32::MAX, u32::MAX]);

    // 4. Subtract exactly same number (should get [0])
    let a = vec![12345, 67890];
    let b = vec![12345, 67890];
    let result = BigNum::sub_digits(&a, &b);
    println!("Case 4:");
    show(&result);
    assert_eq!(result, vec![0]);

    // 5. Minuend longer than subtrahend
    let a = vec![1, 2, 3, 4];
    let b = vec![1, 2];
    let result = BigNum::sub_digits(&a, &b);
    println!("Case 5:");
    show(&result);
    assert_eq!(result, vec![0, 0, 3, 4]);

    // 6. Stress test with large random vectors
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let mut a: Vec<u32> = (0..1000).map(|_| rng.gen()).collect();
    let b: Vec<u32> = (0..800).map(|_| rng.gen()).collect();

    // Ensure a >= b by zero-padding b
    for _ in 0..(a.len() - b.len()) {
        a.push(0);
    }

    let result = BigNum::sub_digits(&a, &b);
    println!("Case 6: Large random subtraction done. Result length: {}", result.len());
}


pub fn test_add_bignum() {
    println!("Running test_add_bignum...");

    fn show(label: &str, n: &BigNum) {
        println!("{} => sign: {}, digits: {:?}", label, n.sign, n.digits);
    }

    // 1. Simple positive + positive
    {
        let a = BigNum::from_i32(123);
        let b = BigNum::from_i32(456);
        let c = a + b;
        show("Case 1 (123 + 456)", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![579]);
    }

    // 2. Simple negative + negative
    {
        let a = BigNum::from_i32(-100);
        let b = BigNum::from_i32(-50);
        let c = a + b;
        show("Case 2 (-100 + -50)", &c);
        assert_eq!(c.sign, -1);
        assert_eq!(c.digits, vec![150]);
    }

    // 3. Positive + negative (positive bigger)
    {
        let a = BigNum::from_i32(200);
        let b = BigNum::from_i32(-50);
        let c = a + b;
        show("Case 3 (200 + -50)", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![150]);
    }

    // 4. Positive + negative (negative bigger)
    {
        let a = BigNum::from_i32(50);
        let b = BigNum::from_i32(-200);
        let c = a + b;
        show("Case 4 (50 + -200)", &c);
        assert_eq!(c.sign, -1);
        assert_eq!(c.digits, vec![150]);
    }

    // 5. Opposite numbers (should become zero)
    {
        let a = BigNum::from_i32(777);
        let b = BigNum::from_i32(-777);
        let c = a + b;
        show("Case 5 (777 + -777)", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![0]);
    }

    // 6. Large numbers (carry across digits)
    {
        let a = BigNum::from_u64(0xFFFF_FFFF);
        let b = BigNum::from_u64(1);
        let c = a + b;
        show("Case 6 (0xFFFFFFFF + 1)", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![0, 1]); // [low, high]
    }

    // 7. Very large numbers with multi-limbs
    {
        let a = BigNum::from_u64(0xFFFF_FFFF_FFFF_FFFF);
        let b = BigNum::from_u64(0x1);
        let c = a + b;
        show("Case 7 (0xFFFFFFFFFFFFFFFF + 1)", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![0, 0, 1]); // Carry over 2 limbs
    }

    println!("All test_add_bignum cases passed!\n");
}

pub fn test_neg_bignum() {
    println!("Running test_neg_bignum...");

    fn show(label: &str, n: &BigNum) {
        println!("{} => sign: {}, digits: {:?}", label, n.sign, n.digits);
    }

    // 1. Negate positive number
    {
        let a = BigNum::from_i32(123);
        let b = -a;
        show("Case 1 (negate 123)", &b);
        assert_eq!(b.sign, -1);
        assert_eq!(b.digits, vec![123]);
    }

    // 2. Negate negative number
    {
        let a = BigNum::from_i32(-456);
        let b = -a;
        show("Case 2 (negate -456)", &b);
        assert_eq!(b.sign, 1);
        assert_eq!(b.digits, vec![456]);
    }

    // 3. Negate zero
    {
        let a = BigNum::from_i32(0);
        let b = -a;
        show("Case 3 (negate 0)", &b);
        assert_eq!(b.sign, 1); // Should stay positive
        assert_eq!(b.digits, vec![0]);
    }

    // 4. Negate large number (multi-limb BigNum)
    {
        let a = BigNum::from_u64(0xFFFF_FFFF_FFFF_FFFF);
        let b = -a;
        show("Case 4 (negate 0xFFFFFFFFFFFFFFFF)", &b);
        assert_eq!(b.sign, -1);
        assert_eq!(b.digits, vec![0xFFFF_FFFF, 0xFFFF_FFFF]);
    }

    // 5. Double negation should restore original
    {
        let a = BigNum::from_i32(789);
        let b = -a;
        let c = -b;
        show("Case 5 (double negation 789)", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![789]);
    }

    println!("All test_neg_bignum cases passed!\n");
}

pub fn test_sub_bignum() {
    println!("Running test_sub_bignum...");

    fn show(label: &str, n: &BigNum) {
        println!("{} => sign: {}, digits: {:?}", label, n.sign, n.digits);
    }

    // 1. Simple positive - positive (no borrow)
    {
        let a = BigNum::from_i32(500);
        let b = BigNum::from_i32(123);
        let c = a - b;
        show("Case 1 (500 - 123)", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![377]);
    }

    // 2. Positive - larger positive (result negative)
    {
        let a = BigNum::from_i32(123);
        let b = BigNum::from_i32(500);
        let c = a - b;
        show("Case 2 (123 - 500)", &c);
        assert_eq!(c.sign, -1);
        assert_eq!(c.digits, vec![377]);
    }

    // 3. Negative - negative (smaller minus bigger)
    {
        let a = BigNum::from_i32(-100);
        let b = BigNum::from_i32(-500);
        let c = a - b;
        show("Case 3 (-100 - -500)", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![400]);
    }

    // 4. Positive - negative (should add)
    {
        let a = BigNum::from_i32(123);
        let b = BigNum::from_i32(-77);
        let c = a - b;
        show("Case 4 (123 - (-77))", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![200]);
    }

    // 5. Negative - positive (should subtract properly)
    {
        let a = BigNum::from_i32(-123);
        let b = BigNum::from_i32(77);
        let c = a - b;
        show("Case 5 (-123 - 77)", &c);
        assert_eq!(c.sign, -1);
        assert_eq!(c.digits, vec![200]);
    }

    // 6. Subtract zero from a number
    {
        let a = BigNum::from_i32(321);
        let b = BigNum::from_i32(0);
        let c = a - b;
        show("Case 6 (321 - 0)", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![321]);
    }

    // 7. Subtract a number from zero
    {
        let a = BigNum::from_i32(0);
        let b = BigNum::from_i32(321);
        let c = a - b;
        show("Case 7 (0 - 321)", &c);
        assert_eq!(c.sign, -1);
        assert_eq!(c.digits, vec![321]);
    }

    // 8. Subtract same number (should be zero)
    {
        let a = BigNum::from_i32(789);
        let b = BigNum::from_i32(789);
        let c = a - b;
        show("Case 8 (789 - 789)", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![0]);
    }

    // 9. Large number subtraction with borrow across digits
    {
        let a = BigNum::from_u64(0x1_0000_0000);
        let b = BigNum::from_u64(1);
        let c = a - b;
        show("Case 9 (0x100000000 - 1)", &c);
        assert_eq!(c.sign, 1);
        assert_eq!(c.digits, vec![0xFFFF_FFFF]);
    }

    println!("All test_sub_bignum cases passed!\n");
}


fn main() {
    println!("Testing BigNum struct...");

    // test_constructor();

    // test_abs();

    // test_add_digits();

    // test_sub_digits();

    // test_add_bignum();
    //test_neg_bignum();
    test_sub_bignum()

}