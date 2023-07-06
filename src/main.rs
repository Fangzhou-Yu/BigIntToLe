use num_bigint::{BigUint, BigInt, ToBigInt};
use num_traits::Num;
use babyjubjub_rs::*;
use ff::*;

fn main() {
    let num_str = "4338620300185947561074059802482547481416142213883829469920100239455078257889";
    let num = BigUint::parse_bytes(num_str.as_bytes(), 10).unwrap();    // this 10 means decimal

    let bytes = num.to_bytes_le();

    let raw_digits: Vec<u64> = bytes
        .chunks_exact(8)
        .map(|chunk| {
            let mut bytes = [0u8; 8];       // 8 so we break it up into length of 8 array pieces
            bytes.copy_from_slice(chunk);
            u64::from_le_bytes(bytes)       // little endian form
        })
        .collect();

    println!("Number: {}", num);
    println!("Little-endian Bytes: {:?}", bytes);
    println!("Raw Digits: {:?}", raw_digits);

    for value in raw_digits.iter() {
        let hex_string = format!("0x{:02x}", value);
        println!("{}", hex_string);
    }

    let p: Point = Point {
        x: Fr::from_str(
            "17777552123799933955779906779655732241715742912184938656739573121738514868268",
        )
        .unwrap(),
        y: Fr::from_str(
            "2626589144620713026669568689430873010625803728049924121243784502389097019475",
        )
        .unwrap(),
    };

    let res_m = p.mul_scalar(&3.to_bigint().unwrap());
    println!("{:?}", res_m.x);
    println!("{:?}", res_m.y);
    
}
