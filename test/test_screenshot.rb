require_relative "test_helper"

class TestScreenshotBasic < Minitest::Test
  def setup
    @parser = create_parser
  end

  def test_screenshot_returns_array
    result = @parser.screenshot(SAMPLE_PDF)
    assert_kind_of Array, result
    assert result.length > 0
  end

  def test_screenshot_result_fields
    result = @parser.screenshot(SAMPLE_PDF)
    ss = result[0]
    assert_kind_of LiteParse::ScreenshotResult, ss
    assert_kind_of Integer, ss.page_num
    assert ss.page_num >= 1
    assert_kind_of Integer, ss.width
    assert ss.width > 0
    assert_kind_of Integer, ss.height
    assert ss.height > 0
  end

  def test_screenshot_has_image_bytes
    result = @parser.screenshot(SAMPLE_PDF)
    bytes = result[0].image_bytes
    assert_kind_of String, bytes
    assert bytes.length > 0
  end

  def test_screenshot_page_numbers_filter
    result = @parser.screenshot(SAMPLE_PDF, page_numbers: [1])
    assert_equal 1, result.length
    assert_equal 1, result[0].page_num
  end

  def test_screenshot_default_all_pages
    result = @parser.screenshot(SAMPLE_PDF)
    assert_equal 1, result.length
  end
end

class TestScreenshotErrors < Minitest::Test
  def test_file_not_found
    parser = create_parser
    assert_raises(RuntimeError) { parser.screenshot("/nonexistent/file.pdf") }
  end
end