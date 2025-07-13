// Converted from V8 C++ source files:
// Header: unicode.h
// Implementation: unicode.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod unibrow {
    pub type uchar = u32;

    /**
     * \file
     * Definitions and convenience functions for working with unicode.
     */

    /**
     * The max length of the result of converting the case of a single
     * character.
     */
    pub const K_MAX_MAPPING_SIZE: i32 = 4;

    #[derive(Debug)]
    pub enum UnicodeError {
        EncodingError,
        UnexpectedValue,
    }

    #[cfg(not(feature = "intl"))]
    pub mod not_intl {
        use super::uchar;

        const K_START_BIT: i32 = 1 << 30;
        const K_CHUNK_BITS: i32 = 1 << 13;

        #[allow(dead_code)]
        #[inline]
        fn table_get<const D: usize>(table: &[i32], index: usize) -> uchar {
            table[D * index] as uchar
        }

        #[allow(dead_code)]
        #[inline]
        fn get_entry(entry: i32) -> uchar {
            (entry & (K_START_BIT - 1)) as uchar
        }

        #[allow(dead_code)]
        #[inline]
        fn is_start(entry: i32) -> bool {
            (entry & K_START_BIT) != 0
        }

        #[allow(dead_code)]
        fn lookup_predicate(table: &[i32], size: u16, chr: uchar) -> bool {
            const K_ENTRY_DIST: usize = 1;
            let value: u16 = (chr & (K_CHUNK_BITS - 1) as u32) as u16;
            let mut low: usize = 0;
            let mut high: usize = (size - 1) as usize;

            while high != low {
                let mid: usize = low + ((high - low) >> 1);
                let current_value: uchar = super::unibrow::not_intl::get_entry(table_get::<K_ENTRY_DIST>(table, mid as usize) as i32);

                if (current_value <= value)
                    && (mid + 1 == size as usize
                        || super::unibrow::not_intl::get_entry(table_get::<K_ENTRY_DIST>(table, mid + 1) as i32) > value)
                {
                    low = mid;
                    break;
                } else if current_value < value {
                    low = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    high = mid - 1;
                }
            }

            let field: i32 = table_get::<K_ENTRY_DIST>(table, low) as i32;
            let entry: uchar = get_entry(field);
            let is_start: bool = is_start(field);

            (entry == value) || (entry < value && is_start)
        }

        const SENTINEL: uchar = u32::MAX;

        struct MultiCharacterSpecialCase<const KW: usize> {
            chars: [uchar; KW],
        }

        impl<const KW: usize> MultiCharacterSpecialCase<KW> {
            const K_END_OF_ENCODING: uchar = SENTINEL;
        }

        #[allow(dead_code)]
        fn lookup_mapping<const RANGES_ARE_LINEAR: bool, const KW: usize>(
            table: &[i32],
            size: u16,
            multi_chars: &[MultiCharacterSpecialCase<KW>],
            chr: uchar,
            next: uchar,
            result: &mut [uchar],
            allow_caching_ptr: Option<&mut bool>,
        ) -> i32 {
            const K_ENTRY_DIST: usize = 2;
            let key: u16 = (chr & (K_CHUNK_BITS - 1) as u32) as u16;
            let chunk_start: u32 = (chr as u32) - (key as u32);

            let mut low: usize = 0;
            let mut high: usize = (size - 1) as usize;

            while high != low {
                let mid: usize = low + ((high - low) >> 1);
                let current_value: uchar = super::unibrow::not_intl::get_entry(table_get::<K_ENTRY_DIST>(table, mid) as i32);

                if (current_value <= key)
                    && (mid + 1 == size as usize
                        || super::unibrow::not_intl::get_entry(table_get::<K_ENTRY_DIST>(table, mid + 1) as i32) > key)
                {
                    low = mid;
                    break;
                } else if current_value < key {
                    low = mid + 1;
                } else {
                    if mid == 0 {
                        break;
                    }
                    high = mid - 1;
                }
            }

            let field: i32 = table_get::<K_ENTRY_DIST>(table, low) as i32;
            let entry: uchar = get_entry(field);
            let is_start: bool = is_start(field);
            let found: bool = (entry == key) || (entry < key && is_start);

            if found {
                let value: i32 = table[2 * low + 1];
                if value == 0 {
                    return 0;
                } else if (value & 3) == 0 {
                    if RANGES_ARE_LINEAR {
                        result[0] = (chr as i32 + (value >> 2)) as uchar;
                    } else {
                        result[0] = (entry as u32 + chunk_start + ((value >> 2) as u32)) as uchar;
                    }
                    return 1;
                } else if (value & 3) == 1 {
                    if let Some(ptr) = allow_caching_ptr {
                        *ptr = false;
                    }

                    let mapping: &MultiCharacterSpecialCase<KW> = &multi_chars[(value >> 2) as usize];
                    let mut length: usize = 0;
                    for l in 0..KW {
                        let mapped: uchar = mapping.chars[l];
                        if mapped == MultiCharacterSpecialCase::<KW>::K_END_OF_ENCODING {
                            break;
                        }
                        if RANGES_ARE_LINEAR {
                            result[length] = (mapped as i32 + ((key as i32) - (entry as i32))) as uchar;
                        } else {
                            result[length] = mapped;
                        }
                        length += 1;
                    }
                    length as i32
                } else {
                    if let Some(ptr) = allow_caching_ptr {
                        *ptr = false;
                    }

                    match value >> 2 {
                        1 => {
                            if next != 0 && super::unibrow::not_intl::Letter::is(next) {
                                result[0] = 0x03C3;
                            } else {
                                result[0] = 0x03C2;
                            }
                            1
                        }
                        _ => 0,
                    }
                }
            } else {
                0
            }
        }

        pub mod Uppercase {
            use super::uchar;

            static K_UPPERCASE_TABLE0: &[i32] = &[
                1073741889, 90, 1073742016, 214, 1073742040, 222, 256, 258, 260, 262, 264, 266, 268, 270, 272, 274,
                276, 278, 280, 282, 284, 286, 288, 290, 292, 294, 296, 298, 300, 302, 304, 306, 308, 310, 313, 315,
                317, 319, 321, 323, 325, 327, 330, 332, 334, 336, 338, 340, 342, 344, 346, 348, 350, 352, 354, 356,
                358, 360, 362, 364, 366, 368, 370, 372, 374, 1073742200, 377, 379, 381, 1073742209, 386, 388,
                1073742214, 391, 1073742217, 395, 1073742222, 401, 1073742227, 404, 1073742230, 408, 1073742236, 413,
                1073742239, 416, 418, 420, 1073742246, 423, 425, 428, 1073742254, 431, 1073742257, 435, 437,
                1073742263, 440, 444, 452, 455, 458, 461, 463, 465, 467, 469, 471, 473, 475, 478, 480, 482, 484,
                486, 488, 490, 492, 494, 497, 500, 1073742326, 504, 506, 508, 510, 512, 514, 516, 518, 520, 522,
                524, 526, 528, 530, 532, 534, 536, 538, 540, 542, 544, 546, 548, 550, 552, 554, 556, 558, 560, 562,
                1073742394, 571, 1073742397, 574, 577, 1073742403, 582, 584, 586, 588, 590, 880, 882, 886, 895, 902,
                1073742728, 906, 908, 1073742734, 911, 1073742737, 929, 1073742755, 939, 975, 1073742802, 980, 984,
                986, 988, 990, 992, 994, 996, 998, 1000, 1002, 1004, 1006, 1012, 1015, 1073742841, 1018,
                1073742845, 1071, 1120, 1122, 1124, 1126, 1128, 1130, 1132, 1134, 1136, 1138, 1140, 1142, 1144,
                1146, 1148, 1150, 1152, 1162, 1164, 1166, 1168, 1170, 1172, 1174, 1176, 1178, 1180, 1182, 1184,
                1186, 1188, 1190, 1192, 1194, 1196, 1198, 1200, 1202, 1204, 1206, 1208, 1210, 1212, 1214,
                1073743040, 1217, 1219, 1221, 1223, 1225, 1227, 1229, 1232, 1234, 1236, 1238, 1240, 1242, 1244,
                1246, 1248, 1250, 1252, 1254, 1256, 1258, 1260, 1262, 1264, 1266, 1268, 1270, 1272, 1274, 1276,
                1278, 1280, 1282, 1284, 1286, 1288, 1290, 1292, 1294, 1296, 1298, 1300, 1302, 1304, 1306, 1308,
                1310, 1312, 1314, 1316, 1318, 1320, 1322, 1324, 1326, 1073743153, 1366, 1073746080, 4293, 4295, 4301,
                7680, 7682, 7684, 7686, 7688, 7690, 7692, 7694, 7696, 7698, 7700, 7702, 7704, 7706, 7708, 7710,
                7712, 7714, 7716, 7718, 7720, 7722, 7724, 7726, 7728, 7730, 7732, 7734, 7736, 7738, 7740, 7742,
                7744, 7746, 7748, 7750, 7752, 7754, 7756, 7758, 7760, 7762, 7764, 7766, 7768, 7770, 7772, 7774,
                7776, 7778, 7780, 7782, 7784, 7786, 7788, 7790, 7792, 7794, 7796, 7798, 7800, 7802, 7804, 7806,
                7808, 7810, 7812, 7814, 7816, 7818, 7820, 7822, 7824, 7826, 7828, 7838, 7840, 7842, 7844, 7846,
                7848, 7850, 7852, 7854, 7856, 7858, 7860, 7862, 7864, 7866, 7868, 7870, 7872, 7874, 7876, 7878,
                7880, 7882, 7884, 7886, 7888, 7890, 7892, 7894, 7896, 7898, 7900, 7902, 7904, 7906, 7908, 7910,
                7912, 7914, 7916, 7918, 7920, 7922, 7924, 7926, 7928, 7930, 7932, 7934, 1073749768, 7951, 1073749784,
                7965, 1073749800, 7983, 1073749816, 7999, 1073749832, 8013, 8025, 8027, 8029, 8031, 1073749864, 8047,
                1073749944, 8123, 1073749960, 8139, 1073749976, 8155, 1073749992, 8172, 1073750008, 8187,
            ];
            static K_UPPERCASE_TABLE1: &[i32] = &[
                258, 263, 1073742091, 269, 1073742096, 274, 277, 1073742105, 285, 292, 294, 296, 1073742122, 301,
                1073742128, 307, 1073742142, 319, 325, 387, 1073744896, 3118, 3168, 1073744994, 3172, 3175, 3177, 3179,
                1073745005, 3184, 3186, 3189, 1073745022, 3200, 3202, 3204, 3206, 3208, 3210, 3212, 3214, 3216,
                3218, 3220, 3222, 3224, 3226, 3228, 3230, 3232, 3234, 3236, 3238, 3240, 3242, 3244, 3246, 3248,
                3250, 3252, 3254, 3256, 3258, 3260, 3262, 3264, 3266, 3268, 3270, 3272, 3274, 3276, 3278, 3280,
                3282, 3284, 3286, 3288, 3290, 3292, 3294, 3296, 3298, 3307, 3309, 3314,
            ];
            static K_UPPERCASE_TABLE5: &[i32] = &[
                1600, 1602, 1604, 1606, 1608, 1610, 1612, 1614, 1616, 1618, 1620, 1622, 1624, 1626, 1628, 1630,
                1632, 1634, 1636, 1638, 1640, 1642, 1644, 1664, 1666, 1668, 1670, 1672, 1674, 1676, 1678, 1680,
                1682, 1684, 1686, 1688, 1690, 1826, 1828, 1830, 1832, 1834, 1836, 1838, 1842, 1844, 1846, 1848,
                1850, 1852, 1854, 1856, 1858, 1860, 1862, 1864, 1866, 1868, 1870, 1872, 1874, 1876, 1878, 1880,
                1882, 1884, 1886, 1888, 1890, 1892, 1894, 1896, 1898, 1900, 1902, 1913, 1915, 1073743741, 1918,
                1920, 1922, 1924, 1926, 1931, 1933, 1936, 1938, 1942, 1944, 1946, 1948, 1950, 1952, 1954, 1956,
                1958, 1960, 1073743786, 1965, 1073743792, 1969,
            ];
            static K_UPPERCASE_TABLE7: &[i32] = &[1073749793, 7994];
            const K_UPPERCASE_TABLE0_SIZE: u16 = 455;
            const K_UPPERCASE_TABLE1_SIZE: u16 = 86;
            const K_UPPERCASE_TABLE5_SIZE: u16 = 101;
            const K_UPPERCASE_TABLE7_SIZE: u16 = 2;
            #[allow(dead_code)]
            pub fn is(c: uchar) -> bool {
                let chunk_index = c >> 13;
                match chunk_index {
                    0 => super::unibrow::not_intl::lookup_predicate(K_UPPERCASE_TABLE0, K_UPPERCASE_TABLE0_SIZE, c),
                    1 => super::unibrow::not_intl::lookup_predicate(K_UPPERCASE_TABLE1, K_UPPERCASE_TABLE1_SIZE, c),
                    5 => super::unibrow::not_intl::lookup_predicate(K_UPPERCASE_TABLE5, K_UPPERCASE_TABLE5_SIZE, c),
                    7 => super::unibrow::not_intl::lookup_predicate(K_UPPERCASE_TABLE7, K_UPPERCASE_TABLE7_SIZE, c),
                    _ => false,
                }
            }
        }

        pub mod Letter {
            use super::uchar;

            static K_LETTER_TABLE0: &[i32] = &[
                1073741889, 90, 1073741921, 122, 170, 181, 186, 1073742016, 214, 1073742040, 246, 1073742072, 705,
                1073742534, 721, 1073742560, 740, 748, 750, 1073742704, 884, 1073742710, 887, 1073742714, 893, 895,
                902, 1073742728, 906, 908, 1073742734, 929, 1073742755, 1013, 1073742839, 1153, 1073742986, 1327,
                1073743153, 1366, 1369, 1073743201, 1415, 1073743312, 1514, 1073743344, 1522, 1073743392, 1610,
                1073743470, 1647, 1073743473, 1747, 1749, 1073743589, 1766, 1073743598, 1775, 1073743610, 1788,
                1791, 1808, 1073743634, 1839, 1073743693, 1957, 1969, 1073743818, 2026, 1073743860, 2037, 2042,
                1073743872, 2069, 2074, 2084, 2088, 1073743936, 2136, 1073744032, 2226, 1073744132, 2361, 2365,
                2384, 1073744216, 2401, 1073744241, 2432, 1073744261, 2444, 1073744271, 2448, 1073744275, 2472,
                1073744298, 2480, 2482, 1073744310, 2489, 2493, 2510, 1073744348, 2525, 1073744351, 2529,
                1073744368, 2545, 1073744389, 2570, 1073744399, 2576, 1073744403, 2600, 1073744426, 2608,
                1073744434, 2611, 1073744437, 2614, 1073744440, 2617, 1073744473, 2652, 2654, 1073744498, 2676,
                1073744517, 2701, 1073744527, 2705, 1073744531, 2728, 1073744554, 2736, 1073744562, 2739,
                1073744565, 2745, 2749, 2768, 1073744608, 2785, 1073744645, 2828, 1073744655, 2832, 1073744659,
                2856, 1073744682, 2864, 1073744690, 2867, 1073744693, 2873, 2877, 1073744732, 2909, 1073744735,
                2913, 2929, 2947, 1073744773, 2954, 1073744782, 2960, 1073744786, 2965, 1073744793, 2970, 2972,
                1073744798, 2975, 1073744803, 2980, 1073744808, 2986, 1073744814, 3001, 3024, 1073744901, 3084,
                1073744910, 3088, 1073744914, 3112, 1073744938, 3129, 3133, 1073744984, 3161, 1073744992, 3169,
                1073745029, 3212, 1073745038, 3216, 1073745042, 3240, 1073745066, 3251, 1073745077, 3257, 3261,
                3294, 1073745120, 3297, 1073745137, 3314, 1073745157, 3340, 1073745166, 3344, 1073745170, 3386,
                3389, 3406, 1073745248, 3425, 1073745274, 3455, 1073745285, 3478, 1073745306, 350
