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

### What I Learnt
Building Primitive Tokenizer led to me having my biggest 'AHA!' moment when it comes to machine learning.

- The most primitive binary state I can think of is existence. 0 = doesn't exist, 1 = exists or 0% vs. 100%. Computers thrive with binary states because they are deterministic, allowing for 100% of what you programmed to happen to happen (regardless of whether it should happen or not).
- Only when you gather contrasting/non-identical/variating binary states (e.g., {1, 0} -> 50%) is when you observe *proportions* and *percentages* of states, giving birth to **probabilistic** systems.
- Before all the matrix/tensor/attention confusion, Tokenization started from us gathering frequencies of bytepairs (discrete binary state!!!) and then prioritizing the bytepair with the highest frequency – which just so happens to be the bytepair type of highest proportion/percentage!
- I have no earthly idea what the driving forces were to certain bytepairs having more occurences than others, but what I can focus on is the fact that we've implemented an algorithm for *natural selective pressure*. The same inexplicable force that drives certain physical entities to exist and others not to. Or certain organisms to survive and dominate, and others not to.
- With an **objective** enough sample (values that emulate production of natural functions), we can train computational creatures to *replicate* the natural/physically efficient/economically efficient value to a task (survive, think, create, destroy).

I don't know the full details of how an algorithm with 1 sole purpose scales to a multitasking generative algorithm yet, but it is exciting and less overwhelming to know that the overarching principles of `input -> mixture of sole purpose functions -> output` remain the same for BPE and transformers alike, and dare I may say – the human body.

FYI: Just because an algorithm has a sole purpose does not mean it is deterministic. BPE is probabilistic because a different input leads to a variation of outputs. It is not like a letter on your keyboard, where you can only press it or leave it alone to either have a letter appear on your pixel grid or nothing at all. Again, I'm not going to bother worrying about quantum probabilities thinga majig (events are deterministic with perfect information) because my job is to just enforce *natural selective pressure* (for now).

### Tools
- Heuristics for BPE: https://github.com/karpathy/minbpe
- GPT4 Tokenizer in Rust: https://github.com/openai/tiktoken/blob/main/src/lib.rs
- Google Sentencepiece (BPE that merges BPE codepoints (e.g., utf-8 || utf-16 || utf-32) instead of only utf-8). I don't know why this is better, but apparently it can also *inference* tokenizers instead of just *training* them: https://github.com/google/sentencepiece

### Primitive
AI code that touches base with the simplest ideas in programming (e.g., hashmaps and loops)
