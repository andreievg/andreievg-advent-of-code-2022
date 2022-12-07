pub(crate) fn solve1(input: &str) -> u32 {
    sums(input).into_iter().max().unwrap()
}

pub(crate) fn solve2(input: &str) -> u32 {
    let mut result = sums(input);
    result.sort_by(|a, b| b.cmp(a));
    result[..3].into_iter().fold(0, |acc, v| acc + v)
}

fn sums(input: &str) -> Vec<u32> {
    input
        .trim()
        .replace(" ", "")
        .split("\n\n")
        .map(|elf_count_combined| {
            elf_count_combined
                .split("\n")
                .fold(0, |acc, string_calories| {
                    acc + string_calories.parse::<u32>().unwrap()
                })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &'static str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    #[test]
    fn day1_1() {
        assert_eq!(solve1(INPUT), 24000)
    }

    #[test]
    fn day1_2() {
        assert_eq!(solve2(INPUT), 45000)
    }
}
