# frozen_string_literal: true

require "fileutils"
require "tempfile"

# This is a simple script to count the number of lines in all Rust files in the
# current repository. It is not meant to be super efficient, and is mostly
# cobbled together with the help of GitHub CoPilot.

# get all rs files and calculate line count
def get_rs_files
  Dir.glob("**/*.rs")
end

def calculate_file_line_count(file)
  count = 0
  File.open(file, "r") do |f|
    f.each_line do |line|
      count += 1
    end
  end
  count
end

def generate_badge_link(total_count)
  "![lines of code](https://img.shields.io/badge/lines%20of%20rust-#{total_count}-informational)"
end

total_count = 0
get_rs_files.each do |file|
  next if file.include?("target")

  intermediate_count = calculate_file_line_count(file)
  total_count += intermediate_count
  puts "#{file} #{intermediate_count}"
end

puts "Total: #{total_count}"

puts "updating the readme badge..."
temp = Tempfile.new "readme_temp.md"
File.open("README.md", "r") do |f|
  f.each_line do |line|
    if line.start_with?("![lines of code]")
      temp.puts(generate_badge_link(total_count))
    else
      temp.puts line
    end
  end
end
temp.close
FileUtils.mv(temp.path, "README.md")

for arg in ARGV do
  if arg == "--nocommit"
    puts "not committing changes to the readme"
    exit
  end
end

puts "checking git"
git_status = `git status --porcelain`
if git_status.length == 0
  puts "nothing to commit"
else
  puts "staging..."
  puts "git add ."
  puts `git add .`
  puts "committing..."
  puts "git commit -m stats.rb"
  puts `git commit -m stats.rb`
end
