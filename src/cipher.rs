use std::{collections::VecDeque, vec};

use crate::{
    bit::{Bit, Bitwise, ShiftLeft, StringBit, ToBitVec, ToInteger},
    tables::{
        FINAL_PERMUTATION_TABLE, INITIAL_PERMUTATION_TABLE, KEY_SHIFT, P_TABLE, PC1, PC2,
        RE_EXPANSION_TABLE, SBOX_TABLE,
    },
};

fn get_bits(number: u64) -> Vec<Bit> {
    let mut bits: Vec<u8> = vec![];

    for i in 0..64 {
        let is_one = (number >> i) & 0b1 > 0;
        bits.push(if is_one { 1 } else { 0 });
    }

    bits.reverse();
    bits
}

// warning: it_number é one-based
fn make_subkey(key: u64, it_number: usize) -> Vec<Bit> {
    assert!(it_number != 0, "it_number={} inválido", it_number);
    assert!(it_number <= PC2.len(), "it_number={} inválido", it_number);

    let key_bits = get_bits(key);
    let mut encoded_pc1: Vec<Bit> = vec![];
    println!("Bits da chave: {:?}", key_bits.bits());

    for pc1_value in PC1.iter() {
        let swapped_bit = key_bits.get((pc1_value - 1) as usize).unwrap();
        encoded_pc1.push(*swapped_bit);
    }

    println!("pc1-encodado : {:?}", encoded_pc1.bits());

    let mut chunks = encoded_pc1.chunks_exact(28);
    let mut c: VecDeque<Bit> = VecDeque::from(chunks.next().unwrap().to_vec());
    let mut d: VecDeque<Bit> = VecDeque::from(chunks.next().unwrap().to_vec());

    println!("half       : c={:?} d={:?}", c.bits(), d.bits());
    for i in 0..(it_number) {
        c.shift_left(KEY_SHIFT[i].into(), true);
        d.shift_left(KEY_SHIFT[i].into(), true);
    }
    println!("half  <<   : c={:?} d={:?}", c.bits(), d.bits());

    let mut concat = c.clone();
    concat.extend(d);
    println!("concat       : {:?}", concat.bits());

    // ate aqui beleza

    let mut subkey: Vec<Bit> = vec![];
    for pc2_value in PC2.iter() {
        let swapped_bit = concat.get((pc2_value - 1) as usize).unwrap();
        subkey.push(*swapped_bit);
    }

    println!("subkey       : {:?}", subkey.bits());
    subkey
}

// vai diminuir o input  de 48 bits para 32 bits
fn permute_re(input: &Vec<Bit>) -> Vec<Bit> {
    assert_eq!(input.len(), 48, "input não tem 48 bits");

    let mut sboxed: Vec<Bit> = vec![];

    for i in 0..8 {
        let start = i * 6;
        let end = start + 6;
        let group: Vec<Bit> = input[start..end].into();

        assert_eq!(group.len(), 6, "group não tem 6 bits");

        println!("group_{}      : {:?}", i + 1, group.bits());

        let group_number = group.to_u8();
        let row_no = ((group_number & 0b100000) >> 4) | (group_number & 0b1);
        let col_no = (group_number & 0b011110) >> 1;

        let sbox_val = SBOX_TABLE[i][row_no as usize][col_no as usize];
        let mut sbox_val_bits = sbox_val.to_bit_vec();

        println!(
            "sbox[{}][{}][{}]: {} ({:?})",
            i,
            row_no,
            col_no,
            sbox_val,
            sbox_val_bits.bits()
        );

        // como convertemos um u8 para Vec<Bit>, o Vec<Bit> tem 8 itens, mas sbox_val só retornaria 4 bits na realidade
        // então vamos descartar os 4 zeros iniciais
        for bit in &sbox_val_bits[4..8] {
            sboxed.push(*bit);
        }
    }

    println!("sboxed       : {:?}", sboxed);

    // permutacao do sboxed sobre ptable
    let mut permuted: Vec<Bit> = vec![];
    for pvalue in P_TABLE.iter() {
        let swapped_bit = sboxed.get((pvalue - 1) as usize).unwrap();
        permuted.push(*swapped_bit);
    }

    println!("permuted     : {:?}", permuted);

    permuted
}

fn bit_expansion(
    re: &Vec<Bit>,     /* 32 bits */
    subkey: &Vec<Bit>, /* 48 bits na subkey */
) -> Vec<Bit> {
    assert_eq!(re.len(), 32, "re não tem 32 bits");
    assert_eq!(subkey.len(), 48, "subkey não tem 48 bits");

    let mut expanded_re: Vec<Bit> = vec![];
    for keyexpand_value in RE_EXPANSION_TABLE.iter() {
        let swapped_bit = re.get((keyexpand_value - 1) as usize).unwrap();
        expanded_re.push(*swapped_bit);
    }

    let exp_re_xor = expanded_re.xor(subkey);

    println!("expanded_re  : {:?}", expanded_re);
    println!("exp_re_xor   : {:?}", exp_re_xor);

    exp_re_xor
}

#[derive(PartialEq)]
pub enum DESMode {
    Cipher,
    Decipher,
}

