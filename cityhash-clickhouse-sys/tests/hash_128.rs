use cityhash_clickhouse_sys::cityhash::city_hash_128;

#[test]
fn hash_128() {
    let key = "Moscow";
    let hash = city_hash_128(key.as_bytes());

    assert_eq!(hash, 46102140593102845793298614593550999405);
}
