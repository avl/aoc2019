use std::convert::TryFrom;

const module_masses_str : &str = "
129192
58561
57267
95382
84995
127372
93598
97264
138550
79327
135661
139468
108860
149642
72123
128333
69002
98450
86267
70171
101333
79822
142539
142743
51371
111381
62073
72210
125168
135952
131060
121842
88234
146774
136571
126719
50644
75696
51195
77171
118052
83691
133779
149814
64847
110697
92695
59453
139517
129487
79271
97896
146987
149822
71866
90797
104732
54997
50139
134115
133017
144979
89428
124750
91833
57252
67195
121624
102706
138245
127700
124098
110382
121557
103613
133576
122801
112306
120203
134696
76129
84576
80854
147237
71025
127513
143631
125090
115698
57979
84880
120177
147389
88380
114688
56355
126265
58220
63523
130179
    ";

fn fuel_for_module(mass:i32) -> i32 {
    ((mass/3)-2).max(0)
}

fn fuel_for_module_and_fuel(mass:i32) -> i32 {
    if mass == 0 {
        return 0;
    }
    let fuel = fuel_for_module(mass);
    fuel + fuel_for_module_and_fuel(fuel)
}

fn main() {

    let mut tot_mass_part1 = 0;
    let mut tot_mass_part2 = 0;
    for module_mass in module_masses_str.split_ascii_whitespace().map(|x|x.parse().unwrap()) {
        tot_mass_part1 += fuel_for_module(module_mass);
        tot_mass_part2 += fuel_for_module_and_fuel(module_mass);
    }
    println!("Answer part #1: {}",tot_mass_part1);
    println!("Answer part #2: {}",tot_mass_part2);

}


#[test]
pub fn precondition1() {
    assert_eq!(fuel_for_module(12),2);
    assert_eq!(fuel_for_module(14),2);
    assert_eq!(fuel_for_module(1969),654);
    assert_eq!(fuel_for_module(100756),33583);
}

#[test]
pub fn precondition2() {
    assert_eq!(fuel_for_module_and_fuel(14),2);
    assert_eq!(fuel_for_module_and_fuel(1969),966);
}
