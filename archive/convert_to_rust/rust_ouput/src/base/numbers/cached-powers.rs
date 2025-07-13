// Converted from V8 C++ source files:
// Header: cached-powers.h
// Implementation: cached-powers.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod numbers {
use std::f64;

pub struct DiyFp {
        pub f: u64,
        pub e: i32,
    }

    impl DiyFp {
        pub fn new(f: u64, e: i32) -> Self {
            DiyFp { f, e }
        }

        // Adjust kSignificandSize if needed
        pub const kSignificandSize: i32 = 64;
    }

pub struct PowersOfTenCache {}

impl PowersOfTenCache {
    // Not all powers of ten are cached. The decimal exponent of two neighboring
    // cached numbers will differ by kDecimalExponentDistance.
    pub const kDecimalExponentDistance: i32 = 8;

    pub const kMinDecimalExponent: i32 = -348;
    pub const kMaxDecimalExponent: i32 = 340;

    // Returns a cached power-of-ten with a binary exponent in the range
    // [min_exponent; max_exponent] (boundaries included).
    pub fn get_cached_power_for_binary_exponent_range(
        min_exponent: i32,
        max_exponent: i32,
        power: &mut DiyFp,
        decimal_exponent: &mut i32,
    ) {
        let k_q = DiyFp::kSignificandSize;
        // Some platforms return incorrect sign on 0 result. We can ignore that here,
        // which means we can avoid depending on platform.h.
        let k = ((min_exponent as f64 + k_q as f64 - 1.0) * kD_1_LOG2_10).ceil();
        let foo = K_CACHED_POWERS_OFFSET;
        let index = (foo + k as i32 - 1) / Self::kDecimalExponentDistance + 1;

        let cached_power = K_CACHED_POWERS[index as usize];
        
        *decimal_exponent = cached_power.decimal_exponent;
        *power = DiyFp::new(cached_power.significand, cached_power.binary_exponent as i32);
    }

    // Returns a cached power of ten x ~= 10^k such that
    //   k <= decimal_exponent < k + kCachedPowersDecimalDistance.
    // The given decimal_exponent must satisfy
    //   kMinDecimalExponent <= requested_exponent, and
    //   requested_exponent < kMaxDecimalExponent + kDecimalExponentDistance.
    pub fn get_cached_power_for_decimal_exponent(
        requested_exponent: i32,
        power: &mut DiyFp,
        found_exponent: &mut i32,
    ) {
        let index =
            (requested_exponent + K_CACHED_POWERS_OFFSET) / Self::kDecimalExponentDistance;
        let cached_power = K_CACHED_POWERS[index as usize];
        *power = DiyFp::new(cached_power.significand, cached_power.binary_exponent as i32);
        *found_exponent = cached_power.decimal_exponent;
    }
}

#[derive(Clone, Copy)]
struct CachedPower {
    significand: u64,
    binary_exponent: i16,
    decimal_exponent: i16,
}

