import Data.Foldable

dfac 0 = 1
dfac 1 = 1
dfac n = n * dfac(n - 2)

dfac1 :: Int -> Int
dfac1 n = go 1 n where
    go acc 0 = acc
    go acc 1 = acc
    go acc n = go (acc * n) (n - 2)

dfac2 :: Int -> Int
dfac2 n = foldl' (*) 1 [n, (n - 2) .. 1]