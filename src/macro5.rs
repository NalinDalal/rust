use std::fmt::Error;

trait Serialize {
    fn serialize(&self) -> Vec<u8>;
}

trait Deserialize: Sized {
    fn deserialize(base: &[u8]) -> Result<Self, Error>;
}

struct Swap {
    qty_1: i32,
    qty_2: i32,
}

impl Serialize for Swap {
    fn serialize(&self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.qty_1.to_be_bytes());
        result.extend_from_slice(&self.qty_2.to_be_bytes());
        result
    }
}

impl Deserialize for Swap {
    fn deserialize(base: &[u8]) -> Result<Self, Error> {
        if base.len() < 8 {
            return Err(Error);
        }

        let qty_1_bytes: [u8; 4] = base[0..4].try_into().map_err(|_| Error)?;
        let qty_1 = i32::from_be_bytes(qty_1_bytes);

        let qty_2_bytes: [u8; 4] = base[4..8].try_into().map_err(|_| Error)?;
        let qty_2 = i32::from_be_bytes(qty_2_bytes);

        Ok(Swap { qty_1, qty_2 })
    }
}

fn main() {
    let s = Swap { qty_1: 1, qty_2: 2 };

    let bytes = s.serialize();
    let s2 = Swap::deserialize(&bytes).unwrap();
    print!("{:?}", bytes);
    assert!(s.qty_1 == s2.qty_1);
    assert!(s.qty_2 == s2.qty_2);
    println!("Test done")
}
