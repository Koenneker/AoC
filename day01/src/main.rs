use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), io::Error> {
  let test_result = day01_1("src/test.txt") ? ;
  println !("Day01_1 Test:  {}", test_result);
  let input_result = day01_1("src/input.txt") ? ;
  println !("Day01_1 Input: {}", input_result);
  println !("");

  let test_result_02 = day01_2("src/test.txt") ? ;
  println !("Day01_2 Test:  {}", test_result_02);
  let input_result_02 = day01_2("src/input.txt") ? ;
  println !("Day01_2 Input: {}", input_result_02);
  Ok(())
}

fn day01_1(filename : &str) -> Result<i64, io::Error> {
  let mut first_numbers = Vec::new ();
  let mut second_numbers = Vec::new ();

  let file = File::open(filename) ? ;
  let reader = BufReader::new (file);

      for
        line in reader.lines() {
          let line = line ? ;
          let numbers : Vec<&str> = line.split_whitespace().collect();

          if numbers
            .len() == 2 {
              if let
                Ok(first) = numbers[0].parse::<i64>() {
                  first_numbers.push(first);
                }
              if let
                Ok(second) = numbers[1].parse::<i64>() {
                  second_numbers.push(second);
                }
            }
        }

      first_numbers.sort();
      second_numbers.sort();

      let mut sum_of_differences = 0;

      for (first_number, second_number)
        in first_numbers.iter().zip(second_numbers.iter()) {
          sum_of_differences += (*first_number - *second_number).abs();
        }

      Ok(sum_of_differences)
}

fn day01_2(filename : &str) -> Result<i64, io::Error> {
  let mut first_numbers = Vec::new ();
  let mut second_numbers = Vec::new ();

  let file = File::open(filename) ? ;
  let reader = BufReader::new (file);

      for
        line in reader.lines() {
          let line = line ? ;
          let numbers : Vec<&str> = line.split_whitespace().collect();

          if numbers
            .len() == 2 {
              if let
                Ok(first) = numbers[0].parse::<i64>() {
                  first_numbers.push(first);
                }
              if let
                Ok(second) = numbers[1].parse::<i64>() {
                  second_numbers.push(second);
                }
            }
        }

      let mut second_counts
          : Vec<(i64, i64)> =
                second_numbers.into_iter()
                    .fold(
                        std::collections::HashMap::new (), | mut map,
                        val |
                            {
                              *map.entry(val).or_insert(0) += 1;
                              map
                            })
                    .into_iter()
                    .collect();

      first_numbers.sort();
      second_counts.sort_by(| (a, _), (b, _) | a.cmp(b));

      let mut first_iterator = first_numbers.iter().peekable();
      let mut second_iterator = second_counts.iter().peekable();

      let mut similarity_score = 0;

      while (first_iterator.peek().is_some() &&
             second_iterator.peek().is_some()) {
        if let (Some(&first_number), Some(&(second_number, count)))
          = (first_iterator.peek(), second_iterator.peek()) {
            if (first_number < second_number) {
              first_iterator.next();
            } else if (first_number == second_number) {
              similarity_score += second_number * count;
              first_iterator.next();
            } else {
              second_iterator.next();
            }
          }
      }

      Ok(similarity_score)
}
