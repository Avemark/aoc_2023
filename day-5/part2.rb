
class TranslationTable
  class Translation
    def initialize(line)
      to, from, length = line.chomp.split(" ").map(&:to_i)
      @first = from
      @last = from + length - 1
      @delta = to - from
      @length = length
    end

    attr_reader :first, :last

    def translate(range)
      head = nil
      tail = nil

      if range.begin < first
        head = (range.begin..first - 1)
        range = (first..range.end)
      end

      if range.end > last
        tail = (last + 1..range.end)
        range = (range.begin..last)
      end

      result = (range.begin + @delta .. range.end + @delta)

      [head, result, tail]
    end

    def cover?(range)
      range.begin < last && range.end > first
    end

    def to_s
      "#{(first..last)} -> #{(first + @delta..last + @delta)}: #{@delta}"
    end
  end

  def initialize(lines)
    @name = lines.shift.split(" ").first
    @translations = lines.map { |line| Translation.new(line) }
  end

  attr_reader :name, :translations

  def to_s
    "#{name}\n#{translations.map(&:to_s).join("\n")}"
  end

  def translate(range)
    translation = @translations.find { |t| t.cover?(range) }
    if translation
      translation.translate(range)
    else
      [nil, range, nil]
    end
  end
end

class Seeds
  def initialize(ranges)
    @ranges = ranges.sort_by { |r| r.begin }
  end

  def translate(translation_map)
    todo = @ranges.dup
    results = []
    while todo.any?
      range = todo.shift
      head, result, tail = translation_map.translate(range)
      results << result
      todo.unshift tail if tail
      todo.unshift head if head
    end

    @ranges = results
    self
  end

  def min
    @ranges.map(&:begin).min
  end
end

class Solver
  def initialize(file)
    @input = File.read(file)
  end

  def solve
    blocks = @input.split("\n\n")
    seed_line = blocks.shift
    seed_pairs = seed_line.sub("seeds: ", "").split(" ").map(&:to_i)
    ranges = seed_pairs.each_slice(2).map { |(start,length)| (start .. (start + length - 1)) }
    seeds = Seeds.new(ranges)

    tables = blocks.map {|block| TranslationTable.new(block.split("\n")) }

    tables.each do |map|
      seeds.translate(map)
    end

    puts "\nDone!"
    puts seeds.min.inspect
  end
end

Solver.new("input.txt").solve
