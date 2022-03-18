use crate::{characters::*, sealed::ChineseNumeralBase, sealed::SignedInteger, MidScaleInt, Sign};

/// Long scale integers (上数).
///
/// 「上数者，数穷则变。若言万万曰亿，亿亿曰兆、兆兆曰京也。」
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
pub struct LongScaleInt {
    pub(super) sign: Sign,
    pub(super) data: u128,
}

impl LongScaleInt {
    /// Generates a new long scale integer from given sign and absolute value.
    ///
    /// The range of primitive `u128` is smaller than long scale can reach. This crate provides [`LongScaleBigInt`] for integers with absolute value larger than [`u128::MAX`].
    pub fn new(sign: Sign, data: u128) -> Self {
        if data == 0 {
            Self::default()
        } else {
            Self { sign, data }
        }
    }
}

impl ChineseNumeralBase for LongScaleInt {
    fn to_chars(&self) -> Vec<NumChar> {
        let mut chars = Vec::new();
        let mut num = *self.data();
        let mut prev_rem = 1000_0000_0000_0000;

        // u128 uses up to NUM_CHARS[16] = Jing (京) for long scale numerals
        for exp in 14..=16 {
            let rem = num % 1_0000_0000_0000_0000;
            num /= 1_0000_0000_0000_0000;

            if rem > 0 {
                if !chars.is_empty() && prev_rem < 1000_0000_0000_0000 {
                    chars.push(NUM_CHARS[0]);
                }
                if exp > 14 {
                    chars.push(NUM_CHARS[exp]);
                }
                let mid = MidScaleInt::from(rem);
                let mut node = mid.to_chars();
                chars.append(&mut node);
            }
            prev_rem = rem;
        }
        chars
    }

    fn to_chars_trimmed(&self) -> Vec<NumChar> {
        let mut chars = self.to_chars();
        let mut data = *self.data();
        while data >= 1_0000 {
            data /= 1_0000;
        }
        if data >= 10 && data <= 19 {
            let one = chars.pop();
            debug_assert_eq!(one, Some(NumChar::One));
        }
        chars
    }
}

#[cfg(feature = "bigint")]
use num_bigint::BigUint;
#[cfg(feature = "bigint")]
use num_integer::Integer;
#[cfg(feature = "bigint")]
use num_traits::{ToPrimitive, Zero};

/// Long scale big integers (上数).
///
/// Use it by turning on feature "bigint". It uses [`BigUint`](num_bigint::BigUint) to store the absolute value. Therefore, all integers that can be expressed in long scale are included.
#[cfg(feature = "bigint")]
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug, Default)]
pub struct LongScaleBigInt {
    pub(super) sign: Sign,
    pub(super) data: BigUint,
}

