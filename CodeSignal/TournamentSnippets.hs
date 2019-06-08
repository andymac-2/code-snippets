import Text.Parsec hiding (Error)
import Text.Parsec.Char
import Text.Parsec.String





pointInLine [x, y] [a, b, c] = a * x + b * y + c == 0


countLineColorings points colors = colors * ((colors - 1) ^ (points - 1))

sumOfMultiples n k = sum [k, 2*k .. n]

sumOfCubes n = sum $ fmap (^3) [1..n]

import Data.List
isInformationConsistent evidences = and $ fmap (\x -> all (>=0) x || all (<=0) x) people where
    people = transpose evidences

countWaysToChangeDigit :: Int -> Int
countWaysToChangeDigit = sum . fmap ((9-) . read . (:[])) . show


weakNumbers n = [min, length $ filter (==min) allWeakness] where
    allWeakness = fmap weakness [1..n]
    min = minimum allWeakness

d :: Int -> [Int]
d n = filter (\x -> n `mod` x == 0) [1..n]

weakness :: Int -> Int
weakness n = filter (> d n) $ fmap d [1..n]

numberOfOperations a b
    | a `rem` b == 0 = 1 + numberOfOperations (a `div` b) b
    | b `rem` a == 0 = 1 + numberOfOperations (b `div` a) a
    | otherwise = 0


fibonacciNumber n = fibs !! n
fibs = 0: 1: zipWith (+) fibs (tail fibs)

smallestMultiple left right = foldr1 lcm [left..right]


toAndFro a b t = toB t where
    toB t
        | t > abs (a - b) = toA (t - abs (a - b))
        | a < b = a + t
        | a > b = a - t
        
    toA t
        | t > abs (a - b) = toB (t - abs (a - b))
        | b < a = b + t
        | b > a = b - t
