require_relative "test_helper"

class TestParseBasic < Minitest::Test
  def setup
    @parser = create_parser
  end

  def test_parse_returns_result
    result = @parser.parse(SAMPLE_PDF)
    assert_kind_of LiteParse::ParseResult, result
  end

  def test_result_has_pages
    result = @parser.parse(SAMPLE_PDF)
    assert result.pages.length > 0
    assert_equal result.pages.length, result.num_pages
  end

  def test_result_has_text
    result = @parser.parse(SAMPLE_PDF)
    assert_kind_of String, result.text
    assert result.text.length > 0
  end

  def test_parse_bytes
    data = File.binread(SAMPLE_PDF).bytes
    result = @parser.parse_bytes(data)
    assert_kind_of LiteParse::ParseResult, result
    assert result.text.length > 0
  end

  def test_multi_page_parse
    result = @parser.parse(SWIM_MEET_PDF)
    assert_equal 12, result.num_pages
  end

  def test_multi_page_text_joined
    result = @parser.parse(SAMPLE_PDF)
    pages_text = result.pages.map(&:text).join("\n\n")
    assert_equal result.text, pages_text
  end
end

class TestParsedPageStructure < Minitest::Test
  def setup
    @parser = create_parser
    @result = @parser.parse(SAMPLE_PDF)
    @page = @result.pages[0]
  end

  def test_page_fields
    assert_kind_of Integer, @page.page_num
    assert @page.page_num >= 1
    assert_kind_of Float, @page.width
    assert @page.width > 0
    assert_kind_of Float, @page.height
    assert @page.height > 0
    assert_kind_of String, @page.text
  end

  def test_page_has_text_items
    assert @page.text_items.length > 0
  end

  def test_text_item_fields
    item = @page.text_items[0]
    assert_kind_of String, item.text
    assert_kind_of Float, item.x
    assert_kind_of Float, item.y
    assert_kind_of Float, item.width
    assert_kind_of Float, item.height
    refute_nil item.font_name
    refute_nil item.confidence
  end
end

class TestParseOptions < Minitest::Test
  def setup
    @parser = create_parser
  end

  def test_target_pages
    parser = LiteParse::LiteParse.new(target_pages: "1")
    result = parser.parse(SWIM_MEET_PDF)
    assert_equal 1, result.num_pages
    assert_equal 1, result.pages[0].page_num
  end

  def test_max_pages
    parser = LiteParse::LiteParse.new(max_pages: 1)
    result = parser.parse(SWIM_MEET_PDF)
    assert_equal 1, result.num_pages
  end

  def test_dpi_setting
    parser = LiteParse::LiteParse.new(dpi: 300)
    result = parser.parse(SAMPLE_PDF)
    assert result.text.length > 0
  end

  def test_ocr_enabled_false
    parser = LiteParse::LiteParse.new(ocr_enabled: false)
    result = parser.parse(SAMPLE_PDF)
    assert result.text.length > 0
  end

  def test_image_file_parse
    skip_unless_imagemagick
    result = @parser.parse(RECEIPT_PNG)
    assert result.text.length > 0
    assert_includes result.text, "$"
  end

  def test_get_page_helper
    result = @parser.parse(SAMPLE_PDF)
    page1 = result.get_page(1)
    refute_nil page1
    assert_equal 1, page1.page_num
    assert_nil result.get_page(999)
  end
end

class TestParseErrors < Minitest::Test
  def test_file_not_found
    parser = create_parser
    assert_raises(RuntimeError) { parser.parse("/nonexistent/file.pdf") }
  end

  def test_bad_pdf_bytes
    parser = create_parser
    assert_raises(RuntimeError) { parser.parse_bytes([0, 1, 2]) }
  end
end