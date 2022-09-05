pub struct PinyinMap {
    characters_map : std::collections::HashMap<&'static str, char>
}

impl PinyinMap {
    pub fn new() -> PinyinMap {
	PinyinMap{characters_map :
		  std::collections::HashMap::from([
		      ("wǒ"  , '我'),
		      ("shì" , '是'),
		      ("měi" , '美'),
		      ("guó" , '国'),
		      ("rén" , '人'),
		      ("nǐ"  , '你'),
		      ("hǎo" , '好'),
		      ("zài" , '再'),
		      ("jiàn", '见')])
	}
    }



    pub fn get_from_pinyin(&self, pinyin_syllabe : &str) -> Option<&char> {
	self.characters_map.get(pinyin_syllabe)
    }

    fn parse_tone(&self, pinyin_syllabe : &str) -> String {

	let char_transforms = std::collections::HashMap::from([
	    ("+a",'á'),
	    ("*a",'ǎ'),
	    ("'a",'à'),
	    ("-a",'ā'),
	    ("+e",'é'),
	    ("*e",'ě'),
	    ("'e",'è'),
	    ("-e",'ē'),
	    ("+i",'í'),
	    ("*i",'ǐ'),
	    ("'i",'ì'),
	    ("-i",'ī'),
	    ("+o",'ó'),
	    ("*o",'ǒ'),
	    ("'o",'ò'),
	    ("-o",'ō'),
	    ("+u",'ú'),
	    ("*u",'ǔ'),
	    ("'u",'ù'),
	    ("-u",'ū')
	]);
	
	let tone_char = pinyin_syllabe.chars().position(|c| ['*', '\'', '+', '-'].contains(&c));

	if tone_char != None {
            let vowel = match tone_char {
		Some(pos) => pinyin_syllabe.chars().nth(pos+1),
		_ => None
	    };
	    
	    let mut chars = pinyin_syllabe.chars();

	    let mut parsed_syllabe = String::new();

	    for _i in 0..tone_char.unwrap() {
		parsed_syllabe.push(chars.next().unwrap());
	    };

	    let tone = chars.next();

	    let mut toned = String::new();
	    toned.push(tone.unwrap()); toned.push(vowel.unwrap());

	    let toned_char = char_transforms.get(&toned[..]).unwrap();

	    parsed_syllabe.push(*toned_char);

	    chars.next();
	    for c in chars {
		parsed_syllabe.push(c);
	    };
	    
	    parsed_syllabe
	} else {
	    pinyin_syllabe.to_string()

	}
	
    } 
}

#[test]
fn test_tone() {
    let map = PinyinMap::new();

    let tone = map.parse_tone("w*o");
    assert_eq!("wǒ", tone);
    let tone = map.parse_tone("s-i");
    assert_eq!("sī", tone);
}

#[test]
fn test_map() {

    let map = PinyinMap::new();
    
    assert_eq!('我', *map.get_from_pinyin(&"wǒ").unwrap());
    assert_eq!('你', *map.get_from_pinyin(&"nǐ").unwrap());
    assert_eq!(None, map.get_from_pinyin(&"non_chinese"));
}



//w'o w+o w-o w*o 
