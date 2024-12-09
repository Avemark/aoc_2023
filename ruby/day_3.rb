class Part1
  attr_reader :input

  def initialize(input)
    @input = input
  end

  def self.day
    3
  end

  def self.example
    new <<~STR
      xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
    STR
  end

  def self.validate
    example.sum == 161
  end

  def self.real
    new File.read("#{__dir__}/day_#{day}.txt").strip
  end

  class Multiplication
    def initialize(named_captures)
      @a = named_captures["a"].to_i
      @b = named_captures["b"].to_i
    end

    attr_reader :a, :b

    def to_s
      "#{a} x #{b} = #{a * b}"
    end

    def result
      a * b
    end

    def inspect
      "mul(#{a},#{b})"
    end
  end

  REGEX = /mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)/
  def multiplications
    offset = 0
    multiplications = []
    loop do
      match = input.match(REGEX, offset)
      break unless match
      offset = match.offset(0)[1]
      multiplications << Multiplication.new(match.named_captures)
    end
    multiplications
  end

  def sum
    multiplications.sum(&:result)
  end

  def to_s
    multiplications.map(&:to_s).join("\n")
  end

  def print
    puts to_s
  end

  def inspect
    "<#{self.class.name}: multiplications: [#{multiplications.map(&:inspect).join(", ")}], sum: #{sum}>"
  end
end

class Part2 < Part1
  COMMAND_REGEX = /(?<cmd>do|don't)\(\)/

  def self.example
    new <<~STR
      xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
    STR
  end

  def self.validate
    example.sum == 48
  end

  def multiplications
    offset = 0
    enabled = true
    multiplications = []
    loop do
      command = input.match(COMMAND_REGEX, offset)
      multiplication = input.match(REGEX, offset)
      break unless multiplication
      if command && command.offset(0)[0] < multiplication.offset(0)[0]
        offset = command.offset(0)[1]
        case command.named_captures["cmd"]
        when "do"
          enabled = true
        when "don't"
          enabled = false
        end
      else
        offset = multiplication.offset(0)[1]
        multiplications << Multiplication.new(multiplication.named_captures) if enabled
      end
    end
    multiplications
  end
end
