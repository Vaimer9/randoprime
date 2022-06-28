import Prime
import System.Environment

main :: IO()

main = do
    args <- getArgs
    let number = read $ args !! 0 :: Int
    print $ (number, findPrimes number)
