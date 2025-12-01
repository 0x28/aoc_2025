module Day01 (day01, part1, part2) where

import Text.Printf

day01 :: IO ()
day01 = do
  input <- readFile "input/input01.txt"
  let dirs = map parse $ lines input
  printf "part 1 = %d\n" $ part1 dirs
  printf "part 2 = %d\n" $ part2 dirs

parse :: String -> Int
parse ('L' : num) = -read num
parse ('R' : num) = read num
parse _ = error "parse error"

part1 :: [Int] -> Int
part1 input =
  part1' input 50
  where
    part1' (d : ds) curr
      | curr' == 0 = 1 + part1' ds curr'
      | otherwise = part1' ds curr'
      where
        curr' = (curr + d) `mod` 100
    part1' [] _ = 0

part2 :: [Int] -> Int
part2 input =
  part2' input 50
  where
    step dist pos
      | dist == 0 = 0
      | otherwise = c + step (dist - sign) (pos + sign)
      where
        c = if pos `mod` 100 == 0 then 1 else 0
        sign = signum dist
    part2' (d : ds) curr = step d curr + part2' ds ((curr + d) `mod` 100)
    part2' [] _ = 0
