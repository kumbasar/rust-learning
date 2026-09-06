fn print_country(country: String) {
    println!("{}", country);
}

fn get_length(word :&String) {
    println!("{}", word.split_whitespace().count());
}

fn main() {
    let country = String::from("FR");
    print_country(country.clone());
    print_country(country);


    let mut my_string = String::new();

    for _ in 0..50 {
        my_string.push_str(" more words ");
        get_length(&my_string);
    }
}

/*
FR
FR
2
4
6
8
10
12
14
16
18
20
22
24
26
28
30
32
34
36
38
40
42
44
46
48
50
52
54
56
58
60
62
64
66
68
70
72
74
76
78
80
82
84
86
88
90
92
94
96
98
100


 */
