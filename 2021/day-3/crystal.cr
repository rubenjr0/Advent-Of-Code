require "bit_array"

data = File.read_lines("input").map { |line|
  Array(Bool).new (line.size) { |i|
    line[i] == '1'
  }
}

rows = data.size
cols = data[0].size

def get_column(col_index : Int32, xs : Array(Array(Bool)))
  Array(Bool).new (xs.size) { |i|
    xs[i][col_index]
  }
end

def get_common_bit(xs : Array(Bool), crit : Proc)
  ones = xs.count true
  n = xs.size
  crit.call(ones - n/2_f32)
end

def invert(xs : Array(Bool))
  Array(Bool).new (xs.size) do |i|
    !xs[i]
  end
end

def bin_to_dec(xs : Array(Bool))
  num = 0
  n = xs.size - 1
  xs.map_with_index { |val, i|
    val ? 2**(n - i) : 0
  }.sum
end

most_common_bit_crit = ->(ones_prop : Float32) { ones_prop >= 0 ? true : false }
least_common_bit_crit = ->(ones_prop : Float32) { ones_prop >= 0 ? false : true }

most_common_bits = Array(Bool).new (cols) do |i|
  get_common_bit (get_column i, data), most_common_bit_crit
end

least_common_bits = invert most_common_bits

gamma = bin_to_dec most_common_bits
epsilon = bin_to_dec least_common_bits

puts "Part One: #{gamma * epsilon}"

def get_life_support(data : Array(Array(Bool)), crit)
  i = 0
  until data.size == 1
    common_bit = get_common_bit (get_column i, data), crit
    data = data.select { |d| d[i] == common_bit }
    i += 1
  end
  data[0]
end

channel = Channel(Int32).new 2
spawn name: "Oxygen" do
  channel.send(bin_to_dec (get_life_support data, most_common_bit_crit))
end
spawn name: "CO2" do
  channel.send(bin_to_dec (get_life_support data, least_common_bit_crit))
end

puts "Part Two: #{channel.receive * channel.receive}"
