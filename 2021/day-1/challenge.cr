def df(xs)
  Array(Int16).new (xs.size - 1) { |i| xs[i + 1] - xs[i] }
end

input = File.read_lines("input").map { |x| x.to_i16 }

df_1 = df input

puts df_1.select { |x| x > 0 }.size

windows = Array(Int16).new (input.size - 2) { |i|
  input[i + 2] + input[i + 1] + input[i]
}

df_2 = df windows

puts df_2.select { |x| x > 0 }.size
