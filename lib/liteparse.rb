require_relative "liteparse/version"
require_relative "liteparse/liteparse"

module LiteParse
  class Error < StandardError; end
end

# Wrap native new to accept 0 args (the Rust constructor expects 1 positional arg).
LiteParse::LiteParse.singleton_class.alias_method :native_new, :new
LiteParse::LiteParse.define_singleton_method(:new) do |**kwargs|
  native_new(kwargs.empty? ? nil : kwargs)
end

# Wrap screenshot to accept page_numbers as keyword arg (matching Python API).
LiteParse::LiteParse.alias_method :native_screenshot, :screenshot
LiteParse::LiteParse.define_method(:screenshot) do |input, page_numbers: nil|
  native_screenshot(input, page_numbers)
end