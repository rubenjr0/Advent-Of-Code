function op_1(x) # 18
    (x + 3)
end

function op_2(x) # 23
    (x * 4)
end

function op_3(x) # 29
    (x * 12)
end

function op_4(x) # 13
    (x + 15)
end

function final(x)
    if op_1(x) % 18 == 0 
        if op_2(x) % 23 == 0
            (x |> op_1 |> op_2 |> op_3) % 29
        else
            (x |> op_1 |> op_2 |> op_4) % 13
        end
    else
        if op_3(x) % 29 == 0
            (x |> op_1 |> op_3 |> op_4) % 13
        else
            (x |> op_1 |> op_4 |> op_3) % 29
        end
    end
end

function equiv(x)
    for y in 1:1000
        if final(y) == final(x)
            return y
        end
    end
end

# 28 -> 28
# 29 -> 15
# 30 -> 1
# 31 -> 2