#[cfg(feature = "bigint")]
impl LongScaleBigInt {
    pub(super) const MAX_ABS_ARR: &'static [u32] = &[
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295, 4294967295,
        4294967295, 4294967295, 4294967295, 4294967295, 1691320320, 2671006246, 1682531301,
        2072858707, 1240508969, 3108358191, 1125119096, 2470144952, 1610099978, 1690632660,
        1941696884, 2663506355, 1006364675, 3909158537, 4147711374, 1072663936, 4078768933,
        745751659, 4123687570, 471458681, 655028926, 4113407388, 3945524552, 985625313, 1254424514,
        2127508744, 570530434, 945388122, 3194649404, 2589065070, 2731705399, 202030749,
        2090780394, 3348662271, 1481754777, 1130635472, 4025144705, 1924486271, 2578567861,
        125491448, 1558036315, 994248173, 3817216711, 763950077, 1030439870, 959586474, 3845661701,
        483795093, 1637944470, 2275463649, 3398804829, 1758016486, 2665513698, 2004912571,
        1094885097, 4223064276, 3307819021, 651121777, 1757003305, 3603542336, 129917786,
        2215974994, 3042386306, 2205352757, 3944939700, 3710987569, 97967515, 1217242524,
        930630949, 3660328512, 1787663098, 1784141600, 2500542892, 4034561586, 3444961378,
        785043562, 3869499367, 885623728, 2625011087, 3053789617, 1965731793, 3900511934,
        2648823592, 3851062028, 3321968688, 799195417, 1011847510, 1369129160, 1348009103,
        2876796955, 2915408967, 3305284948, 263399535, 1715990604, 2645821294, 1587844552,
        2624912049, 3035631499, 2306636348, 3499275462, 675152704, 854794152, 4004972748,
        1739996642, 1333476491, 4012621867, 3658792931, 3297985728, 2864481726, 3066357406,
        785287846, 1671499798, 433044045, 1919608025, 264833858, 3999983367, 1116778570,
        1301982149, 4213901070, 4081649357, 536169226, 1389008649, 188923873, 373495152,
        2551132278, 1800758715, 3951840330, 2632334454, 3118778225, 1034046547, 1862428410,
        3037609062, 1994608505, 29051798, 2571685694, 264151332, 2260643090, 2717535964,
        3508441116, 3283713017, 1903365635, 923575694, 1219598101, 2288281570, 3676533911,
        1014136356, 555142354, 2389170030, 4185108175, 884862419, 836141292, 2957159173,
        1997444768, 4233903127, 2876184692, 3089125070, 1480848293, 1097600237, 299700527,
        2507669891, 2982628312, 2114881043, 2529576251, 2812279824, 2987750993, 4241938954,
        2204775591, 1037094060, 829315638, 1231047149, 52608178, 3735136637, 3455232602, 962039123,
        488286513, 50685385, 3516451821, 843975207, 1572355722, 675489076, 2428445672, 1555117248,
        3708476086, 10375249, 4172112346, 2117510871, 2227658327, 3187664554, 3050656558,
        328034318, 3179601324, 1247769761, 3439263953, 1431538938, 2962525068, 1213366289,
        3813013550, 2651093719, 1860661503, 3933716208, 264320617, 789980519, 2257856172,
        102000748, 977269860, 1113845122, 3008928583, 1461738106, 557786285, 2926560363,
        1038106190, 3643478847, 828004507, 457818698, 1933056971, 373408056, 2076808229,
        3160935130, 2781854874, 2519636100, 177606000, 4237103862, 3977834316, 1621936232,
        2599050516, 319893558, 3343370366, 765044144, 976657331, 7026264, 294277429, 3829376742,
        3029627280, 2705178718, 3614653880, 230519152, 3288033233, 293525479, 3805751881,
        3227511198, 2520308544, 3648103003, 1111086184, 437622105, 2232033852, 3239146386,
        584244184, 1450926016, 2462430443, 3226534010, 298582169, 4214576928, 1762099469,
        964985185, 1585788148, 1641127666, 787006566, 2315956284, 3258232694, 2275058964,
        2541003317, 1508235863, 2613339827, 4080647514, 1152057965, 3149266279, 731345410,
        914737650, 65395712, 1884566942, 1379520432, 2611027720, 4163073378, 2619704967,
        2746552541, 1388822415, 3005141199, 843440249, 4288674003, 3136174279, 4051522914,
        4144149433, 3427566947, 3419023197, 3758479825, 3893877676, 96899594, 1657725776,
        253618880, 434129337, 1499045748, 2996992534, 4036042074, 2110713869, 906222950, 928326225,
        2541827893, 1604330202, 226792470, 4022228930, 815850898, 1466012310, 3377712199,
        292769859, 2822055597, 3225701344, 3052947004, 385831222, 705324593, 4030158636,
        3540280538, 2982120874, 2136414455, 255762046, 3852783591, 3262064164, 2358991588,
        3756586117, 4143612643, 3326743817, 2897365738, 807711264, 3719310016, 3721264861,
        3627337076, 944539331, 3640975513, 3712525681, 1162911839, 2008243316, 2179489649,
        2867584109, 261861553, 3570253908, 2062868357, 2220328623, 3857004679, 3744109002,
        4138041873, 1451860932, 2364975637, 2802161722, 2680106834, 753401584, 1223182946,
        1245401957, 4163377735, 3565815922, 2216942838, 4036140094, 71979081, 3924559643,
        400477238, 551750683, 1174153235, 859969898, 1185921017, 1711399735, 812991545, 4051735761,
        3549118738, 1631653329, 3631835958, 3648867800, 1206500363, 2155893137, 361030362,
        3454286017, 2505909489, 1083595169, 453595313, 1510564703, 1706163902, 1632924345,
        1381875722, 1661526119, 1082778324, 3571910052, 1140625929, 851544870, 1145546234,
        2938573139, 907528924, 1304752338, 1764668294, 1788942063, 1700368828, 104979467,
        1413911959, 3327497828, 1956384744, 1272712474, 2815637534, 3307809377, 1320574940,
        1111968962, 4073107827, 434096622, 169451929, 3201183459, 3331028877, 2852366972,
        3369830128, 2924794558, 3106537952, 3739481231, 1612955817, 4138608722, 2721281595,
        2755775390, 843505117, 982234295, 1157276611, 814674632, 4246504726, 3532006708, 992340967,
        1647538031, 204696133, 193866982, 3899126129, 300851698, 1379496684, 1759463683,
        1354782756, 1374637239, 3410883240, 1073406229, 3038431791, 1053909855, 3607043270,
        173719711, 3733903830, 171820911, 1573050589, 932781534, 4183534770, 2158849555, 372245998,
        3573073830, 841339264, 2759200520, 1610547277, 2603293319, 3890906486, 1557138278,
        3964109906, 677238797, 537994297, 1124184993, 4287078344, 4207654540, 2943022776,
        2977947524, 3255359985, 4098397558, 2274666217, 2915862060, 243524940, 2467726756,
        2869020032, 507521339, 3403121914, 522051455, 1803903108, 3471254194, 473535371,
        1948602036, 3352095732, 3116527002, 1795743673, 775867940, 2551469548, 3757442064,
        3162525227, 3765412747, 3040105484, 1927625810, 48214767, 2997207130, 1342349989,
        2536583992, 1501320191, 3592287317, 887432730, 967585477, 3334212779, 948663609,
        1064513472, 15386372, 2465931737, 3230242590, 3036652803, 2063155087, 1927500726,
        2821790499, 2187774383, 501520074, 3688568496, 3606711121, 2576459247, 3176542345,
        378322447, 156541411, 1400607301, 1406179107, 677848877, 2253753529, 193196070, 4207435024,
        4166396241, 509467541, 2906024136, 1221753746, 3375413222, 431327897, 2749265123,
        2848827671, 3412997614, 2051920238, 1283516885, 1300498239, 1957256104, 2634010560,
        3531900395, 360276850, 1461184973, 2012063967, 2873572430, 2914608609, 4289554777,
        1539331673, 1859532928, 4213441063, 538215691, 3512720863, 4258743698, 3040408445,
        982396546, 343095663, 4138069496, 1021581857, 214185242, 1968079460, 2864275059,
        3347192726, 4096783459, 3259169450, 3707808869, 142485006, 399610869, 230556456,
        2219467721, 4191227798, 2242548189, 3136366572, 179755707, 3464881829, 452317775,
        3887426070, 3446430233, 1473370015, 1576807208, 3964523248, 419325089, 2373067114,
        1596072055, 1928415752, 3635452689, 1005598891, 3335462724, 3290848636, 3669078247,
        1178176812, 2110774376, 3068593619, 1253036518, 908857731, 3631223047, 4138506423,
        2903592318, 3596915748, 3289036113, 3721512676, 2704409359, 3386016968, 3676268074,
        2185259502, 1096257611, 3360076717, 3548676554, 170167319, 3360064287, 3899940843, 9640,
    ];

    /// The maximum integer can be expressed in long scale.
    pub fn max_value() -> Self {
        Self {
            sign: Sign::Pos,
            data: BigUint::from_slice(Self::MAX_ABS_ARR),
        }
    }

    /// The minimum integer can be expressed in long scale.
    pub fn min_value() -> Self {
        Self {
            sign: Sign::Neg,
            data: BigUint::from_slice(Self::MAX_ABS_ARR),
        }
    }
}

