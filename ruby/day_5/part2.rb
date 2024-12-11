require "#{__dir__}/part1"

class Part2 < Part1
  def invalid_reports
    @reports.reject { _1.valid?(@rules)}
  end

  def make_reports(input)
    input.lines.map { |line| Report.new(line.strip) }
  end

  def result
    result = invalid_reports
      .each { |report| report.fix(@rules) }
      .sum(&:midpoint)

    if invalid_reports.any?
      puts "THERE ARE STILL INVALID REPORTS"
    end

    result
  end

  def self.validate
    result = self.example.result
    if result == 123
      puts "Validation successful"
    else
      puts "Validation failed: #{result} != 123"
    end
  end

  class Report < Part1::Report
    def fix(ruleset, steps = 0)
      @pages.each.with_index do |page, index|
        violations = (ruleset.antecedents(page) || []) & @pages[0 .. [index - 1, 0].max]

        next if violations.none?
        new_spot = violations.map {|culprit| @pages.index(culprit) }.max

        @pages.delete(page)
        @pages.insert(new_spot, page)
      end

      if steps < 20 && !valid?(ruleset)
        fix(ruleset, steps + 1)
      end
    end
  end
end