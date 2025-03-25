use cityhash_clickhouse_sys::cityhash::city_hash_64;

// See https://clickhouse.com/docs/native-protocol/hash

#[test]
fn hash_64_one() {

    let key = "Moscow";
    let hash = city_hash_64(key.as_bytes());

    assert_eq!(hash, 12507901496292878638);
}

#[test] 
fn hash_64_two() {
    let key = "How can you write a big system without C++?  -Paul Glick";
    let hash = city_hash_64(key.as_bytes());

    assert_eq!(hash, 6237945311650045625);
}
