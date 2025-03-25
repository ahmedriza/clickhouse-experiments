# CityHash for ClickHouse

This sys crate contains the CityHash implementation as used by ClickHouse.

We cannot directly use [cityhash-sys](https://crates.io/crates/cityhash-sys)
because it uses a newer version of the CityHash library than ClickHouse uses.

