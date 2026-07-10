//mod bin_container
pub fn add_bits(a: u8, b: u8, carry: u8) -> (u8, u8) {
    let sum = a ^ b ^ carry;
    let new_carry = (a & b) | (a & carry) | (b & carry);
    (sum, new_carry)
}

pub fn sub_bits(a: u8, b: u8, borrow: u8) -> (u8, u8) {
    let diff = a ^ b ^ borrow;
    let new_borrow = ((a ^ 1) & b) | (((a ^ 1) | b) & borrow);
    (diff, new_borrow)
}

pub fn cal_u8(enter_a: u8, enter_b: u8, borrow: u8) -> (u8, u8) {
    let mut result = 0u8;
    let mut carry = 0u8;

    for i in 0..8 {
        let abit = (enter_a >> i) & 1;
        let bbit = (enter_b >> i) & 1;

        let (bit_result, new_carry) = if borrow == 0 {
            add_bits(abit, bbit, carry)
        } else {
            sub_bits(abit, bbit, carry)
        };

        result |= bit_result << i;
        carry = new_carry;
    }

    (result, carry)
}

pub fn increment_address(address: u8) -> u8 {
    cal_u8(address, 1, 0).0
}
// çarpımların optimizasyonu
pub fn mul_u8(mut enter_1: u8, mut enter_2: u8, result_address: &mut u64) -> u8 {
    let mut result = 0u8;

    while enter_2 != 0 {
        if (enter_2 & 1) != 0 {
            (result, _) = cal_u8(result, enter_1, 0);
        }
        enter_1 <<= 1;
        enter_2 >>= 1;
    }
    *result_address = result as u64;
    result
}
//bölmelerin optimizasyonunda bölen kalan
pub fn div_mod_u8(dividend: u8, divisor: u8) -> (u8, u8) {
    if divisor == 0 {
        panic!("Sıfıra bölme hatası");
    }

    let mut quotient = 0u8;
    let mut remainder = 0u8;

    for i in (0..8).rev() {
        remainder <<= 1;
        remainder |= (dividend >> i) & 1;

        if remainder >= divisor {
            remainder = cal_u8(remainder, divisor, 1).0;
            quotient |= 1 << i;
        }
    }

    (quotient, remainder)
}

pub fn div_u8(dividend: u8, divisor: u8) -> u8 {
    div_mod_u8(dividend, divisor).0
}

pub fn mod_u8(dividend: u8, divisor: u8) -> u8 {
    div_mod_u8(dividend, divisor).1
}

//fn half_adder(a: u8, b: u8) -> (u8, u8) {
//    let sum = a ^ b;
//    let carry = a & b;
//
//    (sum, carry)
//}
//
//fn full_adder(a: u8, b: u8, carry_in: u8) -> (u8, u8) {
//    let (sum1, carry1) = half_adder(a, b);
//    let (sum2, carry2) = half_adder(sum1, carry_in);
//
//    let carry_out = carry1 | carry2;
//
//    (sum2, carry_out)
//}
//
//fn add_bits(x: u8, y: u8) -> (result:u8,carry:u8) {
//
//    let mut carry = 0;
//
//    for i in 0..8 {
//        let a = (x >> i) & 1;
//        let b = (y >> i) & 1;
//
//        let (sum, carry_out) = full_adder(a, b, carry);
//
//        result |= sum << i;
//        carry = carry_out;
//    }
//    if carry == 1 {
//
//    }
//    result
//}

//pub fn mul_bits(a: u8, b: u8, carry: u8) -> (u8, u8) {
//    for i in 0..8 {
//
//    }
//
//
//}
//pub fn div_bits(a: u8, b: u8, carry: u8) -> (u8, u8) {}