const K_CACHED_POWERS: [CachedPower; 69] = [
    CachedPower {
        significand: 0xFA8FD5A0081C0288,
        binary_exponent: -1220,
        decimal_exponent: -348,
    },
    CachedPower {
        significand: 0xBAAEE17FA23EBF76,
        binary_exponent: -1193,
        decimal_exponent: -340,
    },
    CachedPower {
        significand: 0x8B16FB203055AC76,
        binary_exponent: -1166,
        decimal_exponent: -332,
    },
    CachedPower {
        significand: 0xCF42894A5DCE35EA,
        binary_exponent: -1140,
        decimal_exponent: -324,
    },
    CachedPower {
        significand: 0x9A6BB0AA55653B2D,
        binary_exponent: -1113,
        decimal_exponent: -316,
    },
    CachedPower {
        significand: 0xE61ACF033D1A45DF,
        binary_exponent: -1087,
        decimal_exponent: -308,
    },
    CachedPower {
        significand: 0xAB70FE17C79AC6CA,
        binary_exponent: -1060,
        decimal_exponent: -300,
    },
    CachedPower {
        significand: 0xFF77B1FCBEBCDC4F,
        binary_exponent: -1034,
        decimal_exponent: -292,
    },
    CachedPower {
        significand: 0xBE5691EF416BD60C,
        binary_exponent: -1007,
        decimal_exponent: -284,
    },
    CachedPower {
        significand: 0x8DD01FAD907FFC3C,
        binary_exponent: -980,
        decimal_exponent: -276,
    },
    CachedPower {
        significand: 0xD3515C2831559A83,
        binary_exponent: -954,
        decimal_exponent: -268,
    },
    CachedPower {
        significand: 0x9D71AC8FADA6C9B5,
        binary_exponent: -927,
        decimal_exponent: -260,
    },
    CachedPower {
        significand: 0xEA9C227723EE8BCB,
        binary_exponent: -901,
        decimal_exponent: -252,
    },
    CachedPower {
        significand: 0xAECC49914078536D,
        binary_exponent: -874,
        decimal_exponent: -244,
    },
    CachedPower {
        significand: 0x823C12795DB6CE57,
        binary_exponent: -847,
        decimal_exponent: -236,
    },
    CachedPower {
        significand: 0xC21094364DFB5637,
        binary_exponent: -821,
        decimal_exponent: -228,
    },
    CachedPower {
        significand: 0x9096EA6F3848984F,
        binary_exponent: -794,
        decimal_exponent: -220,
    },
    CachedPower {
        significand: 0xD77485CB25823AC7,
        binary_exponent: -768,
        decimal_exponent: -212,
    },
    CachedPower {
        significand: 0xA086CFCD97BF97F4,
        binary_exponent: -741,
        decimal_exponent: -204,
    },
    CachedPower {
        significand: 0xEF340A98172AACE5,
        binary_exponent: -715,
        decimal_exponent: -196,
    },
    CachedPower {
        significand: 0xB23867FB2A35B28E,
        binary_exponent: -688,
        decimal_exponent: -188,
    },
    CachedPower {
        significand: 0x84C8D4DFD2C63F3B,
        binary_exponent: -661,
        decimal_exponent: -180,
    },
    CachedPower {
        significand: 0xC5DD44271AD3CDBA,
        binary_exponent: -635,
        decimal_exponent: -172,
    },
    CachedPower {
        significand: 0x936B9FCEBB25C996,
        binary_exponent: -608,
        decimal_exponent: -164,
    },
    CachedPower {
        significand: 0xDBAC6C247D62A584,
        binary_exponent: -582,
        decimal_exponent: -156,
    },
    CachedPower {
        significand: 0xA3AB66580D5FDAF6,
        binary_exponent: -555,
        decimal_exponent: -148,
    },
    CachedPower {
        significand: 0xF3E2F893DEC3F126,
        binary_exponent: -529,
        decimal_exponent: -140,
    },
    CachedPower {
        significand: 0xB5B5ADA8AAFF80B8,
        binary_exponent: -502,
        decimal_exponent: -132,
    },
    CachedPower {
        significand: 0x87625F056C7C4A8B,
        binary_exponent: -475,
        decimal_exponent: -124,
    },
    CachedPower {
        significand: 0xC9BCFF6034C13053,
        binary_exponent: -449,
        decimal_exponent: -116,
    },
    CachedPower {
        significand: 0x964E858C91BA2655,
        binary_exponent: -422,
        decimal_exponent: -108,
    },
    CachedPower {
        significand: 0xDFF9772470297EBD,
        binary_exponent: -396,
        decimal_exponent: -100,
    },
    CachedPower {
        significand: 0xA6DFBD9FB8E5B88F,
        binary_exponent: -369,
        decimal_exponent: -92,
    },
    CachedPower {
        significand: 0xF8A95FCF88747D94,
        binary_exponent: -343,
        decimal_exponent: -84,
    },
    CachedPower {
        significand: 0xB94470938FA89BCF,
        binary_exponent: -316,
        decimal_exponent: -76,
    },
    CachedPower {
        significand: 0x8A08F0F8BF0F156B,
        binary_exponent: -289,
        decimal_exponent: -68,
    },
    CachedPower {
        significand: 0xCDB02555653131B6,
        binary_exponent: -263,
        decimal_exponent: -60,
    },
    CachedPower {
        significand: 0x993FE2C6D07B7FAC,
        binary_exponent: -236,
        decimal_exponent: -52,
    },
    CachedPower {
        significand: 0xE45C10C42A2B3B06,
        binary_exponent: -210,
        decimal_exponent: -44,
    },
    CachedPower {
        significand: 0xAA242499697392D3,
        binary_exponent: -183,
        decimal_exponent: -36,
    },
    CachedPower {
        significand: 0xFD87B5F28300CA0E,
        binary_exponent: -157,
        decimal_exponent: -28,
    },
    CachedPower {
        significand: 0xBCE5086492111AEB,
        binary_exponent: -130,
        decimal_exponent: -20,
    },
    CachedPower {
        significand: 0x8CBCCC096F5088CC,
        binary_exponent: -103,
        decimal_exponent: -12,
    },
    CachedPower {
        significand: 0xD1B71758E219652C,
        binary_exponent: -77,
        decimal_exponent: -4,
    },
    CachedPower {
        significand: 0x9C40000000000000,
        binary_exponent: -50,
        decimal_exponent: 4,
    },
    CachedPower {
        significand: 0xE8D4A51000000000,
        binary_exponent: -24,
        decimal_exponent: 12,
    },
    CachedPower {
        significand: 0xAD78EBC5AC620000,
        binary_exponent: 3,
        decimal_exponent: 20,
    },
    CachedPower {
        significand: 0x813F3978F8940984,
        binary_exponent: 30,
        decimal_exponent: 28,
    },
    CachedPower {
        significand: 0xC097CE7BC90715B3,
        binary_exponent: 56,
        decimal_exponent: 36,
    },
    CachedPower {
        significand: 0x8F7E32CE7BEA5C70,
        binary_exponent: 83,
        decimal_exponent: 44,
    },
    CachedPower {
        significand: 0xD5D238A4ABE98068,
        binary_exponent: 109,
        decimal_exponent: 52,
    },
    CachedPower {
        significand: 0x9F4F2726179A2245,
        binary_exponent: 136,
        decimal_exponent: 60,
    },
    CachedPower {
        significand: 0xED63A231D4C4FB27,
        binary_exponent: 162,
        decimal_exponent: 68,
    },
    CachedPower {
        significand: 0xB0DE65388CC8ADA8,
        binary_exponent: 189,
        decimal_exponent: 76,
    },
    CachedPower {
        significand: 0x83C7088E1AAB65DB,
        binary_exponent: 216,
        decimal_exponent: 84,
    },
    CachedPower {
        significand: 0xC45D1DF942711D9A,
        binary_exponent: 242,
        decimal_exponent: 92,
    },
    CachedPower {
        significand: 0x924D692CA61BE758,
        binary_exponent: 269,
        decimal_exponent: 100,
    },
    CachedPower {
        significand: 0xDA01EE641A708DEA,
        binary_exponent: 295,
        decimal_exponent: 108,
    },
    CachedPower {
        significand: 0xA26DA3999AEF774A,
        binary_exponent: 322,
        decimal_exponent: 116,
    },
    CachedPower {
        significand: 0xF209787BB47D6B85,
        binary_exponent: 348,
        decimal_exponent: 124,
    },
    CachedPower {
        significand: 0xB454E4A179DD1877,
        binary_exponent: 375,
        decimal_exponent: 132,
    },
    CachedPower {
        significand: 0x865B86925B9BC5C2,
        binary_exponent: 402,
        decimal_exponent: 140,
    },
    CachedPower {
        significand: 0xC83553C5C8965D3D,
        binary_exponent: 428,
        decimal_exponent: 148,
    },
    CachedPower {
        significand: 0x952AB45CFA97A0B3,
        binary_exponent: 455,
        decimal_exponent: 156,
    },
    CachedPower {
        significand: 0xDE469FBD99A05FE3,
        binary_exponent: 481,
        decimal_exponent: 164,
    },
    CachedPower {
        significand: 0xA59BC234DB398C25,
        binary_exponent: 508,
        decimal_exponent: 172,
    },
    CachedPower {
        significand: 0xF6C69A72A3989F5C,
        binary_exponent: 534,
        decimal_exponent: 180,
    },
    CachedPower {
        significand: 0xB7DCBF5354E9BECE,
        binary_exponent: 561,
        decimal_exponent: 188,
    },
    CachedPower {
        significand: 0x88FCF317F22241E2,
        binary_exponent: 588,
        decimal_exponent: 196,
    },
    CachedPower {
        significand: 0xCC20CE9BD35C78A5,
        binary_exponent: 614,
        decimal_exponent: 204,
    },
    CachedPower {
        significand: 0x98165AF37B2153DF,
        binary_exponent: 641,
        decimal_exponent: 212,
    },
    CachedPower {
        significand: 0xE2A0B5DC971F303A,
        binary_exponent: 667,
        decimal_exponent: 220,
    },
    CachedPower {
        significand: 0xA8D9D1535CE3B396,
        binary_exponent: 694,
        decimal_exponent: 228,
    },
    CachedPower {
        significand: 0xFB9B7CD9A4A7443C,
        binary_exponent: 720,
        decimal_exponent: 236,
    },
    CachedPower {
        significand: 0xBB764C4CA7A44410,
        binary_exponent: 747,
        decimal_exponent: 244,
    },
    CachedPower {
        significand: 0x8BAB8EEFB6409C1A,
        binary_exponent: 774,
        decimal_exponent: 252,
    },
    CachedPower {
        significand: 0xD01FEF10A657842C,
        binary_exponent: 800,
        decimal_exponent: 260,
    },
    CachedPower {
        significand: 0x9B10A4E5E9913129,
        binary_exponent: 827,
        decimal_exponent: 268,
    },
    CachedPower {
        significand: 0xE7109BFBA19C0C9D,
        binary_exponent: 853,
        decimal_exponent: 276,
    },
    CachedPower {
        significand: 0xAC2820D9623BF429,
        binary_exponent: 880,
        decimal_exponent: 284,
    },
    CachedPower {
        significand: 0x80444B5E7AA7CF85,
        binary_exponent: 907,
        decimal_exponent: 292,
    },
    CachedPower {
        significand: 0xBF21E44003ACDD2D,
        binary_exponent: 933,
        decimal_exponent: 300,
    },
    CachedPower {
        significand: 0x8E679C2F5E44FF8F,
        binary_exponent: 960,
        decimal_exponent: 308,
    },
    CachedPower {
        significand: 0xD433179D9C8CB841,
        binary_exponent: 986,
        decimal_exponent: 316,
    },
    CachedPower {
        significand: 0x9E19DB92B4E31BA9,
        binary_exponent: 1013,
        decimal_exponent: 324,
    },
    CachedPower {
        significand: 0xEB96BF6EBADF77D9,
        binary_exponent: 1039,
        decimal_exponent: 332,
    },
    CachedPower {
        significand: 0xAF87023B9BF0EE6B,
        binary_exponent: 1066,
        decimal_exponent: 340,
    },
];

const K_CACHED_POWERS_OFFSET: i32 = 348; // -1 * the first decimal_exponent.
const kD_1_LOG2_10: f64 = 0.30102999566398114; //  1 / lg(10)

} // namespace numbers
} // namespace v8
