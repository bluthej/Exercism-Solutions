const SEPARATORS: [&str; 7] = [
    "",
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

pub fn encode(n: u64) -> String {
    let chunks = create_chunks(n);
    let sep = &SEPARATORS[0..chunks.len()];

    chunks
        .into_iter()
        .map(encode_0_to_999)
        .zip(sep.iter().rev())
        .filter_map(|(i, s)| {
            if n != 0 && i == "zero" {
                None
            } else if s.is_empty() {
                Some(format!("{}", i))
            } else {
                Some(format!("{} {}", i, s))
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn create_chunks(n: u64) -> Vec<u64> {
    let mut res = Vec::new();
    if n == 0 {
        res.push(0);
    }
    let mut m = n;
    while m > 0 {
        res.push(m % 1_000);
        m = m / 1_000;
    }
    res.into_iter().rev().collect()
}

fn encode_0_to_999(n: u64) -> String {
    if n < 100 {
        return encode_0_to_99(n);
    }
    let prefix = encode_0_to_19(n / 100);
    let last_two_digits = n % 100;
    let suffix = if last_two_digits > 0 {
        format!(" {}", encode_0_to_99(n % 100))
    } else {
        String::from("")
    };
    format!("{} hundred{}", prefix, suffix)
}

fn encode_0_to_99(n: u64) -> String {
    if n < 20 {
        return encode_0_to_19(n);
    }

    let prefix = match n / 10 {
        2 => String::from("twenty"),
        3 => String::from("thirty"),
        4 => String::from("forty"),
        5 => String::from("fifty"),
        6 => String::from("sixty"),
        7 => String::from("seventy"),
        8 => String::from("eighty"),
        9 => String::from("ninety"),
        _ => unreachable!(),
    };
    let unit = n % 10;
    let suffix = if unit > 0 {
        format!("-{}", encode_0_to_19(n % 10))
    } else {
        String::from("")
    };
    format!("{}{}", prefix, suffix)
}

fn encode_0_to_19(n: u64) -> String {
    return match n {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        _ => unreachable!(),
    };
}
