{-# LANGUAGE OverloadedStrings #-}
import Crypto.Hash.SHA256 (hash)
import qualified Data.ByteString.Char8 as BS
import Data.ByteString.Base16 (encode)
import System.Random

data Node = Node {
    name :: String,
    hashVal :: String,
    children :: [Node]
} deriving Show

sha256 :: String -> String
sha256 s = BS.unpack $ encode $ hash $ BS.pack s

generateName :: String -> IO String
generateName level = do
    n <- randomRIO (1,1000)
    return $ level ++ "_" ++ show n

generateNode :: String -> Int -> String -> IO Node
generateNode level depth parentHash = do
    n <- generateName level
    numChildren <- if depth > 0 then randomRIO (1,3) else return 0
    childNodes <- mapM (\_ -> 
        if depth == 3 then generateNode "Bundesland" (depth-1) parentHash
        else if depth == 2 then generateNode "Bezirk" (depth-1) parentHash
        else if depth == 1 then generateNode "Dorf" (depth-1) parentHash
        else return undefined
        ) [1..numChildren]
    let nodeHash = sha256 (n ++ parentHash ++ concatMap hashVal childNodes)
    return $ Node n nodeHash childNodes

printNode :: Node -> Int -> IO ()
printNode node indent = do
    putStrLn $ replicate (indent*2) ' ' ++ name node ++ " : " ++ hashVal node
    mapM_ (\c -> printNode c (indent+1)) (children node)

main :: IO ()
main = do
    tree <- generateNode "Land" 3 ""
    printNode tree 0
