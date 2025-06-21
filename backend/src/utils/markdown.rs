use comrak::{markdown_to_html, ComrakOptions};

/// 返回一个配置好安全选项和扩展的 ComrakOptions 实例。
fn get_comrak_options() -> ComrakOptions<'static> {
    let mut options = ComrakOptions::default();

    options.render.unsafe_ = false; // 禁止原始 HTML 标签，防止XSS
    options.extension.strikethrough = true; // 允许删除线
    options.extension.table = true; // 允许表格
    options.extension.tasklist = true; // 允许任务列表
    options.extension.footnotes = true; // 允许脚注
    options.extension.description_lists = true; // 允许描述列表
    // 未来可以在这里统一管理所有 Markdown 相关的配置
    options
}

/// 将 Markdown 字符串安全地转换为 HTML 字符串。
///
/// 此函数使用预设的安全配置来渲染 HTML，禁用了原始 HTML 以防止 XSS 攻击，
/// 并启用了一些常用的 GFM (GitHub Flavored Markdown) 扩展。
pub fn markdown_to_html_safe(markdown_input: &str) -> String {
    let options = get_comrak_options();
    markdown_to_html(markdown_input, &options)
}

// 使用 #[cfg(test)] 属性，这段代码只会在执行 `cargo test` 时被编译
#[cfg(test)]
mod tests {
    use super::*;
    // 导入父模块的所有内容，即 markdown_to_html_safe 函数

    #[test]
    fn test_basic_markdown_rendering() {
        let markdown = "# Hello\n\nThis is **bold** text.";
        let expected_html = "<h1>Hello</h1>\n<p>This is <strong>bold</strong> text.</p>\n";
        assert_eq!(markdown_to_html_safe(markdown), expected_html);
    }

    #[test]
    fn test_gfm_extensions_rendering() {
        // 测试表格
        let table_md = "| Head 1 | Head 2 |\n|--------|--------|\n| Cell 1 | Cell 2 |";
        let table_html = "<table>\n<thead>\n<tr>\n<th>Head 1</th>\n<th>Head 2</th>\n</tr>\n</thead>\n<tbody>\n<tr>\n<td>Cell 1</td>\n<td>Cell 2</td>\n</tr>\n</tbody>\n</table>\n";
        assert_eq!(markdown_to_html_safe(table_md), table_html);

        // 测试删除线
        let strikethrough_md = "This is ~~deleted~~ text.";
        let strikethrough_html = "<p>This is <del>deleted</del> text.</p>\n";
        assert_eq!(markdown_to_html_safe(strikethrough_md), strikethrough_html);
    }

    #[test]
    fn test_security_xss_sanitization() {
        // 这是最重要的安全测试！
        // 验证 <script> 标签被正确移除，替换为安全的注释
        let malicious_md = "Hello <script>alert('xss');</script> world!";
        let result = markdown_to_html_safe(malicious_md);

        // 确保输出不包含任何可执行的脚本标签
        assert!(!result.contains("<script"));
        assert!(!result.contains("</script>"));
        assert!(result.contains("<!-- raw HTML omitted -->"));

        // 验证 onclick 事件处理器的处理
        let event_handler_md = "<b onclick=alert('danger')>Click me</b>";
        let event_result = markdown_to_html_safe(event_handler_md);

        // 确保危险的属性和标签被安全处理
        // 无论是转义还是移除，都不应该包含可执行的内容
        assert!(!event_result.contains("<b onclick"));
        assert!(!event_result.contains("</b>"));

        // 内容本身应该被保留
        assert!(event_result.contains("Click me"));
    }

    #[test]
    fn test_security_ensures_no_dangerous_content() {
        // 额外的安全测试：确保各种危险内容都被正确处理
        let dangerous_inputs = vec![
            "<script>alert('xss')</script>",
            "<img src=x onerror=alert('xss')>",
            "<div onclick='alert(1)'>Click</div>",
            "<iframe src='javascript:alert(1)'></iframe>",
        ];

        for input in dangerous_inputs {
            let result = markdown_to_html_safe(input);

            // 关键安全检查：确保没有可执行的危险标签和属性
            assert!(!result.contains("<script>"));
            assert!(!result.contains("</script>"));
            assert!(!result.contains("<iframe"));
            assert!(!result.contains("javascript:"));

            // 对于事件处理器，检查它们不会以可执行形式出现
            // 只有当事件处理器在未转义的标签中才是危险的
            let has_executable_handler = (result.contains("onerror=") && !result.contains("&lt;"))
                || (result.contains("onclick=") && !result.contains("&lt;"));
            assert!(
                !has_executable_handler,
                "Found executable event handler in: {}",
                result
            );

            // 确保至少一部分内容被安全处理（转义或移除）
            let is_safely_processed = result.contains("<!-- raw HTML omitted -->")
                || result.contains("&lt;")
                || result.contains("&gt;");
            assert!(
                is_safely_processed,
                "Content should be safely processed: {}",
                result
            );

            println!("Input: {} -> Output: {}", input, result); // 调试输出
        }
    }

    #[test]
    fn test_empty_string_input() {
        let markdown = "";
        let expected_html = "";
        assert_eq!(markdown_to_html_safe(markdown), expected_html);
    }
}