pub fn process(input: u64, key: u64, mode: DESMode) -> u64 {
    let input_bits = input.to_bit_vec();
    println!("inpplain    : {:?}", input_bits);

    let mut input_permuted: Vec<Bit> = vec![];
    for idx in INITIAL_PERMUTATION_TABLE {
        let swapped_bit = input_bits.get((idx - 1) as usize).unwrap();
        input_permuted.push(*swapped_bit);
    }

    println!("permut       : {:?}", input_permuted);

    let mut left: Vec<Bit> = input_permuted[0..32].into();
    let mut right: Vec<Bit> = input_permuted[32..64].into();

    for mut round_no in 1..17 {
        if mode == DESMode::Decipher {
            round_no = 17 - round_no;
        }

        println!("round {}", round_no);
        println!("left         : {:?}", left);
        println!("right        : {:?}", right);

        let subkey = make_subkey(key, round_no);

        let re_expanded = bit_expansion(&right, &subkey);
        let f_output = permute_re(&re_expanded);

        let next_le = right.clone();
        let next_re = f_output.xor(&left);

        println!("next_le      : {:?}", next_le.bits());
        println!("next_re      : {:?}", next_re.bits());

        left = next_le;
        right = next_re;
    }

    // rounds feitas, vamos concatenar novamente
    let mut concated: Vec<Bit> = vec![];
    for bit in right {
        concated.push(bit);
    }
    for bit in left {
        concated.push(bit);
    }

    let mut final_reshuffle: Vec<Bit> = vec![];
    for idx in FINAL_PERMUTATION_TABLE {
        let swapped_bit = concated.get((idx - 1) as usize).unwrap();
        final_reshuffle.push(*swapped_bit);
    }

    println!("final: {:?}", final_reshuffle);

    final_reshuffle.to_u64()
}

#[cfg(test)]
mod tests {
    use crate::bit::{ToBitVec, explode_bitstring};

    use super::*;

    #[test]
    fn util_bitstring_exploder() {
        let expected_subkey = explode_bitstring("0101");
        assert_eq!(expected_subkey, [0, 1, 0, 1]);

        let expected_subkey = explode_bitstring("1111 0000");
        assert_eq!(expected_subkey, [1, 1, 1, 1, 0, 0, 0, 0]);
    }

    #[test]
    fn util_u64_to_bits() {
        assert_eq!(
            0b10000000_00000000_00000000_00000000_00000000_00000000_10000000_00001111u64
                .to_bit_vec(),
            explode_bitstring(
                "10000000 00000000 00000000 00000000 00000000 00000000 10000000 00001111"
            )
        );
    }

    #[test]
    fn subkey_generator() {
        let key = 0x0123456789ABCDEFu64;

        assert_eq!(
            make_subkey(key, 1),
            explode_bitstring("000010 110000 001001 100111 100110 110100 100110 100101")
        );
        assert_eq!(
            make_subkey(key, 2),
            explode_bitstring("011010 011010 011001 011001 001001 010110 101000 100110")
        );
        assert_eq!(
            make_subkey(key, 3),
            explode_bitstring("010001 011101 010010 001010 101101 000010 100011 010010")
        );
        assert_eq!(
            make_subkey(key, 4),
            explode_bitstring("011100 101000 100111 010010 101001 011000 001001 010111")
        );
        assert_eq!(
            make_subkey(key, 5),
            explode_bitstring("001111 001110 100000 000011 000101 111010 011011 000010")
        );
        assert_eq!(
            make_subkey(key, 6),
            explode_bitstring("001000 110010 010100 011110 001111 001000 010101 000101")
        );
        assert_eq!(
            make_subkey(key, 7),
            explode_bitstring("011011 000000 010010 010101 000010 101110 010011 000110")
        );
        assert_eq!(
            make_subkey(key, 8),
            explode_bitstring("010101 111000 100000 111000 011011 001110 010110 000001")
        );
        assert_eq!(
            make_subkey(key, 9),
            explode_bitstring("110000 001100 100111 101001 001001 101011 100000 111001")
        );
        assert_eq!(
            make_subkey(key, 10),
            explode_bitstring("100100 011110 001100 000111 011000 110001 110101 110010")
        );
        assert_eq!(
            make_subkey(key, 11),
            explode_bitstring("001000 010001 111110 000011 000011 011000 100100 111010")
        );
        assert_eq!(
            make_subkey(key, 12),
            explode_bitstring("011100 010011 000011 100101 010001 010101 110001 010100")
        );
        assert_eq!(
            make_subkey(key, 13),
            explode_bitstring("100100 011100 010011 010000 010010 011000 000011 111100")
        );
        assert_eq!(
            make_subkey(key, 14),
            explode_bitstring("010101 000100 001110 110110 100000 011101 110010 001101")
        );
        assert_eq!(
            make_subkey(key, 15),
            explode_bitstring("101101 101001 000100 000101 000010 100001 011010 110101")
        );
        assert_eq!(
            make_subkey(key, 16),
            explode_bitstring("110010 100011 110100 000011 101110 000111 000000 110010")
        );

        println!();
    }

    #[test]
    fn cipher() {
        let ciphered = process(0x0123456789ABCDEF, 0x0123456789ABCDEF, DESMode::Cipher);
        println!("ciphered: {:X}", ciphered);
        assert_eq!(ciphered, 0x56CC09E7CFDC4CEFu64);
    }

    #[test]
    fn decipher() {
        let deciphered: u64 = process(0x56CC09E7CFDC4CEFu64, 0x0123456789ABCDEF, DESMode::Decipher);
        println!("deciphered: {:X}", deciphered);
        assert_eq!(deciphered, 0x0123456789ABCDEF);
    }
}
