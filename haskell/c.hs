import Control.Monad

-- some helper functions
th = flip (!!)
st = flip (!!)
nd = flip (!!)
rd = flip (!!)

main = do
    -- python like indentation
    when (5 > 2) $
        print "five is greater than 2"
    
    -- Some basic types
    let x = 10
    let x = 10.9
    let x = "Heyo!"
    let x = True
    let x = False

    -- lists
    let x = [1, 2, 3]
    let x = 1: 2: 3: 4: 5: 6: []

    -- printing and list access
    print $ 0 `th` x
    print $ 1 `st` x
    print $ 2 `nd` x

    -- length
    print $ length x

    -- conversion to string
    let x = "10"
    print $ read x + 10

    -- conversion from string
    let y = 20
    print $ "String: " ++ show y

    -- 'if' statement. 'when' is actually just a regular function
    let x = 5
    when (x > 9) $
        print("Hey")
        
    -- pattern matching
    let y = 11
    case y of
        11 -> print "Equal to 10"
        _ -> print "Not equal to 10"

    
    -- define a factorial (it's not in prelude)
    let fac 0 = 1
        fac x = x * fac (x - 1)

        -- define your own operator
        n `c` r = fac n / (fac (n - r) * fac r)
    
    print $ 5 `c` 2

