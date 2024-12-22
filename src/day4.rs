use aoc_runner_derive::{aoc, aoc_generator};

struct WordSearch {
    letters: Vec<Vec<char>>,
}

impl WordSearch {
    fn new(letters: Vec<Vec<char>>) -> WordSearch {
        WordSearch { letters }
    }

    fn count_occurrences_from(&self, x: usize, y: usize) -> u32 {
        let mut count = 0;

        if x < self.letters.len() - 3 {
            if self.letters[x][y] == 'X'
                && self.letters[x + 1][y] == 'M'
                && self.letters[x + 2][y] == 'A'
                && self.letters[x + 3][y] == 'S'
            {
                count += 1;
            }

            if y < self.letters[x].len() - 3 {
                if self.letters[x][y] == 'X'
                    && self.letters[x + 1][y + 1] == 'M'
                    && self.letters[x + 2][y + 2] == 'A'
                    && self.letters[x + 3][y + 3] == 'S'
                {
                    count += 1;
                }
            }

            if y > 2 {
                if self.letters[x][y] == 'X'
                    && self.letters[x + 1][y - 1] == 'M'
                    && self.letters[x + 2][y - 2] == 'A'
                    && self.letters[x + 3][y - 3] == 'S'
                {
                    count += 1;
                }
            }
        }

        if x > 2 {
            if self.letters[x][y] == 'X'
                && self.letters[x - 1][y] == 'M'
                && self.letters[x - 2][y] == 'A'
                && self.letters[x - 3][y] == 'S'
            {
                count += 1;
            }

            if y < self.letters[x].len() - 3 {
                if self.letters[x][y] == 'X'
                    && self.letters[x - 1][y + 1] == 'M'
                    && self.letters[x - 2][y + 2] == 'A'
                    && self.letters[x - 3][y + 3] == 'S'
                {
                    count += 1;
                }
            }

            if y > 2 {
                if self.letters[x][y] == 'X'
                    && self.letters[x - 1][y - 1] == 'M'
                    && self.letters[x - 2][y - 2] == 'A'
                    && self.letters[x - 3][y - 3] == 'S'
                {
                    count += 1;
                }
            }
        }

        if y < self.letters[x].len() - 3 {
            if self.letters[x][y] == 'X'
                && self.letters[x][y + 1] == 'M'
                && self.letters[x][y + 2] == 'A'
                && self.letters[x][y + 3] == 'S'
            {
                count += 1;
            }
        }

        if y > 2 {
            if self.letters[x][y] == 'X'
                && self.letters[x][y - 1] == 'M'
                && self.letters[x][y - 2] == 'A'
                && self.letters[x][y - 3] == 'S'
            {
                count += 1;
            }
        }

        count
    }

    fn count_occurrences(&self) -> u32 {
        let mut count = 0;

        for x in 0..self.letters.len() {
            for y in 0..self.letters[x].len() {
                count += self.count_occurrences_from(x, y);
            }
        }

        count
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> WordSearch {
    let letters = input.lines().map(|l| l.chars().collect()).collect();
    WordSearch::new(letters)
}

#[aoc(day4, part1)]
fn part1(input: &WordSearch) -> u32 {
    input.count_occurrences()
}
