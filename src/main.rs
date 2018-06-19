
// 0は使わず、A=14とする.
const NUMS: [u16; 14] = [
    0b10000000000001, // A = 1 or 14
    0b00000000000010,
    0b00000000000100,
    0b00000000001000,
    0b00000000010000,
    0b00000000100000,
    0b00000001000000,
    0b00000010000000,
    0b00000100000000,
    0b00001000000000,
    0b00010000000000,
    0b00100000000000,
    0b01000000000000,
    0b10000000000001, // A = 1 or 14
];

const STRAIGHT: u16 = 0b11111000000001;

fn hand_score(hands: &mut [u8; 5]) -> u32 {
    hands.sort();

    let mut hand_nums = 0;
    let mut hand_suit = 0;
    let mut pre = 15;
    let mut hand_nums2: u32 = 0;

    for card in hands {
        let num = *card / 4;
        let suit = *card % 4;
        let num = if num == 0 {13} else {num};
        println!("num: {}, suit: {}", num, suit);

        if num == pre {
            // cont += 1;
            hand_nums2 += 1;
        } else {
            hand_nums2 = hand_nums2 << 4;
            hand_nums2 += num as u32;
            hand_nums2 = hand_nums2 << 2;
            pre = num;
        }

        hand_suit |= 0b0001 << suit;
        hand_nums |= NUMS[num as usize];
    }
    let is_straight = (0..9).fold(false, |r, s| r || (STRAIGHT >> s) ^ hand_nums == 0);
    let is_straight = hand_nums ^ (0b10000000011111) == 0 || is_straight;
    let is_flush = (0..4).fold(false, |r, s| r || (0b1000 >> s) ^ hand_suit == 0);

    let mut four = 0;
    let mut three = 0;
    let mut pairs = 0;
    let mut highs = 0;
    let kind_checker = 0b000011;
    let num_checker  = 0b111100;
    for i in 0..5 {
        let num = ((hand_nums2 >> (6 * i)) & num_checker) >> 2;
        // let num = if num == 0 {14} else {num};
        let set = (hand_nums2 >> (6 * i)) & kind_checker;
        println!("{}, {:b}", i, hand_nums2 >> (6*i));
        println!("num2: {}", num);
        println!("kind: {}", set);
        match set {
            1 => { 
                pairs = pairs << 4;
                pairs += num;
            },
            2 => {
                println!("three");
                three = num;
            },
            3 => {
                println!("four");
                four = num;
            },
            _ => {
                if num != 0 {
                    highs <<= 4;
                    highs += num;
                }
            }
        }
    }

    // Debug print
    println!("---------------");
    println!("hand: {:013b}", hand_nums);
    println!("hand2: {:030b}", hand_nums2);
    println!("suits: {:04b}", hand_suit);
    println!("isStraight: {}", is_straight);
    println!("isFlush: {}", is_flush);

    println!("four: {}", four);
    println!("three: {}", three);
    println!("pairs: {:08b}", pairs);
    print!("pairs: ");
    for i in 0..2 {
        print!("{} ", (pairs >> ((1-i) * 4)) & 0b1111);
    }
    println!("\nhighs: {:020b}", highs);
    print!("highs: ");
    for i in 0..5 {
        print!("{} ", (highs >> ((4-i) * 4)) & 0b1111);
    }
    println!("");

    return 0;
}


fn main() {
    // // STRAIGHT
    // let mut hands = [0,1,2,3,4];
    // hand_score(&mut hands);
    // let mut hands = [13,9,10,11,12];
    // hand_score(&mut hands);
    //
    // // FLUSH
    // let mut hands = [4,8,12,20,52];
    // hand_score(&mut hands);
    //
    // // FOUR OF A KIND
    // let mut hands = [4,5,6,7,15];
    // hand_score(&mut hands);
    //
    // FULL HOUSE
    // let mut hands = [4,5,6,20,21];
    // hand_score(&mut hands);

    // TWO PAIR
    let mut hands = [4,5,20,21,50];
    hand_score(&mut hands);
}
