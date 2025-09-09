use regex_tokenizer::{RegexTokenizer, Token};

struct TestCase<'a> {
    description: &'a str,
    input: &'a str,
    pattern: &'a str,
    expected: Vec<Token>,
}

#[test]
fn test_regex_tokenize() {
    let whitespace_pattern = r"[a-zA-Z0-9]+(?:[-.:'][a-zA-Z0-9]+)*|[.!?]+|[,;:]";
    let number_pattern = r"\d+(?:\.\d+)?";
    let punctuation_pattern = r"[.!?,;:]";

    let cases = [
        // Whitespace tokenizer pattern
        TestCase {
            description: "Whitespace tokenization pattern",
            pattern: whitespace_pattern,
            input: "Hello, world! It's 3.14.",
            expected: vec![
                Token {
                    text: "Hello".into(),
                    start: 0,
                    end: 5,
                },
                Token {
                    text: ",".into(),
                    start: 5,
                    end: 6,
                },
                Token {
                    text: "world".into(),
                    start: 7,
                    end: 12,
                },
                Token {
                    text: "!".into(),
                    start: 12,
                    end: 13,
                },
                Token {
                    text: "It's".into(),
                    start: 14,
                    end: 18,
                },
                Token {
                    text: "3.14".into(),
                    start: 19,
                    end: 23,
                },
                Token {
                    text: ".".into(),
                    start: 23,
                    end: 24,
                },
            ],
        },
        // Number-specific pattern
        TestCase {
            description: "Number pattern tokenization",
            pattern: number_pattern,
            input: "Values: 3.14, 22.5, and 100",
            expected: vec![
                Token {
                    text: "3.14".into(),
                    start: 8,
                    end: 12,
                },
                Token {
                    text: "22.5".into(),
                    start: 14,
                    end: 18,
                },
                Token {
                    text: "100".into(),
                    start: 24,
                    end: 27,
                },
            ],
        },
        // Punctuation-specific pattern
        TestCase {
            description: "Punctuation pattern tokenization",
            pattern: punctuation_pattern,
            input: "Hello! Is it working? Yes, it is.",
            expected: vec![
                Token {
                    text: "!".into(),
                    start: 5,
                    end: 6,
                },
                Token {
                    text: "?".into(),
                    start: 20,
                    end: 21,
                },
                Token {
                    text: ",".into(),
                    start: 25,
                    end: 26,
                },
                Token {
                    text: ".".into(),
                    start: 32,
                    end: 33,
                },
            ],
        },
        // Single-word sentence
        TestCase {
            description: "Single word",
            pattern: whitespace_pattern,
            input: "Rust",
            expected: vec![Token {
                text: "Rust".into(),
                start: 0,
                end: 4,
            }],
        },
        // Empty string
        TestCase {
            description: "Empty string",
            pattern: whitespace_pattern,
            input: "",
            expected: vec![],
        },
    ];

    for case in cases.iter() {
        let tokenizer = RegexTokenizer::new(case.pattern);
        let result = tokenizer.tokenize(case.input);

        assert_eq!(
            result.len(),
            case.expected.len(),
            "Length mismatch: {}",
            case.description
        );

        for (token, expected) in result.iter().zip(case.expected.iter()) {
            assert_eq!(
                token.text, expected.text,
                "Text mismatch: {}",
                case.description
            );
            assert_eq!(
                token.start, expected.start,
                "Start mismatch: {}",
                case.description
            );
            assert_eq!(
                token.end, expected.end,
                "End mismatch: {}",
                case.description
            );
        }
    }
}

#[test]
#[should_panic]
fn test_invalid_regex_pattern() {
    RegexTokenizer::new("[invalid");
}
