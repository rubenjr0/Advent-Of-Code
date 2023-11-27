fn = "input"

course = File.read_lines(fn).map { |line|
  datum = line.split ' '
  {datum[0], datum[1].to_i32}
}

def run_course(course : Array(Tuple(String, Int32)))
  position = 0
  depth = 0
  course.each do |direction, quantity|
    case direction
    when "forward"
      position += quantity
    when "up"
      depth -= quantity
    when "down"
      depth += quantity
    end
  end
  position * depth
end

def run_course_with_aim(course : Array(Tuple(String, Int32)))
  position = 0
  depth = 0
  aim = 0
  course.each do |direction, quantity|
    case direction
    when "forward"
      position += quantity
      depth += quantity * aim
    when "up"
      aim -= quantity
    when "down"
      aim += quantity
    end
  end
  position * depth
end

puts "Part One: #{(run_course course)}"
puts "Part Two: #{(run_course_with_aim course)}"
