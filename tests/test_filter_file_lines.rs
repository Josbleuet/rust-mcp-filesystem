#[path = "common/common.rs"]
pub mod common;

use common::{create_temp_file, setup_service};
use rust_mcp_filesystem::tools::*;
use rust_mcp_sdk::schema::{ContentBlock, schema_utils::CallToolError};

#[tokio::test]
async fn regex_filter_basic_match() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Hello World\nFoo Bar\nHello Rust\nBaz Qux",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Regex,
        criteria: "Hello".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    assert_eq!(call_result.content.len(), 1);

    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 2"));
            assert!(text.text.contains("1: Hello World"));
            assert!(text.text.contains("3: Hello Rust"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn regex_filter_case_insensitive() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "HELLO World\nhello rust\nHeLLo there",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Regex,
        criteria: "hello".to_string(),
        options: Some(FilterOptions {
            case_insensitive: Some(true),
            ..Default::default()
        }),
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 3"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn regex_filter_case_sensitive() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "HELLO World\nhello rust\nHeLLo there",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Regex,
        criteria: "hello".to_string(),
        options: Some(FilterOptions {
            case_insensitive: Some(false),
            ..Default::default()
        }),
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 1"));
            assert!(text.text.contains("2: hello rust"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn regex_filter_complex_pattern() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "test123\ntest456\nabc789\ntest_file\ndata999",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Regex,
        criteria: r"test\d+".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 2"));
            assert!(text.text.contains("1: test123"));
            assert!(text.text.contains("2: test456"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn keywords_filter_single_keyword() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Error: Something went wrong\nInfo: All good\nError: Failed again",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "Error".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 2"));
            assert!(text.text.contains("Error: Something went wrong"));
            assert!(text.text.contains("Error: Failed again"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn keywords_filter_multiple_keywords_or_logic() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Error occurred\nWarning detected\nInfo message\nDebug statement",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "Error, Warning".to_string(),
        options: Some(FilterOptions {
            match_all: Some(false), // OR logic
            ..Default::default()
        }),
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 2"));
            assert!(text.text.contains("Error occurred"));
            assert!(text.text.contains("Warning detected"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn keywords_filter_multiple_keywords_and_logic() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Error: Critical failure\nWarning: Minor issue\nError and Warning both present\nJust info",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "Error, Warning".to_string(),
        options: Some(FilterOptions {
            match_all: Some(true), // AND logic
            ..Default::default()
        }),
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 1"));
            assert!(text.text.contains("Error and Warning both present"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn keywords_filter_whole_words_only() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "test file\ntesting phase\ntest\ncontest results",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "test".to_string(),
        options: Some(FilterOptions {
            whole_words: Some(true),
            ..Default::default()
        }),
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 2"));
            assert!(text.text.contains("test file"));
            assert!(text.text.contains("1: test file"));
            assert!(text.text.contains("3: test"));
            assert!(!text.text.contains("testing"));
            assert!(!text.text.contains("contest"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn keywords_filter_case_sensitivity() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "ERROR message\nerror lowercase\nError mixed\nno match",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "error".to_string(),
        options: Some(FilterOptions {
            case_insensitive: Some(false),
            ..Default::default()
        }),
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 1"));
            assert!(text.text.contains("2: error lowercase"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn lines_filter_single_line() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Lines,
        criteria: "3".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 1"));
            assert!(text.text.contains("3: Line 3"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn lines_filter_range() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Lines,
        criteria: "2-4".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 3"));
            assert!(text.text.contains("2: Line 2"));
            assert!(text.text.contains("3: Line 3"));
            assert!(text.text.contains("4: Line 4"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn lines_filter_multiple_ranges_and_singles() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Lines,
        criteria: "1-3,5,8-9".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 6"));
            assert!(text.text.contains("1: Line 1"));
            assert!(text.text.contains("2: Line 2"));
            assert!(text.text.contains("3: Line 3"));
            assert!(text.text.contains("5: Line 5"));
            assert!(text.text.contains("8: Line 8"));
            assert!(text.text.contains("9: Line 9"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn lines_filter_out_of_range() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Line 1\nLine 2\nLine 3",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Lines,
        criteria: "5-10".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 0"));
            assert!(text.text.contains("No matching lines found"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn context_before_and_after() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Line 1\nLine 2\nError here\nLine 4\nLine 5",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "Error".to_string(),
        options: Some(FilterOptions {
            context_before: Some(1),
            context_after: Some(1),
            ..Default::default()
        }),
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("2: Line 2")); // context before
            assert!(text.text.contains("3: Error here")); // match
            assert!(text.text.contains("4: Line 4")); // context after
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn context_multiple_lines() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Line 1\nLine 2\nLine 3\nError here\nLine 5\nLine 6\nLine 7",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "Error".to_string(),
        options: Some(FilterOptions {
            context_before: Some(2),
            context_after: Some(2),
            ..Default::default()
        }),
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("2: Line 2"));
            assert!(text.text.contains("3: Line 3"));
            assert!(text.text.contains("4: Error here"));
            assert!(text.text.contains("5: Line 5"));
            assert!(text.text.contains("6: Line 6"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn without_line_numbers() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Hello World\nFoo Bar\nHello Rust",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "Hello".to_string(),
        options: Some(FilterOptions {
            include_line_numbers: Some(false),
            ..Default::default()
        }),
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Hello World"));
            assert!(text.text.contains("Hello Rust"));
            assert!(!text.text.contains("1:"));
            assert!(!text.text.contains("3:"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn max_results_limit() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Error 1\nError 2\nError 3\nError 4\nError 5",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "Error".to_string(),
        options: Some(FilterOptions {
            max_results: Some(2),
            ..Default::default()
        }),
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            // The text should show that 5 matches were found
            assert!(text.text.contains("Matches found: 5"));
            // But only 2 results are displayed
            assert!(text.text.contains("1: Error 1"));
            assert!(text.text.contains("2: Error 2"));
            assert!(!text.text.contains("3: Error 3"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn invalid_file_path() {
    let (_temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);

    let params = FilterFileLines {
        path: "/nonexistent/file.txt".to_string(),
        filter_type: FilterType::Keywords,
        criteria: "test".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn invalid_regex_pattern() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Test content",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Regex,
        criteria: "[invalid(regex".to_string(), // Invalid regex pattern
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn invalid_line_number_format() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Line 1\nLine 2\nLine 3",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Lines,
        criteria: "abc".to_string(), // Invalid line number
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn path_outside_allowed_directories() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let unauthorized_dir = temp_dir.join("unauthorized");
    std::fs::create_dir_all(&unauthorized_dir).unwrap();
    let unauthorized_file = create_temp_file(
        &unauthorized_dir,
        "file.txt",
        "Unauthorized content",
    );

    let params = FilterFileLines {
        path: unauthorized_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "content".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_err());
    match result {
        Err(CallToolError { .. }) => {
            // Expected error type
        }
        _ => panic!("Expected CallToolError for unauthorized path"),
    }
}

#[tokio::test]
async fn empty_file() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "empty.txt",
        "",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "test".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 0"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn no_matches_found() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "sample.txt",
        "Line 1\nLine 2\nLine 3",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "nonexistent".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 0"));
            assert!(text.text.contains("No matching lines found"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn single_line_file() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);
    let test_file = create_temp_file(
        &temp_dir.join("test"),
        "single.txt",
        "Single line with keyword",
    );

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "keyword".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 1"));
            assert!(text.text.contains("1: Single line with keyword"));
        }
        _ => panic!("Expected TextContent"),
    }
}

#[tokio::test]
async fn large_file_with_many_matches() {
    let (temp_dir, service, _allowed_dirs) = setup_service(vec!["test".to_string()]);

    // Create a file with 1000 lines
    let mut lines = Vec::new();
    for i in 1..=1000 {
        if i % 10 == 0 {
            lines.push(format!("Line {} with ERROR", i));
        } else {
            lines.push(format!("Line {}", i));
        }
    }
    let content = lines.join("\n");
    let test_file = create_temp_file(&temp_dir.join("test"), "large.txt", &content);

    let params = FilterFileLines {
        path: test_file.to_str().unwrap().to_string(),
        filter_type: FilterType::Keywords,
        criteria: "ERROR".to_string(),
        options: None,
        line_range: None,
    };

    let result = FilterFileLines::run_tool(params, &service).await;
    assert!(result.is_ok());

    let call_result = result.unwrap();
    match &call_result.content[0] {
        ContentBlock::TextContent(text) => {
            assert!(text.text.contains("Matches found: 100"));
        }
        _ => panic!("Expected TextContent"),
    }
}
