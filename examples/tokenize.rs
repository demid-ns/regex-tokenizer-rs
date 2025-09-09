use regex_tokenizer::RegexTokenizer;

fn main() {
    let text = "It's state-of-the-art! The price is $3.14... Really?! Time: 12:30.";
    
    // Pattern explanation:
    // [a-zA-Z0-9]+(?:[-.:'][a-zA-Z0-9]+)* - Words with optional internal punctuation (hyphens, periods, colons, apostrophes)
    // |[.!?]+                             - Multiple punctuation marks (periods, exclamation marks, question marks)
    // |[,;:]                              - Single punctuation marks (commas, semicolons, colons)
    let pattern = r"[a-zA-Z0-9]+(?:[-.:'][a-zA-Z0-9]+)*|[.!?]+|[,;:]";
    let regex_tokenizer = RegexTokenizer::new(pattern);

    let tokens = regex_tokenizer.tokenize(text);

    println!("{:?}", tokens);
    
    // Expected output:
    // [
    //     Token { text: "It's", start: 0, end: 4 },
    //     Token { text: "state-of-the-art", start: 5, end: 21 },
    //     Token { text: "!", start: 21, end: 22 },
    //     Token { text: "The", start: 23, end: 26 },
    //     Token { text: "price", start: 27, end: 32 },
    //     Token { text: "is", start: 33, end: 35 },
    //     Token { text: "3.14", start: 37, end: 41 },
    //     Token { text: "...", start: 41, end: 44 },
    //     Token { text: "Really", start: 45, end: 51 },
    //     Token { text: "?!", start: 51, end: 53 },
    //     Token { text: "Time", start: 54, end: 58 },
    //     Token { text: ":", start: 58, end: 59 },
    //     Token { text: "12:30", start: 60, end: 65 },
    //     Token { text: ".", start: 65, end: 66 }
    // ]
}
