mod bin_container;
mod bin_math;

use bin_container::Memory;

fn main() {
    let mut memory = Memory::new(256);

    let addr = memory.alloc(4).unwrap();

    memory.write(addr, &[0b10001001, 20, 30, 40]);

    let data = memory.read(addr, 4).unwrap();

    println!("{:?}", data);
}
//fn add_bits(a: u8, b: u8, carry: u8) -> (u8, u8) {
//    let sum = a ^ b ^ carry;
//
//    let new_carry = if (a & b) == 1 || (a & carry) == 1 || (b & carry) == 1 {
//        1
//    } else {
//        0
//    };
//
//    (sum, new_carry)
//}
//
//fn add_u8(a: u8, b: u8) -> (u8, u8) {
//    let mut result = 0u8;
//    let mut carry = 0u8;
//
//    for i in 0..8 {
//        let bit_a = (a >> i) & 1;
//        let bit_b = (b >> i) & 1;
//
//        let (sum_bit, new_carry) = add_bits(bit_a, bit_b, carry);
//
//        result = result | (sum_bit << i);
//        carry = new_carry;
//    }
//
//    (result, carry)
//}
//
//fn increment_address(address: u8) -> u8 {
//    let (new_address, _carry) = add_u8(address, 1);
//    new_address
//}
//
//fn main() {
//    let address_a = 0u8;
//    let address_b = 1u8;
//    let mut result_address = 2u8;
//
//    let value_at_address_a = 0b11111111u8;
//    let value_at_address_b = 0b00000001u8;
//
//    let (result_value, mut carry) = add_u8(value_at_address_a, value_at_address_b);
//
//    println!("address {} value = {:08b}", address_a, value_at_address_a);
//    println!("address {} value = {:08b}", address_b, value_at_address_b);
//    println!(
//        "write address {} value = {:08b}",
//        result_address, result_value
//    );
//
//    while carry == 1 {
//        result_address = increment_address(result_address);
//
//        let value_at_next_address = 0u8;
//        let (new_value, new_carry) = add_u8(value_at_next_address, carry);
//
//        println!("write address {} value = {:08b}", result_address, new_value);
//
//        carry = new_carry;
//    }
//}
