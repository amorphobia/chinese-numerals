#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum NumChar {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Shi,
    Bai,
    Qian,
    Wan,
    Yi,
    Zhao,
    Jing,
    Gai,
    Zi,
    Rang,
    Gou,
    Jian,
    Zheng,
    Zai,
    Neg,
}

use NumChar::*;

pub(crate) const NUM_CHARS: [NumChar; 25] = [Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Shi, Bai, Qian, Wan, Yi, Zhao, Jing, Gai, Zi, Rang, Gou, Jian, Zheng, Zai, Neg];

impl NumChar {
    pub fn to_lowercase_simp(self) -> char {
        match self {
            Zero => '零',
            One => '一',
            Two => '二',
            Three => '三',
            Four => '四',
            Five => '五',
            Six => '六',
            Seven => '七',
            Eight => '八',
            Nine => '九',
            Shi => '十',
            Bai => '百',
            Qian => '千',
            Wan => '万',
            Yi => '亿',
            Zhao => '兆',
            Jing => '京',
            Gai => '垓',
            Zi => '秭',
            Rang => '穰',
            Gou => '沟',
            Jian => '涧',
            Zheng => '正',
            Zai => '载',
            Neg => '负',
        }
    }

    pub fn to_uppercase_simp(self) -> char {
        match self {
            One => '壹',
            Two => '贰',
            Three => '叁',
            Four => '肆',
            Five => '伍',
            Six => '陆',
            Seven => '柒',
            Eight => '捌',
            Nine => '玖',
            Shi => '拾',
            Bai => '佰',
            Qian => '仟',
            Wan => '万',
            _ => self.to_lowercase_simp(),
        }
    }

    pub fn to_lowercase_trad(self) -> char {
        match self {
            Wan => '萬',
            Yi => '億',
            Zhao => '兆',
            Gou => '溝',
            Jian => '澗',
            Zai => '載',
            Neg => '負',
            _ => self.to_lowercase_simp(),
        }
    }

    pub fn to_uppercase_trad(self) -> char {
        match self {
            Two => '貳',
            Three => '叄',
            Six => '陸',
            _ => self.to_lowercase_trad(),
        }
    }
}

