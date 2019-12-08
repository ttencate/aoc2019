const BLACK: u8 = b'0';
const WHITE: u8 = b'1';
const TRANSPARENT: u8 = b'2';

fn part1(input: &str) -> usize {
    let width = 25;
    let height = 6;
    let layer = input.trim()
        .as_bytes()
        .chunks(width * height)
        .min_by_key(|layer| layer.iter().filter(|&&c| c == b'0').count())
        .unwrap();
    layer.iter().filter(|&&c| c == b'1').count() * layer.iter().filter(|&&c| c == b'2').count()
}

fn decode_image(width: usize, height: usize, input: &str) -> String {
    let mut image = vec![b'.'; width * height];
    for layer in input.trim().as_bytes().chunks(width * height) {
        for (i, chr) in layer.iter().enumerate() {
            match *chr {
                BLACK => if image[i] == b'.' {
                    image[i] = b' ';
                },
                WHITE => if image[i] == b'.' {
                    image[i] = b'#';
                },
                TRANSPARENT => {},
                _ => panic!("Unknown pixel {}", chr),
            }
        }
    }
    image.chunks(width).map(|row| "\n".to_string() + &String::from_utf8(row.to_vec()).unwrap()).collect::<Vec<String>>().join("")
}

#[test]
fn test_parse_image() {
    assert_eq!(decode_image(2, 2, "0222112222120000"), "\n #\n# ");
}

fn part2(input: &str) -> String {
    decode_image(25, 6, input)
}

fn main() {
    aoc::main(part1, part2);
}

#[test]
fn test_answers() {
    aoc::test(part1, 1088, part2, "
#     ##  #   ##  # ###  
#    #  # #   ##  # #  # 
#    #     # # #### ###  
#    # ##   #  #  # #  # 
#    #  #   #  #  # #  # 
####  ###   #  #  # ###  ".to_string());
}
