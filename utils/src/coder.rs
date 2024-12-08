use bincode;
use serde::{Serialize, Deserialize};
use sha256::digest;

pub fn my_serialze<T: ?Sized>(value: &T) -> Vec<u8>
where
    T: Serialize,
{
    let serialized = bincode::serialize(value).unwrap();
    serialized
}

pub fn my_deserialize<'a, T>(bytes: &'a [u8]) -> T
where
    T: Deserialize<'a>,
{
    let deserialized = bincode::deserialize(bytes).unwrap();
    deserialized
}

pub fn get_hash(value: &[u8]) -> String {
    digest(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Serialize, Deserialize, Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn coders_shold_work() {
        let point = Point { x: 1, y: 2 };
        let serialized = my_serialze(&point);
        let deserialized: Point = my_deserialize(&serialized);
        assert_eq!(point.x, deserialized.x);
        assert_eq!(point.y, deserialized.y);
    }

    #[test]
    fn hash_should_work() {
        let hash = get_hash(b"hello");
        println!("hash: {}", hash);
        assert_eq!(hash, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }
}
