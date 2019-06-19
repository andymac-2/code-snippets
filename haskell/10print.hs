main:: IO ()
main = do
    rand <- readFile "/dev/urandom"
    putStr . take 10000 . map slashify $ rand

slashify :: Char -> Char
slashify x = case fromEnum x `rem` 2 of
    1 -> '/'
    0 -> '\\'
    

header :: String
header = "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"100%\" height=\"100%\"><rect width=\"100%\" height=\"100%\" />"

footer :: String
footer = "</svg>"

line :: Bool -> String
line True = "<line x1=\"0\" y1=\"0\" x2=\"32\" y2=\"32\" stroke=\"white\" />"
line False = "<line x1=\"0\" y2=\"0\" x2=\"32\" y1=\"32\" stroke=\"white\" />"