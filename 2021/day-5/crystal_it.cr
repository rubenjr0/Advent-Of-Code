struct Point
  property x
  property y

  def initialize(@x : Int16, @y : Int16)
  end

  def to_s(io)
    io << "(" << @x << "," << @y << ")"
  end
end

def filter_points(data, diags = false)
  data.select { |p1, p2|
    if p1.x == p2.x || p1.y == p2.y
      true
    elsif diags
      dx = (p1.x - p2.x).abs
      dy = (p1.y - p2.y).abs
      dx/dy == 1
    else
      false
    end
  }
end

data = File.read_lines("input")

point_pairs = Array({Point, Point}).new (data.size) { |i|
  line = data[i]
  l, r = line.split(" -> ")
  x1, y1 = l.split(',')
  x2, y2 = r.split(',')
  {Point.new(x1.to_i16, y1.to_i16), Point.new(x2.to_i16, y2.to_i16)}
}

def find_danger_points(points)
  danger_points = Hash(Point, UInt8).new
  points.each do |sol|
    p1, p2 = sol
    xi, xf = {p1.x, p2.x}
    yi, yf = {p1.y, p2.y}
    dx = if xf == xi
           0
         elsif xf > xi
           1
         else
           -1
         end
    dy = if yf == yi
           0
         elsif yf > yi
           1
         else
           -1
         end
    x = xi
    y = yi
    # puts "\t> Gotta go from (#{xi},#{yi}) to (#{xf},#{yf}) with steps of (#{dx},#{dy})"
    until x == xf && y == yf
      p = Point.new x, y
      danger_points[p] = danger_points.fetch(p) { 0_u8 } + 1

      x += dx
      y += dy
    end
    p = Point.new x, y
    danger_points[p] = danger_points.fetch(p) { 0_u8 } + 1
  end
  danger_points
end

straight_lines = filter_points point_pairs
danger_points = find_danger_points straight_lines
sol = danger_points.select { |k| danger_points[k] >= 2 }.size
puts "Part One: #{sol}"

diag_lines = filter_points point_pairs, true
danger_points = find_danger_points diag_lines
sol = danger_points.select { |k| danger_points[k] >= 2 }.size
puts "Part Two: #{sol}"
