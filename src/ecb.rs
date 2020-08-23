
// The number of 32 bit words in a key.
const NK: usize = 4;

// The number of rounds in AES Cipher. AES-128 uses 10.
const NR: usize = 10;

// The number of columns in an AES state.
pub const NB: usize = 4;

// The size of the key after expansion, in bytes
pub const AES128_EXP_LEN: usize = 176;

// The size of the AES-128 key, in bytes
pub const AES128_LEN: usize = 16;

// This sets up an NBxNB array typedef for the state array for the cipher alogrithm
#[allow(non_camel_case_types)]
pub type state_t = [[u8; NB]; NB];

// This is a typedef for a key
#[allow(non_camel_case_types)]
pub type key_t = [u8; AES128_LEN];

// This is a typedef for an expanded key
#[allow(non_camel_case_types)]
pub type roundkey_t = [u8; AES128_EXP_LEN];

// This is a struct for the AES context. This could be expanded later to support other cipher modes,
//  but for ECB this is all that is needed.
#[allow(non_camel_case_types)]
pub struct aes_ctx {
    pub roundkey: roundkey_t,
}

const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab, 0x76,
    0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4, 0x72, 0xc0,
    0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71, 0xd8, 0x31, 0x15,
    0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2, 0xeb, 0x27, 0xb2, 0x75,
    0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6, 0xb3, 0x29, 0xe3, 0x2f, 0x84,
    0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb, 0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf,
    0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45, 0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8,
    0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5, 0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2,
    0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44, 0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73,
    0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a, 0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb,
    0xe0, 0x32, 0x3a, 0x0a, 0x49, 0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79,
    0xe7, 0xc8, 0x37, 0x6d, 0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08,
    0xba, 0x78, 0x25, 0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a,
    0x70, 0x3e, 0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e,
    0xe1, 0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb, 0x16,
];

const RSBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7, 0xfb,
    0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde, 0xe9, 0xcb,
    0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42, 0xfa, 0xc3, 0x4e,
    0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49, 0x6d, 0x8b, 0xd1, 0x25,
    0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c, 0xcc, 0x5d, 0x65, 0xb6, 0x92,
    0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15, 0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84,
    0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7, 0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06,
    0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02, 0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b,
    0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc, 0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73,
    0x96, 0xac, 0x74, 0x22, 0xe7, 0xad, 0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e,
    0x47, 0xf1, 0x1a, 0x71, 0x1d, 0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b,
    0xfc, 0x56, 0x3e, 0x4b, 0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4,
    0x1f, 0xdd, 0xa8, 0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f,
    0x60, 0x51, 0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef,
    0xa0, 0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c, 0x7d,
];

const RCON: [u8; 11] = [
    0x8d, 0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36,
];

fn get_sbox_value(a: &u8) -> u8 {
    SBOX[*a as usize]
}

fn get_sbox_invert(a: &u8) -> u8 {
    RSBOX[*a as usize]
}

pub fn key_expansion(a: &key_t) -> roundkey_t {
    let mut round: roundkey_t = [0; AES128_EXP_LEN];
    let mut temp: [u8; NB] = [0; NB];

    // first round
    for i in 0..NK {
        round[(i * NB) + 0] = a[(i * NB) + 0];
        round[(i * NB) + 1] = a[(i * NB) + 1];
        round[(i * NB) + 2] = a[(i * NB) + 2];
        round[(i * NB) + 3] = a[(i * NB) + 3];
    }

    // all other rounds
    for i in NK..(NB * (NR + 1)) {
        let k = (i - 1) * NB;
        temp[0] = round[k + 0];
        temp[1] = round[k + 1];
        temp[2] = round[k + 2];
        temp[3] = round[k + 3];

        if (i % NK) == 0 {
            // rotates left 1 byte
            let t = temp[0];
            temp[0] = temp[1];
            temp[1] = temp[2];
            temp[2] = temp[3];
            temp[3] = t;

            // applies sbox to these inputs
            temp[0] = get_sbox_value(&temp[0]);
            temp[1] = get_sbox_value(&temp[1]);
            temp[2] = get_sbox_value(&temp[2]);
            temp[3] = get_sbox_value(&temp[3]);

            temp[0] ^= RCON[i / NK];
        }

        let j = i * NB;
        let k = (i - NK) * NB;

        round[j + 0] = round[k + 0] ^ temp[0];
        round[j + 1] = round[k + 1] ^ temp[1];
        round[j + 2] = round[k + 2] ^ temp[2];
        round[j + 3] = round[k + 3] ^ temp[3];
    }

    round
}

pub fn aes_init_ctx(a: key_t) -> aes_ctx {
    let ctx: aes_ctx = aes_ctx {
        roundkey: key_expansion(&a),
    };

    ctx
}

fn add_round_key(round: u8, state: &state_t, roundkey: &roundkey_t) -> state_t {
    let mut ns: state_t = [[0; NB]; NB];

    for i in 0..NB {
        for j in 0..NB {
            ns[i][j] =
                state[i][j] ^ roundkey[(round as usize * NB * 4) + (i as usize * NB) + j as usize];
        }
    }

    ns
}

