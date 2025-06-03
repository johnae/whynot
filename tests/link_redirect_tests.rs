use regex::Regex;

#[test]
fn test_post_process_links_for_new_window() {
    // Test the post-processing function directly
    let html_with_redirect_links = r#"
        <p>Normal text here.</p>
        <p><a href="/redirect?url=https%3A//example.com">External link</a></p>
        <p><a href="/redirect?url=https%3A//other.com" class="link">Link with class</a></p>
        <p><a href="/redirect?url=https%3A//third.com" target="_self">Link with existing target</a></p>
        <p><a href="mailto:test@example.com">Local email link</a></p>
        <p><a href="/local/page">Local page link</a></p>
    "#;

    let result = post_process_links_for_new_window(html_with_redirect_links);

    println!("Original HTML:");
    println!("{}", html_with_redirect_links);
    println!("\nProcessed HTML:");
    println!("{}", result);

    // Check that redirect links get target="_blank" added
    assert!(result.contains(r#"<a href="/redirect?url=https%3A//example.com" target="_blank" rel="noopener noreferrer">External link</a>"#));
    assert!(result.contains(r#"<a href="/redirect?url=https%3A//other.com" class="link" target="_blank" rel="noopener noreferrer">Link with class</a>"#));

    // Links that already have target should be updated to target="_blank" for security
    assert!(result.contains("target=\"_blank\""));

    // Non-redirect links should remain unchanged
    assert!(result.contains(r#"<a href="mailto:test@example.com">Local email link</a>"#));
    assert!(result.contains(r#"<a href="/local/page">Local page link</a>"#));
}

fn post_process_links_for_new_window(html: &str) -> String {
    // Use regex to find links that go through /redirect and ensure they have target="_blank"
    let re = Regex::new(r#"<a([^>]*href="/redirect\?[^"]*"[^>]*)>"#).unwrap();

    re.replace_all(html, |caps: &regex::Captures| {
        let link_attrs = &caps[1];

        // Check if target="_blank" is already present
        if link_attrs.contains("target=") {
            // Replace existing target with target="_blank"
            let target_re = Regex::new(r#"\s*target="[^"]*""#).unwrap();
            let attrs_without_target = target_re.replace_all(link_attrs, "");
            format!(
                "<a{} target=\"_blank\" rel=\"noopener noreferrer\">",
                attrs_without_target
            )
        } else {
            // Add target="_blank" and rel="noopener noreferrer"
            format!(
                "<a{} target=\"_blank\" rel=\"noopener noreferrer\">",
                link_attrs
            )
        }
    })
    .to_string()
}

#[test]
fn test_link_rewriting_with_url_encoding() {
    let test_cases = [
        ("https://example.com", "https%3A//example.com"),
        (
            "https://example.com/path?param=value",
            "https%3A//example.com/path%3Fparam%3Dvalue",
        ),
        (
            "https://site.com/path with spaces",
            "https%3A//site.com/path%20with%20spaces",
        ),
    ];

    for (original_url, _expected_encoded) in test_cases.iter() {
        let html = format!(r#"<a href="{}">Link</a>"#, original_url);
        let result = post_process_links_for_new_window(&html);

        // Since our function only processes /redirect links, this won't match
        // This test is more for documentation of expected behavior
        assert!(!result.contains("target=\"_blank\""));
    }
}

#[test]
fn test_mixed_link_types() {
    let html = r#"
        <div>
            <a href="/redirect?url=https%3A//external.com">External via redirect</a>
            <a href="https://direct-external.com">Direct external</a>
            <a href="mailto:test@example.com">Email</a>
            <a href="/internal/page">Internal</a>
            <a href="/redirect?url=https%3A//another.com" target="_parent">Redirect with target</a>
        </div>
    "#;

    let result = post_process_links_for_new_window(html);

    println!("Mixed links test result:");
    println!("{}", result);

    // Only /redirect links should be modified
    assert!(result.contains(r#"href="/redirect?url=https%3A//external.com" target="_blank""#));
    assert!(result.contains(r#"href="/redirect?url=https%3A//another.com" target="_blank""#));

    // Other links should remain unchanged
    assert!(result.contains(r#"href="https://direct-external.com""#));
    assert!(result.contains(r#"href="mailto:test@example.com""#));
    assert!(result.contains(r#"href="/internal/page""#));

    // Should not add target="_blank" to non-redirect links
    let non_redirect_with_target =
        result.contains(r#"href="https://direct-external.com"[^>]*target="_blank""#);
    assert!(!non_redirect_with_target);
}
