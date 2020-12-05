function find_pair_sum_to(data, n)
    for x in data
        rem = n - x
        if rem in data
            return (x, rem)
        end
    end
	return (nothing, nothing)
end

function find_triplet(data, n)
    for x in data
        rem = n - x
        (a, b) = find_pair_sum_to(data, rem)
		if a != b && a != x && b != x
        	return (a, b, x)
		end
    end
end

data = open("2020\\d1\\input.txt") do f
    parse.(Int32, readlines(f))
end

a1 = find_pair_sum_to(data, 2020)
a2 = find_triplet(data, 2020)

println("$a1 -> $(prod(a1))")
println("$a2 -> $(prod(a2))")