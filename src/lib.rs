#![no_std]
/// UGLY-MAXIMAL (the name) font.

#[rustfmt::skip]
pub const LETTERS: [&[u8]; 26] = 
[ 
 /*A*/   & [0x1e, 0x9, 0x9, 0x9, 0x1e], 
 /*B*/   & [0x1f, 0x15, 0x15, 0x15, 0xa],
 /*C*/   & [0xe, 0x11, 0x11, 0x11, 0x11],
 /*D*/   & [0x1f, 0x11, 0x11, 0x11, 0xe],
 /*E*/   & [0x1f, 0x15, 0x15, 0x15, 0x11],
 /*F*/   & [0x1f, 0x5, 0x5, 0x5, 0x1],
 /*G*/   & [0xe, 0x11, 0x15, 0x15, 0x9],
 /*H*/   & [0x1f, 0x4, 0x4, 0x4, 0x1f],
 /*I*/   & [0x1f],
 /*J*/   & [0xc, 0x10, 0x10, 0x10, 0xf],
 /*K*/   & [0x1f, 0x4, 0x4, 0xa, 0x11],
 /*L*/   & [0x1f, 0x10, 0x10, 0x10, 0x8],
 /*M*/   & [0x1f, 0x1, 0x6, 0x1, 0x1f],
 /*N*/   & [0x1f, 0x1, 0xe, 0x10, 0x1f],
 /*O*/   & [0xe, 0x11, 0x11, 0x11, 0xe],
 /*P*/   & [0x1f, 0x5, 0x5, 0x5, 0x2],
 /*Q*/   & [0xe, 0x11, 0x15, 0x9, 0x16],
 /*R*/   & [0x1f, 0x5, 0x5, 0xd, 0x16],
 /*S*/   & [0x16, 0x15, 0x15, 0x15, 0xd],
 /*T*/   & [0x1, 0x1, 0x1f, 0x1, 0x1],
 /*U*/   & [0xf, 0x10, 0x10, 0x10, 0xf],
 /*V*/   & [0x7, 0x8, 0x10, 0x8, 0x7],
 /*W*/   & [0xf, 0x10, 0x1f, 0x10, 0xf],
 /*X*/   & [0x11, 0xa, 0x4, 0xa, 0x11],
 /*Y*/   & [0x1, 0x2, 0x1c, 0x2, 0x1],
 /*Z*/   & [0x11, 0x19, 0x15, 0x13, 0x11],
];

#[rustfmt::skip]
pub const DIGITS: [[u8; 4]; 10] = 
[
/*0*/   [0x1f, 0x11, 0x11, 0x1f],
/*1*/   [0x4, 0x2, 0x1, 0x1f],
/*2*/   [0x1d, 0x15, 0x15, 0x17],
/*3*/   [0x15, 0x15, 0x15, 0x1f],
/*4*/   [0x7, 0x4, 0x4, 0x1f],
/*5*/   [0x17, 0x15, 0x15, 0x1d],
/*6*/   [0x1f, 0x15, 0x15, 0x1d],
/*7*/   [0x3, 0x1, 0x1, 0x1f],
/*8*/   [0x1f, 0x15, 0x15, 0x1f],
/*9*/   [0x7, 0x5, 0x5, 0x1f],
];

#[rustfmt::skip]
pub const SYMBOLS: [&[u8]; 4] = 
[
  /* ! */ & [0x17], 
  /* . */ & [0x10],
  /*   */ & [0x0],
  /* - */ & [0x4, 0x4, 0x4],  
];

pub const UNSUPPORTED: [u8; 5] = [0x11, 0x13, 0x15, 0x19, 0x11];
pub const SPACING: [u8; 1] = [0u8; 1];

pub fn col_defs(text: &str, out: &mut [&[u8]]) {
    let mut ix = 0;
    for c in text.chars() {
        out[ix] = col_def(c);
        ix += 1;
        out[ix] = &SPACING;
        ix += 1;
    }

    for _ in 0..4 {
        out[ix] = &SPACING;
        ix += 1;
    }
}

pub fn col_def(mut c: char) -> &'static [u8] {
    if c.is_ascii() {
        if c.is_lowercase() {
            c = c.to_ascii_uppercase()
        }
    }

    let code = c as usize;
    if code > 64 && code < 91 {
        return &LETTERS[code - 65];
    }

    if code > 47 && code < 58 {
        return &DIGITS[code - 48];
    }

    match c {
        '!' => SYMBOLS[0],
        '.' => SYMBOLS[1],
        ' ' => SYMBOLS[2],
        '-' => SYMBOLS[3],
        _ => &UNSUPPORTED,
    }
}