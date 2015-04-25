require "bad_idea/version"

Object.send(:remove_const, :Array) if defined?(Array)
require 'bad_idea/bad_idea'
