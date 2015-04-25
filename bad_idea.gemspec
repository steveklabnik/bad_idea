# coding: utf-8
lib = File.expand_path('../lib', __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require 'bad_idea/version'

Gem::Specification.new do |spec|
  spec.name          = "bad_idea"
  spec.version       = BadIdea::VERSION
  spec.authors       = ["Steve Klabnik"]
  spec.email         = ["steve@steveklabnik.com"]

  spec.summary       = %q{This is a bad idea.}
  spec.description   = %q{This is a bad idea.}
  spec.homepage      = "https://github.com/steveklabnik/bad_idea"

  spec.files         = `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  spec.bindir        = "exe"
  spec.executables   = spec.files.grep(%r{^exe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.platform = Gem::Platform::CURRENT
  spec.extensions << "ext/bad_idea/extconf.rb"

  spec.add_development_dependency "bundler", "~> 1.9"
  spec.add_development_dependency "rake", "~> 10.0"
end
