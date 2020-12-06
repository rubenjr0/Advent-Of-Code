function decode(data, high, low_symbol, high_symbol)::Int16
    low::Int16 = 0
    for x in data
        mid = (high + low) / 2
        if x == low_symbol   
            low = mid
        else
            high = mid
        end
    end
    return low
end

function pass_id(pass)
    pass[1] * 8 + pass[2]
end

function non_consecutive_ids(ids)
    sort!(ids)
    for i in 1:(length(ids) - 1)
        if ids[i + 1] != ids[i] + 1
            return ids[i] + 1
        end
    end
end

data = open("2020\\d5\\input.txt") do f
    readlines(f);
end;

seats = [(decode(d[1:7], 128, 'B', 'F'), 
        decode(d[8:10], 8, 'R', 'L')) for d in data
]

ids = pass_id.(seats)

println("The highest id is $(maximum(ids))")
println("Your seat has the id $(non_consecutive_ids(ids))")