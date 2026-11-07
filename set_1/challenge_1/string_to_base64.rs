use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let arg_string = &args[1];
    let mut bin_result = String::from("");
    let mut encoded_base64 = String::from("");

    let base64_table: HashMap<isize, char> = HashMap::from([
        (0, 'A'),  (1, 'B'),  (2, 'C'),  (3, 'D'),
        (4, 'E'),  (5, 'F'),  (6, 'G'),  (7, 'H'),
        (8, 'I'),  (9, 'J'),  (10, 'K'), (11, 'L'),
        (12, 'M'), (13, 'N'), (14, 'O'), (15, 'P'),
        (16, 'Q'), (17, 'R'), (18, 'S'), (19, 'T'),
        (20, 'U'), (21, 'V'), (22, 'W'), (23, 'X'),
        (24, 'Y'), (25, 'Z'),

        (26, 'a'), (27, 'b'), (28, 'c'), (29, 'd'),
        (30, 'e'), (31, 'f'), (32, 'g'), (33, 'h'),
        (34, 'i'), (35, 'j'), (36, 'k'), (37, 'l'),
        (38, 'm'), (39, 'n'), (40, 'o'), (41, 'p'),
        (42, 'q'), (43, 'r'), (44, 's'), (45, 't'),
        (46, 'u'), (47, 'v'), (48, 'w'), (49, 'x'),
        (50, 'y'), (51, 'z'),

        (52, '0'), (53, '1'), (54, '2'), (55, '3'),
        (56, '4'), (57, '5'), (58, '6'), (59, '7'),
        (60, '8'), (61, '9'),

        (62, '+'), (63, '/'),
    ]);

    println!("\nConverting..: {arg_string}");

    //let arg_bytes = arg_string.as_bytes();

    let hex_to_bytes : Vec<u8> = (0..arg_string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&arg_string[i..i + 2], 16).unwrap())
        .collect();

    println!("\nHexadecimal to bytes..: {:?}\n", hex_to_bytes);

    //let test = u8::from_str_radix(arg_string, 2);

    //println!("Teste: {test}");
    
    for char in hex_to_bytes {
        bin_result += &format!("{char:0>8b}");
    }

    for chunk in bin_result.as_bytes().chunks(6) {
        let mut piece = std::str::from_utf8(chunk).unwrap().to_string();
        if piece.len() < 6 {
            let pad = 6 - piece.len();
            //println!("Pad..: {}", pad);
            for _ in 0..pad {
                piece.push('0'); 
            } 
        }

        let decimal_piece =  isize::from_str_radix(&piece, 2).unwrap();
        //println!("Piece..: {} | Decimal Piece..: {}", piece, decimal_piece);
        encoded_base64.push(base64_table.get(&decimal_piece).copied().expect("REASON"));
    }
 
    let mod_base64 = encoded_base64.len() % 4;

    if mod_base64 > 0 {
        let final_pad = 4 - mod_base64;
        for _ in 0..final_pad{
            encoded_base64.push('=');
        }
    }


    println!("Bytes to binary..: {bin_result}\n");

    println!("Binary to Base64..: {}", encoded_base64);
    
}
