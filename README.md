# Primitive Tokenizer
Easiest-to-read implementation of a tokenizer you'll ever get.

## What is the point of building a Tokenizer from scratch?
1. I got frustrated by LLM hallucinations on my AI-app projects so I wanted to better understand how we can feed more comprehensive information to a transformer.
2. I always wondered how we contextualise images (not words) or videos [images && (words || onomatopoeia)] to a transformer. The educated guess is that everything is normalised to text then bytes but there are nuances I can only figure out by building it out.
3. I get that people want to write code concisely and optimally, but sometimes language specific optimisations e.g., `lamda` or `regex` leads me to make more bugs because I don't truly understand them.

## The big picture
Based on the most frequent byte pair combinations from "good" data, we *train* the tokenizer to selectively feed natural language to our transformer most efficiently.

Condition: We want to minimise the number of tokens and at the same time avoid minting bytepairs with not constructive semantics.

As for other conditions, I don't know yet.

### Tools
- Heuristics for BPE: https://github.com/karpathy/minbpe
- GPT4 Tokenizer in Rust: https://github.com/openai/tiktoken/blob/main/src/lib.rs
- Google Sentencepiece (BPE that merges BPE codepoints (e.g., utf-8 || utf-16 || utf-32) instead of only utf-8). I don't know why this is better, but apparently it can also *inference* tokenizers instead of just *training* them: https://github.com/google/sentencepiece

### Primitive
AI code that touches base with the simplest ideas in programming (e.g., hashmaps and loops)
