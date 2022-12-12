readInt :: String -> Int
readInt = read

df :: [Int] -> [Int]
df (x:xs) = run x xs []
    where
        run x' [] acc = acc
        run x' [x2:xs'] acc = run x2 xs' ((x2 - x') : acc)


main = do
    contents <- readFile "input"
    let input = map readInt . words $ contents
    print input