fn sub_bytes(state: &state_t) -> state_t {
    let mut ns: state_t = [[0; NB]; NB];

    for i in 0..NB {
        for j in 0..NB {
            ns[j][i] = get_sbox_value(&state[j][i]);
        }
    }

    ns
}

fn inv_sub_bytes(state: &state_t) -> state_t {
    let mut ns: state_t = [[0; NB]; NB];

    for i in 0..NB {
        for j in 0..NB {
            ns[j][i] = get_sbox_invert(&state[j][i]);
        }
    }

    ns
}

pub fn shift_rows(state: &state_t) -> state_t {
    let mut ns: state_t = [[0; NB]; NB];

    for i in 0..NB {
        for j in 0..NB {
            ns[(j + NB - i) % NB][i] = state[j][i];
        }
    }

    ns
}

fn inv_shift_rows(state: &state_t) -> state_t {
    let mut ns: state_t = [[0; NB]; NB];

    for i in 0..NB {
        for j in 0..NB {
            ns[(j + i) % NB][i] = state[j][i];
        }
    }

    ns
}

fn xtime(a: u8) -> u8 {
    (a << 1) ^ (((a >> 7) & 1) * 0x1b)
}

fn mult(a: &u8, b: u8) -> u8 {
    ((b & 1) * a)
        ^ ((b >> 1 & 1) * xtime(*a))
        ^ ((b >> 2 & 1) * xtime(xtime(*a)))
        ^ ((b >> 3 & 1) * xtime(xtime(xtime(*a))))
        ^ ((b >> 4 & 1) * xtime(xtime(xtime(xtime(*a)))))
}

fn mix_columns(state: &state_t) -> state_t {
    let mut ns: state_t = [[0; NB]; NB];
    #[allow(unused_assignments)]
    let mut t0 = 0;
    #[allow(unused_assignments)]
    let mut t1 = 0;

    for i in 0..NB {
        t0 = state[i][0] ^ state[i][1] ^ state[i][2] ^ state[i][3];

        for j in 0..NB {
            t1 = state[i][j] ^ state[i][(j + 1) % NB];
            t1 = xtime(t1);
            ns[i][j] = state[i][j] ^ t1 ^ t0;
        }
    }

    ns
}

fn inv_mix_columns(state: &state_t) -> state_t {
    let mut ns: state_t = [[0; NB]; NB];

    for i in 0..NB {
        let a = state[i][0];
        let b = state[i][1];
        let c = state[i][2];
        let d = state[i][3];

        ns[i][0] = mult(&a, 0x0e) ^ mult(&b, 0x0b) ^ mult(&c, 0x0d) ^ mult(&d, 0x09);
        ns[i][1] = mult(&a, 0x09) ^ mult(&b, 0x0e) ^ mult(&c, 0x0b) ^ mult(&d, 0x0d);
        ns[i][2] = mult(&a, 0x0d) ^ mult(&b, 0x09) ^ mult(&c, 0x0e) ^ mult(&d, 0x0b);
        ns[i][3] = mult(&a, 0x0b) ^ mult(&b, 0x0d) ^ mult(&c, 0x09) ^ mult(&d, 0x0e);
    }

    ns
}

fn cipher(state: &state_t, roundkey: &roundkey_t) -> state_t {
    let mut round: u8 = 0;

    let mut ns = add_round_key(round, state, roundkey);
    round += 1;

    loop {
        ns = sub_bytes(&ns);
        ns = shift_rows(&ns);

        if round as usize == NR {
            break;
        }

        ns = mix_columns(&ns);
        ns = add_round_key(round, &ns, roundkey);

        round += 1;
    }

    ns = add_round_key(round, &ns, roundkey);

    ns
}

fn inv_cipher(state: &state_t, roundkey: &roundkey_t) -> state_t {
    let mut round: u8 = 10;

    let mut ns = add_round_key(round, state, roundkey);
    round -= 1;

    loop {
        ns = inv_shift_rows(&ns);
        ns = inv_sub_bytes(&ns);
        ns = add_round_key(round, &ns, roundkey);

        if round == 0 {
            break;
        }

        ns = inv_mix_columns(&ns);

        round -= 1;
    }

    ns
}

fn array_to_state(data: &[u8; 16]) -> state_t {
    let mut ns: state_t = [[0; NB]; NB];

    for i in 0..16 {
        ns[i / NB][i % NB] = data[i];
    }

    ns
}

fn state_to_array(state: &state_t) -> [u8; 16] {
    let mut out: [u8; 16] = [0; 16];

    for i in 0..16 {
        out[i] = state[i / NB][i % NB];
    }

    out
}

pub fn aes_ecb_encrypt(ctx: &aes_ctx, data: &[u8; 16]) -> [u8; 16] {
    let mut ns: state_t = array_to_state(&data);

    ns = cipher(&ns, &ctx.roundkey);

    state_to_array(&ns)
}

pub fn aes_ecb_decrypt(ctx: &aes_ctx, data: &[u8; 16]) -> [u8; 16] {
    let mut ns: state_t = array_to_state(&data);

    ns = inv_cipher(&ns, &ctx.roundkey);

    state_to_array(&ns)
}
