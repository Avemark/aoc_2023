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

  def traces(x, y)
    return [] unless letter(x, y) == "X"

    directions = [-1, 0, 1].flat_map do |dx|
      [-1, 0, 1].map do |dy|
        [dx,dy]
      end
    end

    directions.map do |dx, dy|
      if letter(x + dx, y + dy) == "M"
        Trace.new(x,y).set_direction(dx,dy)
      end
    end
  end

  class Trace
    attr_reader :x, :y, :dx, :dy

    def initialize(x,y)
      @x = x
      @y = y
    end

    def set_direction(dx, dy)
      @dx = dx
      @dy = dy
      self
    end

    def eql(other)
      @x == other.x && @y == other.y && @dx == other.dx && @dy == other.dy
    end

    def position(step_count)
      [
        x + dx * step_count,
        y  + dy * step_count,
      ]
    end

    def all
      (0..3).map do |i|
        position(i)
      end
    end
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

  def width
    letters.first.count
  end

  def height
    letters.count
  end

  def each_cord
    height.times.flat_map do |x|
      width.times.map do |y|
        yield x,y
      end
    end
  end

  def all_traces
    each_cord { |x,y| traces(x,y) }.flatten.compact
  end

  def result
    all_traces.count do |trace|
      word(trace.all) == "XMAS"
    end
  end

  def is_letter?(trace,sought)
    letter(*trace.current) == sought
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
end