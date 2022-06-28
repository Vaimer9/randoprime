module Prime
( isPrime
, findPrimes
-- , printPrimes
) where

isPrime :: Int -> Bool
isPrime k = if k > 1 then null [ x | x <- [2..k - 1], isFactor (k, x)] else False

findPrimes :: Int -> [Int]
findPrimes x = filter (isPrime) [2..(x - 1)]

-- Enter into the world of IO
printPairs :: (Int, Int) -> IO()
printPairs (x, y) = print $ show(x) ++ " | " ++ show y

invokePrint :: (Int, Int) -> IO()
invokePrint (x, y) = if isFactor (x, y) then printPairs (x, y) else 

-- Utility functions
removeFirst :: [a] -> [a]
removeFirst = \x ->
    case x of
        [] -> []
        x:xs -> xs

isFactor :: (Int, Int) -> Bool
isFactor (x, y) = x `mod` y == 0
