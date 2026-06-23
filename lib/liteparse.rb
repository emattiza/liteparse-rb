require_relative "version"
require_relative "liteparse/liteparse"

module LiteParse
  class Error < StandardError; end
  ParseError = Class.new(Error)
end