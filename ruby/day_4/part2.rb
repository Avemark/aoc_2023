class Answer
  def initialize(input)
    @input = input
  end

  def letters
    @letters ||= @input.lines.map(&:chomp).map(&:chars)
  end

  def letter(x, y)
    x => Numeric
    y => Numeric

    return if x.negative?
    return if y.negative?
    return unless x < letters.length
    return unless y < letters.first.length
    letters[x][y]
  end

  def word(positions)
    positions.map {|x,y| letter(x,y).to_s }.join
  end

  def draw_letters(positions)
    opted = positions.group_by(&:first)
    rows = letters
      .map(&:dup)
      .map.with_index do |row, rindex|
        row.map.with_index do |letter, cindex|
          if opted[rindex].include?([rindex, cindex])
            letter
          else
            "."
          end
        end.join(" ")
    end
    puts rows.join("\n")
  end

  def each_cord
    height.times.flat_map do |x|
      width.times.map do |y|
        yield x,y
      end
    end
  end

  def width
    letters.first.count
  end

  def height
    letters.count
  end

  def self.example
    new File.read("#{__dir__}/example.txt").strip
  end

  def self.real
    new File.read("#{__dir__}/input.txt").strip
  end

  def self.validate
    self.example.result == 18
  end

  def proper_a(x,y)
    return false unless letter(x,y) == "A"

    first = [
      letter(x + 1, y + 1),
      letter(x - 1, y - 1),
    ]
    second = [
      letter(x + 1, y - 1),
      letter(x - 1, y + 1),
    ]
    return false if first.any?(&:nil?)
    return false if second.any?(&:nil?)
    first.sort == ["M", "S"] && second.sort == ["M", "S"]
  end

  def proper_as
    each_cord do |x,y|
      case letter(x,y)
      when "M", "S"
        if proper_a(x + 1, y + 1)
          [x + 1, y + 1 ]
        elsif proper_a(x + 1, y - 1)
          [x + 1, y - 1]
        else
          nil
        end
      else
        nil
      end
    end.compact.uniq
  end

  def count
    proper_as.count
  end

  def self.verify
    self.example.count == 9
  end
end