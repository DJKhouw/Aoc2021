use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part1 = solve1(&input);
    println!("Part 1: {}", &part1);
}

pub fn get_output(input: &str) -> Vec<Vec<&str>> {
    let mut output = Vec::new();
    for line in input.lines() {
        let parts = line.split(" | ").collect::<Vec<_>>();
        output.push(parts[1].trim().split(' ').collect());
    }
    output
}
pub fn solve1(input: &str) -> usize {
    let output = get_output(input);
    output
        .iter()
        .flatten()
        .filter(|&&w| [2_usize, 3_usize, 4_usize, 7_usize].contains(&w.len()))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    static EXAMPLE_DATA: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
    #[test]
    fn test_get_output() {
        let output = get_output(EXAMPLE_DATA);
        assert_eq!(output.len(), 10);
        assert_eq!(output[0].len(), 4);
        assert_eq!(output[0][0].len(), 7);
        assert_eq!(output[0][0], "fdgacbe");
    }
    #[test]
    fn test_solve1() {
        assert_eq!(solve1(EXAMPLE_DATA),26);
    }
}
