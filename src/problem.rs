
extern crate time;
use problems;

pub fn problem(n: i64) -> (i64, String, f64) {
		let s : String;
		let start = time::now();
        let mut found = false;
		match n {
65 => s = {found = true; problems::p65::solve().to_string()},
64 => s = {found = true; problems::p64::solve().to_string()},
62 => s = {found = true; problems::p62::solve().to_string()},
61 => s = {found = true; problems::p61::solve().to_string()},
54 => s = {found = true; problems::p54::solve().to_string()},
56 => s = {found = true; problems::p56::solve().to_string()},
55 => s = {found = true; problems::p55::solve().to_string()},
53 => s = {found = true; problems::p53::solve().to_string()},
52 => s = {found = true; problems::p52::solve().to_string()},
			1 => s = {found = true; problems::p1::solve().to_string()},
			2 => s = {found = true; problems::p2::solve().to_string()},
			3 => s = {found = true; problems::p3::solve().to_string()},
			4 => s = {found = true; problems::p4::solve().to_string()},
			5 => s = {found = true; problems::p5::solve().to_string()},
			6 => s = {found = true; problems::p6::solve().to_string()},
			7 => s = {found = true; problems::p7::solve().to_string()},
			8 => s = {found = true; problems::p8::solve().to_string()},
			9 => s = {found = true; problems::p9::solve().to_string()},
			10 => s = {found = true; problems::p10::solve().to_string()},
			11 => s = {found = true; problems::p11::solve().to_string()},
			12 => s = {found = true; problems::p12::solve().to_string()},
			13 => s = {found = true; problems::p13::solve().to_string()},
			14 => s = {found = true; problems::p14::solve().to_string()},
			15 => s = {found = true; problems::p15::solve().to_string()},
			16 => s = {found = true; problems::p16::solve().to_string()},
			17 => s = {found = true; problems::p17::solve().to_string()},
			18 => s = {found = true; problems::p18::solve().to_string()},
			19 => s = {found = true; problems::p19::solve().to_string()},
			20 => s = {found = true; problems::p20::solve().to_string()},
			21 => s = {found = true; problems::p21::solve().to_string()},
			22 => s = {found = true; problems::p22::solve().to_string()},
			23 => s = {found = true; problems::p23::solve().to_string()},
			24 => s = {found = true; problems::p24::solve().to_string()},
			25 => s = {found = true; problems::p25::solve().to_string()},

			26 => s = {found = true; problems::p26::solve().to_string()},
			27 => s = {found = true; problems::p27::solve().to_string()},
			28 => s = {found = true; problems::p28::solve().to_string()},
			29 => s = {found = true; problems::p29::solve().to_string()},
            30 => s = {found = true; problems::p30::solve().to_string()},
            31 => s = {found = true; problems::p31::solve().to_string()},
            32 => s = {found = true; problems::p32::solve().to_string()},
            33 => s = {found = true; problems::p33::solve().to_string()},
            34 => s = {found = true; problems::p34::solve().to_string()},
            35 => s = {found = true; problems::p35::solve().to_string()},
			36 => s = {found = true; problems::p36::solve().to_string()},
			37 => s = {found = true; problems::p37::solve().to_string()},
			38 => s = {found = true; problems::p38::solve().to_string()},
			39 => s = {found = true; problems::p39::solve().to_string()},
			40 => s = {found = true; problems::p40::solve().to_string()},
			41 => s = {found = true; problems::p41::solve().to_string()},
			42 => s = {found = true; problems::p42::solve().to_string()},
			43 => s = {found = true; problems::p43::solve().to_string()},
			44 => s = {found = true; problems::p44::solve().to_string()},
			45 => s = {found = true; problems::p45::solve().to_string()},
			46 => s = {found = true; problems::p46::solve().to_string()},
			47 => s = {found = true; problems::p47::solve().to_string()},
			48 => s = {found = true; problems::p48::solve().to_string()},
			49 => s = {found = true; problems::p49::solve().to_string()},
			50 => s = {found = true; problems::p50::solve().to_string()},

			51 => s = {found = true; problems::p51::solve().to_string()},
			57 => s = {found = true; problems::p57::solve().to_string()},
			58 => s = {found = true; problems::p58::solve().to_string()},
			59 => s = {found = true; problems::p59::solve().to_string()},
			60 => s = {found = true; problems::p60::solve().to_string()},
			63 => s = {found = true; problems::p63::solve().to_string()},
			67 => s = {found = true; problems::p67::solve().to_string()},
			71 => s = {found = true; problems::p71::solve().to_string()},

			79 => s = {found = true; problems::p79::solve().to_string()},
			92 => s = {found = true; problems::p92::solve().to_string()},
			97 => s = {found = true; problems::p97::solve().to_string()},
			206 => s = {found = true; problems::p206::solve().to_string()},
			615 => s = {found = true; problems::p615::solve().to_string()},
			684 => s = {found = true; problems::p684::solve().to_string()},
			700 => s = {found = true; problems::p700::solve().to_string()},
			739 => s = {found = true; problems::p739::solve().to_string()},
			_ => s = format!("Problem {} has not been solved.", n)
		}
		let end = time::now();
		let duration = (end - start).num_nanoseconds().unwrap() as f64 /1000000000.0;
        if found {
            
        } else {
            println!("{}",s);
		}
		return (n,s,duration);
	}
