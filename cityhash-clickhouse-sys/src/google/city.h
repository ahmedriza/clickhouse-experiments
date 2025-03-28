// Copyright (c) 2011 Google, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.
//
// CityHash, by Geoff Pike and Jyrki Alakuijala
//
// This file provides a few functions for hashing strings. On x86-64
// hardware in 2011, CityHash64() is faster than other high-quality
// hash functions, such as Murmur.  This is largely due to higher
// instruction-level parallelism.  CityHash64() and CityHash128() also perform
// well on hash-quality tests.
//
// CityHash128() is optimized for relatively long strings and returns
// a 128-bit hash.  For strings more than about 2000 bytes it can be
// faster than CityHash64().
//
// Functions in the CityHash family are not suitable for cryptography.
//
// WARNING: This code has not been tested on big-endian platforms!
// It is known to work well on little-endian platforms that have a small penalty
// for unaligned reads, such as current Intel and AMD moderate-to-high-end CPUs.
//
// By the way, for some hash functions, given strings a and b, the hash
// of a+b is easily derived from the hashes of a and b.  This property
// doesn't hold for any hash functions in this file.

#ifndef CITY_HASH_H_
#define CITY_HASH_H_

#include <stdlib.h>  // for size_t.
#include <stdint.h>
#include <utility>

#include <stdio.h>

/** This is a version of CityHash that predates v1.0.3 algorithm change.
  * Why we need exactly this version?
  * Although hash values of CityHash are not recommended for storing persistently anywhere,
  * it has already been used this way in ClickHouse:
  * - for calculation of checksums of compressed chunks and for data parts;
  * - this version of CityHash is exposed in cityHash64 function in ClickHouse SQL language;
  * - and already used by many users for data ordering, sampling and sharding.
  */
namespace CityHash_v1_0_2
{

typedef uint8_t uint8;
typedef uint32_t uint32;
typedef uint64_t uint64;

/// Represent an unsigned integer of 128 bits as it's used in CityHash.
/// Originally CityHash used `std::pair<uint64, uint64>` instead of this struct,
/// however the members `first` and `second` could be easily confused so they were renamed to `low64` and `high64`:
/// `first` -> `low64`, `second` -> `high64`.
struct uint128
{
    uint64 low64 = 0;
    uint64 high64 = 0;

    uint128() = default;
    uint128(uint64 low64_, uint64 high64_) : low64(low64_), high64(high64_) {}

    friend auto operator<=>(const uint128 &, const uint128 &) = default;
};

extern "C" inline uint64 Uint128Low64(const uint128 & x) { return x.low64; }
extern "C" inline uint64 Uint128High64(const uint128 & x) { return x.high64; }

// Hash function for a byte array.
extern "C" uint64 CityHash64(const char *buf, size_t len);

// Hash function for a byte array.  For convenience, a 64-bit seed is also
// hashed into the result.
uint64 CityHash64WithSeed(const char *buf, size_t len, uint64 seed);

// Hash function for a byte array.  For convenience, two seeds are also
// hashed into the result.
uint64 CityHash64WithSeeds(const char *buf, size_t len,
                           uint64 seed0, uint64 seed1);

// Hash function for a byte array.
uint128 CityHash128(const char *s, size_t len);

// Hash function for a byte array.  For convenience, a 128-bit seed is also
// hashed into the result.
uint128 CityHash128WithSeed(const char *s, size_t len, uint128 seed);

// Hash 128 input bits down to 64 bits of output.
// This is intended to be a reasonably good hash function.
inline uint64 Hash128to64(const uint128& x) {
  // Murmur-inspired hashing.
  const uint64 kMul = 0x9ddfea08eb382d69ULL;
  uint64 a = (Uint128Low64(x) ^ Uint128High64(x)) * kMul;
  a ^= (a >> 47);
  uint64 b = (Uint128High64(x) ^ a) * kMul;
  b ^= (b >> 47);
  b *= kMul;
  return b;
}

// See
// https://github.com/HUD-Software/cityhash-sys/blob/master/src/google/city.h
// Hash 128 usable with C compatible ABI
extern "C" void CityHash128(
    char const *s, 
    size_t len, 
    uint64* result_low_128, 
    uint64* result_high_128 ) {
  const uint128 result_128 = CityHash128(s, len);
  *result_low_128 = Uint128Low64(result_128);
  *result_high_128 = Uint128High64(result_128);
}

}

#endif  // CITY_HASH_H_
