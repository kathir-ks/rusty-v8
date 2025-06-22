// Copyright 2015 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#[macro_export]
macro_rules! define_string_constants {
    ($(#[$attr:meta])*
     $vis:vis mod $module_name:ident {
        $($(#[$item_attr:meta])*
           ($item_vis:vis const $name:ident, $value:expr);)*
    }) => {
        $(#[$attr])*
        $vis mod $module_name {
            $(
                $(#[$item_attr])*
                $item_vis const $name: &str = $value;
            )*
        }
    };
}

#[cfg(feature = "intl")]
define_string_constants! {
    /// Constants related to internationalization support.
    pub mod intl_string_constants {
        (pub const adoptText_string, "adoptText");
        (pub const approximatelySign_string, "approximatelySign");
        (pub const baseName_string, "baseName");
        (pub const accounting_string, "accounting");
        (pub const breakType_string, "breakType");
        (pub const calendars_string, "calendars");
        (pub const cardinal_string, "cardinal");
        (pub const caseFirst_string, "caseFirst");
        (pub const ceil_string, "ceil");
        (pub const compare_string, "compare");
        (pub const collation_string, "collation");
        (pub const collations_string, "collations");
        (pub const compact_string, "compact");
        (pub const compactDisplay_string, "compactDisplay");
        (pub const currency_string, "currency");
        (pub const currencyDisplay_string, "currencyDisplay");
        (pub const currencySign_string, "currencySign");
        (pub const dateStyle_string, "dateStyle");
        (pub const dateTimeField_string, "dateTimeField");
        (pub const dayPeriod_string, "dayPeriod");
        (pub const daysDisplay_string, "daysDisplay");
        (pub const decimal_string, "decimal");
        (pub const dialect_string, "dialect");
        (pub const digital_string, "digital");
        (pub const direction_string, "direction");
        (pub const endRange_string, "endRange");
        (pub const engineering_string, "engineering");
        (pub const exceptZero_string, "exceptZero");
        (pub const expand_string, "expand");
        (pub const exponentInteger_string, "exponentInteger");
        (pub const exponentMinusSign_string, "exponentMinusSign");
        (pub const exponentSeparator_string, "exponentSeparator");
        (pub const fallback_string, "fallback");
        (pub const first_string, "first");
        (pub const firstDay_string, "firstDay");
        (pub const firstDayOfWeek_string, "firstDayOfWeek");
        (pub const floor_string, "floor");
        (pub const format_string, "format");
        (pub const fraction_string, "fraction");
        (pub const fractionalDigits_string, "fractionalDigits");
        (pub const fractionalSecond_string, "fractionalSecond");
        (pub const full_string, "full");
        (pub const granularity_string, "granularity");
        (pub const grapheme_string, "grapheme");
        (pub const group_string, "group");
        (pub const h11_string, "h11");
        (pub const h12_string, "h12");
        (pub const h23_string, "h23");
        (pub const h24_string, "h24");
        (pub const halfCeil_string, "halfCeil");
        (pub const halfEven_string, "halfEven");
        (pub const halfExpand_string, "halfExpand");
        (pub const halfFloor_string, "halfFloor");
        (pub const halfTrunc_string, "halfTrunc");
        (pub const hour12_string, "hour12");
        (pub const hourCycle_string, "hourCycle");
        (pub const hourCycles_string, "hourCycles");
        (pub const hoursDisplay_string, "hoursDisplay");
        (pub const ideo_string, "ideo");
        (pub const ignorePunctuation_string, "ignorePunctuation");
        (pub const Invalid_Date_string, "Invalid Date");
        (pub const integer_string, "integer");
        (pub const isWordLike_string, "isWordLike");
        (pub const kana_string, "kana");
        (pub const language_string, "language");
        (pub const languageDisplay_string, "languageDisplay");
        (pub const lessPrecision_string, "lessPrecision");
        (pub const letter_string, "letter");
        (pub const list_string, "list");
        (pub const literal_string, "literal");
        (pub const locale_string, "locale");
        (pub const loose_string, "loose");
        (pub const lower_string, "lower");
        (pub const ltr_string, "ltr");
        (pub const maximumFractionDigits_string, "maximumFractionDigits");
        (pub const maximumSignificantDigits_string, "maximumSignificantDigits");
        (pub const microsecondsDisplay_string, "microsecondsDisplay");
        (pub const millisecondsDisplay_string, "millisecondsDisplay");
        (pub const min2_string, "min2");
        (pub const minimalDays_string, "minimalDays");
        (pub const minimumFractionDigits_string, "minimumFractionDigits");
        (pub const minimumIntegerDigits_string, "minimumIntegerDigits");
        (pub const minimumSignificantDigits_string, "minimumSignificantDigits");
        (pub const minus_0, "-0");
        (pub const minusSign_string, "minusSign");
        (pub const minutesDisplay_string, "minutesDisplay");
        (pub const monthsDisplay_string, "monthsDisplay");
        (pub const morePrecision_string, "morePrecision");
        (pub const nan_string, "nan");
        (pub const nanosecondsDisplay_string, "nanosecondsDisplay");
        (pub const narrowSymbol_string, "narrowSymbol");
        (pub const negative_string, "negative");
        (pub const never_string, "never");
        (pub const none_string, "none");
        (pub const notation_string, "notation");
        (pub const normal_string, "normal");
        (pub const numberingSystem_string, "numberingSystem");
        (pub const numberingSystems_string, "numberingSystems");
        (pub const numeric_string, "numeric");
        (pub const ordinal_string, "ordinal");
        (pub const percentSign_string, "percentSign");
        (pub const plusSign_string, "plusSign");
        (pub const quarter_string, "quarter");
        (pub const region_string, "region");
        (pub const relatedYear_string, "relatedYear");
        (pub const roundingMode_string, "roundingMode");
        (pub const roundingPriority_string, "roundingPriority");
        (pub const rtl_string, "rtl");
        (pub const scientific_string, "scientific");
        (pub const secondsDisplay_string, "secondsDisplay");
        (pub const segment_string, "segment");
        (pub const SegmentIterator_string, "Segment Iterator");
        (pub const Segments_string, "Segments");
        (pub const sensitivity_string, "sensitivity");
        (pub const sep_string, "sep");
        (pub const shared_string, "shared");
        (pub const signDisplay_string, "signDisplay");
        (pub const standard_string, "standard");
        (pub const startRange_string, "startRange");
        (pub const strict_string, "strict");
        (pub const stripIfInteger_string, "stripIfInteger");
        (pub const style_string, "style");
        (pub const term_string, "term");
        (pub const textInfo_string, "textInfo");
        (pub const timeStyle_string, "timeStyle");
        (pub const timeZones_string, "timeZones");
        (pub const timeZoneName_string, "timeZoneName");
        (pub const trailingZeroDisplay_string, "trailingZeroDisplay");
        (pub const trunc_string, "trunc");
        (pub const two_digit_string, "2-digit");
        (pub const type_string, "type");
        (pub const unknown_string, "unknown");
        (pub const upper_string, "upper");
        (pub const usage_string, "usage");
        (pub const useGrouping_string, "useGrouping");
        (pub const unitDisplay_string, "unitDisplay");
        (pub const weekday_string, "weekday");
        (pub const weekend_string, "weekend");
        (pub const weeksDisplay_string, "weeksDisplay");
        (pub const weekInfo_string, "weekInfo");
        (pub const yearName_string, "yearName");
        (pub const yearsDisplay_string, "yearsDisplay");
    }
}

define_string_constants! {
    /// Constants for extra important internal strings.
    pub mod extra_important_string_constants {
        (pub const empty_string, "");
    }
}

define_string_constants! {
    /// Constants for important internal strings.
    pub mod important_string_constants {
        (pub const prototype_string, "prototype");
        (pub const name_string, "name");
        (pub const enumerable_string, "enumerable");
        (pub const configurable_string, "configurable");
        (pub const value_string, "value");
        (pub const writable_string, "writable");
    }
}

define_string_constants! {
    /// Constants for single character ASCII internal strings.
    pub mod single_character_ascii_string_constants {
        (pub const ascii_nul_string, "\0");
        (pub const ascii_soh_string, "\x01");
        (pub const ascii_stx_string, "\x02");
        (pub const ascii_etx_string, "\x03");
        (pub const ascii_eot_string, "\x04");
        (pub const ascii_enq_string, "\x05");
        (pub const ascii_ack_string, "\x06");
        (pub const ascii_bel_string, "\x07");
        (pub const ascii_bs_string, "\x08");
        (pub const ascii_ht_string, "\t");
        (pub const ascii_lf_string, "\n");
        (pub const ascii_vt_string, "\x0b");
        (pub const ascii_ff_string, "\x0c");
        (pub const ascii_cr_string, "\r");
        (pub const ascii_so_string, "\x0e");
        (pub const ascii_si_string, "\x0f");
        (pub const ascii_dle_string, "\x10");
        (pub const ascii_dc1_string, "\x11");
        (pub const ascii_dc2_string, "\x12");
        (pub const ascii_dc3_string, "\x13");
        (pub const ascii_dc4_string, "\x14");
        (pub const ascii_nak_string, "\x15");
        (pub const ascii_syn_string, "\x16");
        (pub const ascii_etb_string, "\x17");
        (pub const ascii_can_string, "\x18");
        (pub const ascii_em_string, "\x19");
        (pub const ascii_sub_string, "\x1a");
        (pub const ascii_esc_string, "\x1b");
        (pub const ascii_fs_string, "\x1c");
        (pub const ascii_gs_string, "\x1d");
        (pub const ascii_rs_string, "\x1e");
        (pub const ascii_us_string, "\x1f");
        (pub const space_string, " ");
        (pub const exclamation_mark_string, "!");
        (pub const double_quotes_string, "\"");
        (pub const hash_string, "#");
        (pub const dollar_string, "$");
        (pub const percent_sign_string, "%");
        (pub const ampersand_string, "&");
        (pub const single_quote_string, "'");
        (pub const open_parenthesis_string, "(");
        (pub const close_parenthesis_string, ")");
        (pub const asterisk_string, "*");
        (pub const plus_string, "+");
        (pub const comma_string, ",");
        (pub const minus_string, "-");
        (pub const dot_string, ".");
        (pub const slash_string, "/");
        (pub const zero_string, "0");
        (pub const one_string, "1");
        (pub const two_string, "2");
        (pub const three_string, "3");
        (pub const four_string, "4");
        (pub const five_string, "5");
        (pub const six_string, "6");
        (pub const seven_string, "7");
        (pub const eight_string, "8");
        (pub const nine_string, "9");
        (pub const colon_string, ":");
        (pub const semicolon_string, ";");
        (pub const less_than_string, "<");
        (pub const equals_string, "=");
        (pub const greater_than_string, ">");
        (pub const question_mark_string, "?");
        (pub const at_sign_string, "@");
        (pub const A_string, "A");
        (pub const B_string, "B");
        (pub const C_string, "C");
        (pub const D_string, "D");
        (pub const E_string, "E");
        (pub const F_string, "F");
        (pub const G_string, "G");
        (pub const H_string, "H");
        (pub const I_string, "I");
        (pub const J_string, "J");
        (pub const K_string, "K");
        (pub const L_string, "L");
        (pub const M_string, "M");
        (pub const N_string, "N");
        (pub const O_string, "O");
        (pub const P_string, "P");
        (pub const Q_string, "Q");
        (pub const R_string, "R");
        (pub const S_string, "S");
        (pub const T_string, "T");
        (pub const U_string, "U");
        (pub const V_string, "V");
        (pub const W_string, "W");
        (pub const X_string, "X");
        (pub const Y_string, "Y");
        (pub const Z_string, "Z");
        (pub const open_bracket_string, "[");
        (pub const backslash_string, "\\");
        (pub const close_bracket_string, "]");
        (pub const caret_string, "^");
        (pub const underscore_string, "_");
        (pub const backtick_string, "`");
        (pub const a_string, "a");
        (pub const b_string, "b");
        (pub const c_string, "c");
        (pub const d_string, "d");
        (pub const e_string, "e");
        (pub const f_string, "f");
        (pub const g_string, "g");
        (pub const h_string, "h");
        (pub const i_string, "i");
        (pub const j_string, "j");
        (pub const k_string, "k");
        (pub const l_string, "l");
        (pub const m_string, "m");
        (pub const n_string, "n");
        (pub const o_string, "o");
        (pub const p_string, "p");
        (pub const q_string, "q");
        (pub const r_string, "r");
        (pub const s_string, "s");
        (pub const t_string, "t");
        (pub const u_string, "u");
        (pub const v_string, "v");
        (pub const w_string, "w");
        (pub const x_string, "x");
        (pub const y_string, "y");
        (pub const z_string, "z");
        (pub const open_brace_string, "{");
        (pub const pipe_string, "|");
        (pub const close_brace_string, "}");
        (pub const tilde_string, "~");
        (pub const ascii_del_string, "\x7f");
    }
}

define_string_constants! {
    /// Constants for single character internal strings.
    pub mod single_character_string_constants {
        (pub const ascii_nul_string, "\0");
        (pub const ascii_soh_string, "\x01");
        (pub const ascii_stx_string, "\x02");
        (pub const ascii_etx_string, "\x03");
        (pub const ascii_eot_string, "\x04");
        (pub const ascii_enq_string, "\x05");
        (pub const ascii_ack_string, "\x06");
        (pub const ascii_bel_string, "\x07");
        (pub const ascii_bs_string, "\x08");
        (pub const ascii_ht_string, "\t");
        (pub const ascii_lf_string, "\n");
        (pub const ascii_vt_string, "\x0b");
        (pub const ascii_ff_string, "\x0c");
        (pub const ascii_cr_string, "\r");
        (pub const ascii_so_string, "\x0e");
        (pub const ascii_si_string, "\x0f");
        (pub const ascii_dle_string, "\x10");
        (pub const ascii_dc1_string, "\x11");
        (pub const ascii_dc2_string, "\x12");
        (pub const ascii_dc3_string, "\x13");
        (pub const ascii_dc4_string, "\x14");
        (pub const ascii_nak_string, "\x15");
        (pub const ascii_syn_string, "\x16");
        (pub const ascii_etb_string, "\x17");
        (pub const ascii_can_string, "\x18");
        (pub const ascii_em_string, "\x19");
        (pub const ascii_sub_string, "\x1a");
        (pub const ascii_esc_string, "\x1b");
        (pub const ascii_fs_string, "\x1c");
        (pub const ascii_gs_string, "\x1d");
        (pub const ascii_rs_string, "\x1e");
        (pub const ascii_us_string, "\x1f");
        (pub const space_string, " ");
        (pub const exclamation_mark_string, "!");
        (pub const double_quotes_string, "\"");
        (pub const hash_string, "#");
        (pub const dollar_string, "$");
        (pub const percent_sign_string, "%");
        (pub const ampersand_string, "&");
        (pub const single_quote_string, "'");
        (pub const open_parenthesis_string, "(");
        (pub const close_parenthesis_string, ")");
        (pub const asterisk_string, "*");
        (pub const plus_string, "+");
        (pub const comma_string, ",");
        (pub const minus_string, "-");
        (pub const dot_string, ".");
        (pub const slash_string, "/");
        (pub const zero_string, "0");
        (pub const one_string, "1");
        (pub const two_string, "2");
        (pub const three_string, "3");
        (pub const four_string, "4");
        (pub const five_string, "5");
        (pub const six_string, "6");
        (pub const seven_string, "7");
        (pub const eight_string, "8");
        (pub const nine_string, "9");
        (pub const colon_string, ":");
        (pub const semicolon_string, ";");
        (pub const less_than_string, "<");
        (pub const equals_string, "=");
        (pub const greater_than_string, ">");
        (pub const question_mark_string, "?");
        (pub const at_sign_string, "@");
        (pub const A_string, "A");
        (pub const B_string, "B");
        (pub const C_string, "C");
        (pub const D_string, "D");
        (pub const E_string, "E");
        (pub const F_string, "F");
        (pub const G_string, "G");
        (pub const H_string, "H");
        (pub const I_string, "I");
        (pub const J_string, "J");
        (pub const K_string, "K");
        (pub const L_string, "L");
        (pub const M_string, "M");
        (pub const N_string, "N");
        (pub const O_string, "O");
        (pub const P_string, "P");
        (pub const Q_string, "Q");
        (pub const R_string, "R");
        (pub const S_string, "S");
        (pub const T_string, "T");
        (pub const U_string, "U");
        (pub const V_string, "V");
        (pub const W_string, "W");
        (pub const X_string, "X");
        (pub const Y_string, "Y");
        (pub const Z_string, "Z");
        (pub const open_bracket_string, "[");
        (pub const backslash_string, "\\");
        (pub const close_bracket_string, "]");
        (pub const caret_string, "^");
        (pub const underscore_string, "_");
        (pub const backtick_string, "`");
        (pub const a_string, "a");
        (pub const b_string, "b");
        (pub const c_string, "c");
        (pub const d_string, "d");
        (pub const e_string, "e");
        (pub const f_string, "f");
        (pub const g_string, "g");
        (pub const h_string, "h");
        (pub const i_string, "i");
        (pub const j_string, "j");
        (pub const k_string, "k");
        (pub const l_string, "l");
        (pub const m_string, "m");
        (pub const n_string, "n");
        (pub const o_string, "o");
        (pub const p_string, "p");
        (pub const q_string, "q");
        (pub const r_string, "r");
        (pub const s_string, "s");
        (pub const t_string, "t");
        (pub const u_string, "u");
        (pub const v_string, "v");
        (pub const w_string, "w");
        (pub const x_string, "x");
        (pub const y_string, "y");
        (pub const z_string, "z");
        (pub const open_brace_string, "{");
        (pub const pipe_string, "|");
        (pub const close_brace_string, "}");
        (pub const tilde_string, "~");
        (pub const ascii_del_string, "\x7f");
        (pub const latin1_80_string, "\x80");
        (pub const latin1_81_string, "\x81");
        (pub const latin1_82_string, "\x82");
        (pub const latin1_83_string, "\x83");
        (pub const latin1_84_string, "\x84");
        (pub const latin1_85_string, "\x85");
        (pub const latin1_86_string, "\x86");
        (pub const latin1_87_string, "\x87");
        (pub const latin1_88_string, "\x88");
        (pub const latin1_89_string, "\x89");
        (pub const latin1_8a_string, "\x8a");
        (pub const latin1_8b_string, "\x8b");
        (pub const latin1_8c_string, "\x8c");
        (pub const latin1_8d_string, "\x8d");
        (pub const latin1_8e_string, "\x8e");
        (pub const latin1_8f_string, "\x8f");
        (pub const latin1_90_string, "\x90");
        (pub const latin1_91_string, "\x91");
        (pub const latin1_92_string, "\x92");
        (pub const latin1_93_string, "\x93");
        (pub const latin1_94_string, "\x94");
        (pub const latin1_95_string, "\x95");
        (pub const latin1_96_string, "\x96");
        (pub const latin1_97_string, "\x97");
        (pub const latin1_98_string, "\x98");
        (pub const latin1_99_string, "\x99");
        (pub const latin1_9a_string, "\x9a");
        (pub const latin1_9b_string, "\x9b");
        (pub const latin1_9c_string, "\x9c");
        (pub const latin1_9d_string, "\x9d");
        (pub const latin1_9e_string, "\x9e");
        (pub const latin1_9f_string, "\x9f");
        (pub const latin1_a0_string, "\xa0");
        (pub const latin1_a1_string, "\xa1");
        (pub const latin1_a2_string, "\xa2");
        (pub const latin1_a3_string, "\xa3");
        (pub const latin1_a4_string, "\xa4");
        (pub const latin1_a5_string, "\xa5");
        (pub const latin1_a6_string, "\xa6");
        (pub const latin1_a7_string, "\xa7");
        (pub const latin1_a8_string, "\xa8");
        (pub const latin1_a9_string, "\xa9");
        (pub const latin1_aa_string, "\xaa");
        (pub const latin1_ab_string, "\xab");
        (pub const latin1_ac_string, "\xac");
        (pub const latin1_ad_string, "\xad");
        (pub const latin1_ae_string, "\xae");
        (pub const latin1_af_string, "\xaf");
        (pub const latin1_b0_string, "\xb0");
        (pub const latin1_b1_string, "\xb1");
        (pub const latin1_b2_string, "\xb2");
        (pub const latin1_b3_string, "\xb3");
        (pub const latin1_b4_string, "\xb4");
        (pub const latin1_b5_string, "\xb5");
        (pub const latin1_b6_string, "\xb6");
        (pub const latin1_b7_string, "\xb7");
        (pub const latin1_b8_string, "\xb8");
        (pub const latin1_b9_string, "\xb9");
        (pub const latin1_ba_string, "\xba");
        (pub const latin1_bb_string, "\xbb");
        (pub const latin1_bc_string, "\xbc");
        (pub const latin1_bd_string, "\xbd");
        (pub const latin1_be_string, "\xbe");
        (pub const latin1_bf_string, "\xbf");
        (pub const latin1_c0_string, "\xc0");
        (pub const latin1_c1_string, "\xc1");
        (pub const latin1_c2_string, "\xc2");
        (pub const latin1_c3_string, "\xc3");
        (pub const latin1_c4_string, "\xc4");
        (pub const latin1_c5_string, "\xc5");
        (pub const latin1_c6_string, "\xc6");
        (pub const latin1_c7_string, "\xc7");
        (pub const latin1_c8_string, "\xc8");
        (pub const latin1_c9_string, "\xc9");
        (pub const latin1_ca_string, "\xca");
        (pub const latin1_cb_string, "\xcb");
        (pub const latin1_cc_string, "\xcc");
        (pub const latin1_cd_string, "\xcd");
        (pub const latin1_ce_string, "\xce");
        (pub const latin1_cf_string, "\xcf");
        (pub const latin1_d0_string, "\xd0");
        (pub const latin1_d1_string, "\xd1");
        (pub const latin1_d2_string, "\xd2");
        (pub const latin1_d3_string, "\xd3");
        (pub const latin1_d4_string, "\xd4");
        (pub const latin1_d5_string, "\xd5");
        (pub const latin1_d6_string, "\xd6");
        (pub const latin1_d7_string, "\xd7");
        (pub const latin1_d8_string, "\xd8");
        (pub const latin1_d9_string, "\xd9");
        (pub const latin1_da_string, "\xda");
        (pub const latin1_db_string, "\xdb");
        (pub const latin1_dc_string, "\xdc");
        (pub const latin1_dd_string, "\xdd");
        (pub const latin1_de_string, "\xde");
        (pub const latin1_df_string, "\xdf");
        (pub const latin1_e0_string, "\xe0");
        (pub const latin1_e1_string, "\xe1");
        (pub const latin1_e2_string, "\xe2");
        (pub const latin1_e3_string, "\xe3");
        (pub const latin1_e4_string, "\xe4");
        (pub const latin1_e5_string, "\xe5");
        (pub const latin1_e6_string, "\xe6");
        (pub const latin1_e7_string, "\xe7");
        (pub const latin1_e8_string, "\xe8");
        (pub const latin1_e9_string, "\xe9");
        (pub const latin1_ea_string, "\xea");
        (pub const latin1_eb_string, "\xeb");
        (pub const latin1_ec_string, "\xec");
        (pub const latin1_ed_string, "\xed");
        (pub const latin1_ee_string, "\xee");
        (pub const latin1_ef_string, "\xef");
        (pub const latin1_f0_string, "\xf0");
        (pub const latin1_f1_string, "\xf1");
        (pub const latin1_f2_string, "\xf2");
        (pub const latin1_f3_string, "\xf3");
        (pub const latin1_f4_string, "\xf4");
        (pub const latin1_f5_string, "\xf5");
        (pub const latin1_f6_string, "\xf6");
        (pub const latin1_f7_string, "\xf7");
        (pub const latin1_f8_string, "\xf8");
        (pub const latin1_f9_string, "\xf9");
        (pub const latin1_fa_string, "\xfa");
        (pub const latin1_fb_string, "\xfb");
        (pub const latin1_fc_string, "\xfc");
        (pub const latin1_fd_string, "\xfd");
        (pub const latin1_fe_string, "\xfe");
        (pub const latin1_ff_string, "\xff");
    }
}

define_string_constants! {
    /// Constants for not important internal strings.
    pub mod not_important_string_constants {
        #[cfg(feature = "intl")]
        (pub const adoptText_string, "adoptText");
        #[cfg(feature = "intl")]
        (pub const approximatelySign_string, "approximatelySign");
        #[cfg(feature = "intl")]
        (pub const baseName_string, "baseName");
        #[cfg(feature = "intl")]
        (pub const accounting_string, "accounting");
        #[cfg(feature = "intl")]
        (pub const breakType_string, "breakType");
        #[cfg(feature = "intl")]
        (pub const calendars_string, "calendars");
        #[cfg(feature = "intl")]
        (pub const cardinal_string, "cardinal");
        #[cfg(feature = "intl")]
        (pub const caseFirst_string, "caseFirst");
        #[cfg(feature = "intl")]
        (pub const ceil_string, "ceil");
        #[cfg(feature = "intl")]
        (pub const compare_string, "compare");
        #[cfg(feature = "intl")]
        (pub const collation_string, "collation");
        #[cfg(feature = "intl")]
        (pub const collations_string, "collations");
        #[cfg(feature = "intl")]
        