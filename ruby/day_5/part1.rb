class Part1
  def initialize(input)
    rules_input, report_input = input.split("\n\n")

    @rules = RuleSet.new(rules_input)
    @reports = make_reports(report_input)
  end

  def make_reports(input)
    input.lines.map { |line| Report.new(line.strip) }
  end

  def valid_reports
    @reports.select { |report| report.valid?(@rules) }
  end

  def result
    valid_reports.sum(&:midpoint)
  end

  def self.example
    new File.read("#{__dir__}/example.txt").strip
  end

  def self.real
    new File.read("#{__dir__}/input.txt").strip
  end

  def self.validate
    if self.example.result == 143
      puts "Validation successful"
    else
      puts "Validation failed"
    end
  end

  def to_s
    <<~SOLUTION
      <Solution
        #{@rules.to_s}

        #{@reports.map(&:to_s).join("\n")}
    SOLUTION
  end

  class RuleSet
    def initialize(rules)
      @rules = {}
      rules.lines.each do |line|
        precedent, antecedent = line.strip.split("|").map(&:to_i)
        @rules[precedent] ||= []
        @rules[precedent] << antecedent
      end
    end

    def antecedents(page)
      @rules[page]
    end

    def to_s
      @rules.inspect
    end
  end

  class Report
    def initialize(str)
      @pages = str.split(",").map(&:to_i)
    end

    def valid?(ruleset)
      @pages.each.with_index.all? do |page, index|
        previous = @pages[0 .. [index - 1, 0].max]
        violations = (ruleset.antecedents(page) || []) & previous
        violations.none?
      end
    end

    def midpoint
      @pages[@pages.count / 2]
    end

    def to_s
      "#{ @pages.inspect } midpoint: #{midpoint}"
    end
  end
end
