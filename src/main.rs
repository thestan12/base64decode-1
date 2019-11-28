use std::str;
 
const UPPERCASE_OFFSET: i8 = -65;
const LOWERCASE_OFFSET: i8 = 26 - 97;
const NUM_OFFSET: i8 = 52 - 48;

fn base64decode(input: String) -> String {
    let result = input.trim().chars()
        .filter(|&ch| ch != '=')                                //Filter '=' chars
        .map(|ch| {                                             //Map char values using Base64 Characters Table
            let ascii = ch as i8;                           
            let convert = match ch {
                '0' ..= '9' => ascii + NUM_OFFSET,
                'a' ..= 'z' => ascii + LOWERCASE_OFFSET,
                'A' ..= 'Z' => ascii + UPPERCASE_OFFSET,
                '+' => 62,
                '/' => 63,
                _ => 3,
            };
            format!("{:#08b}", convert)[2..].to_string()        //convert indices to binary format and remove the two first digits
        })
        .collect::<String>()                                    //concatenate the resulting binary values
        .chars()
        .collect::<Vec<char>>()
        .chunks(8)                                              //split into 8 character chunks
        .map(|chunk| {
            let num_str = chunk.iter().collect::<String>();
            usize::from_str_radix(&num_str, 2).unwrap() as u8   //convert the binary string into its u8 value
        })
        .collect::<Vec<_>>();
 
    let result = str::from_utf8(&result).unwrap();              //convert into UTF-8 string
    return String::from(result);
}
 
fn main() {
    // let mut input = String::new();
    let mut input = String::from("TGEgcmFjbGV0dGUgKEJyYXRjaMOkcywgwqsgZnJvbWFnZSDCuyByw7R0aSwgZW4gc3Vpc3NlIGFsbGVtYW5kKSBlc3QgZCd1bmUgcGFydCB1biBmcm9tYWdlIChsZSBvdSBsYSByYWNsZXR0ZSkgb3JpZ2luYWlyZSBkdSBjYW50b24gZHUgVmFsYWlzIGVuIFN1aXNzZSwgZXQgZCdhdXRyZSBwYXJ0LCB1bmUgcmVjZXR0ZSBkZSBjdWlzaW5lIHRyYWRpdGlvbm5lbGxlIGV0IGVtYmzDqW1hdGlxdWUgZGUgbGEgY3Vpc2luZSBzdWlzc2UsIGNvbm51ZSBkYW5zIGxlIG1vbmRlIGVudGllciwgdmFyaWFudGUgZGVzIGZvbmR1ZXMgYXUgZnJvbWFnZSwgw6AgYmFzZSBkZSBjZSBmcm9tYWdlIGZvbmR1LCByYWNsw6kgYXUgZnVyIGV0IMOgIG1lc3VyZSBxdeKAmWlsIGZvbmQsIGV0IHNlcnZpZSB0cmFkaXRpb25uZWxsZW1lbnQgYXZlYyBkZXMgcG9tbWVzIGRlIHRlcnJlIGVuIHJvYmUgZGVzIGNoYW1wcyBldCBhY2NvbXBhZ27DqWUgZGUgbMOpZ3VtZXMgYXUgdmluYWlncmUgKGNvcm5pY2hvbnMsIG9pZ25vbnMpLg==");

    // String::

    std::io::stdin().read_line(&mut input).unwrap();
    println!("Input: {}", input);
 
    let output = base64decode(input);
    println!("Output: {}", output);
}