#[cfg(feature = "bigint")]
impl ChineseNumeralBase for LongScaleBigInt {
    fn to_chars(&self) -> Vec<NumChar> {
        let mut chars = Vec::new();
        let mut num = self.data().to_owned();
        // 1000_0000_0000_0000
        let mut prev_rem = BigUint::new(vec![2764472320, 232830]);
        let mut lim = BigUint::new(vec![2764472320, 232830]);
        // 1_0000_0000_0000_0000
        let mut div = BigUint::new(vec![1874919424, 2328306]);
        let ten = BigUint::new(vec![10]);

        for exp in 14..=23 {
            let (_, rem) = num.div_rem(&div);
            num /= &div;

            if rem > BigUint::zero() {
                if !chars.is_empty() && prev_rem < lim {
                    chars.push(NUM_CHARS[0]);
                }
                if exp > 14 {
                    chars.push(NUM_CHARS[exp]);
                }
                let mut node = if exp <= 15 {
                    let rem = rem.to_u64().unwrap();
                    let mid = MidScaleInt::from(rem);
                    mid.to_chars()
                } else {
                    let long = Self::try_from(&rem).unwrap();
                    long.to_chars()
                };
                chars.append(&mut node);
            }
            prev_rem = rem;
            if exp > 14 {
                prev_rem *= &div;
                div = &div * &div;
                lim = &div / &ten;
            }
        }
        chars
    }

    fn to_chars_trimmed(&self) -> Vec<NumChar> {
        let mut chars = self.to_chars();
        let mut data = self.data().to_owned();
        let div = BigUint::new(vec![1_0000]);
        let ten = BigUint::new(vec![10]);
        let nineteen = BigUint::new(vec![19]);
        while data >= div {
            data /= &div;
        }
        if data >= ten && data <= nineteen {
            let one = chars.pop();
            debug_assert_eq!(one, Some(NumChar::One));
        }
        chars
    }
}
