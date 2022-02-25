Unlocking Parsers|unlocking-parsers|2022-02-25

# Unlocking Parsers
_Published February 25, 2022_
<hr>

Recently I've been taking a course titled "Programming Language Concepts", a class dedicated to understanding how to construct,
analyze, and parse grammars for programming languages. The techniques I've gleaned from this class have opened my eyes to a new way to solve problems using simple parsing techniques, which I hope to display today.

## Goal

My end goal in this article is to build a small program that takes an input
string that describes a number and returns the integer representation of that number. For instance:

```rust
assert_eq!(3, parse("three"));
assert_eq!(13, parse("thirteen"));
assert_eq!(27, parse("twenty-seven"));
assert_eq!(1533, parse("one thousand and five hundred thirty-three"));
```

At first glance, this problem seems daunting. Handling the first two cases can be accomplished via a dictionary, but more complex inputs such as the third and fourth cases don't lend an obvious way to solve them.
Before I can solve this problem, I need to understand what I'm looking for in the first place.

## Defining The Language

A crucial step in this process is defining the input strings that are acceptable or unacceptable. The simplest way to do this is to state all applicable rules for input strings, which, when considered all together, constitutes the language's grammar.

### Quick Intro to BNF

The syntax for how grammars are defined is called [BNF, Bauckus-Naur Form.](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form)
As an example, let's create a grammar for a simple language called "list of
numbers", denoted here as `lon`. `lon` can accept the following strings as
input:

```
()
(5)
(5 5 238 1 0)
```

Pretty simple, here is what the grammar for such a language would look like:

```bnf
<lon> ::= ( <lon_nums> )

<lon_nums> ::= NUM <lon_nums>
<lon_nums> ::=
```

Each line is called a "production", any word inside brackets a
"class", and any word not inside brackets a token. The only tokens in this
grammar are `(`, `)`, and `NUM`, where `NUM` can be any valid number. Notice that the last production for `<lon_nums>` contains no rule, which means an empty string can replace it.

As long as classes exist in the input string, the production rules define replacements for the classes. This process continues until there are no classes remaining. For instance, here is how this grammar generates `(5 5 238 1 0)`:

```bnf
<lon>
( <lon_nums> )
( 5 <lon_nums> )
( 5 5 <lon_nums> )
( 5 5 238 <lon_nums> )
( 5 5 238 1 <lon_nums> )
( 5 5 238 1 0 <lon_nums> )
( 5 5 238 1 0 <lon_nums> )
( 5 5 238 1 0 )
```
Starting with the root symbol `<lon>`, each production defines a replacement for each class. The recursive nature of `<lon_nums>` enables generating any number of numbers.

### A More Complex Grammar

But I'm not trying to parse a list of integers, I want to parse the name of a number! Starting with the easiest case, a single digit number is defined as:

```bnf
<ones> ::= zero
<ones> ::= one
<ones> ::= two
<ones> ::= three
<ones> ::= four
<ones> ::= five
<ones> ::= six
<ones> ::= seven
<ones> ::= eight
<ones> ::= nine
```

Case one is covered. The numbers in the teens (10-19) don't follow any naming conventions that are easily programmable, so these are hardcoded into our grammar as well:

```bnf
<teens> ::= ten
<teens> ::= eleven
<teens> ::= twelve
<teens> ::= thirteen
<teens> ::= fourteen
<teens> ::= fifteen
<teens> ::= sixteen
<teens> ::= seventeen
<teens> ::= eighteen
<teens> ::= nineteen
```

The grammar now covers the numbers from 0-19 and, with some quick extensions, will handle 0-99. All that is required is to recognize a few patterns in how numbers are named. Every number is either:

1. Single-digit name (one, two)
2. Teen (twelve, nineteen)
3. Tens place name (fifty, ninety)
4. Tens place name followed by a one's place name (fifty-seven, forty-two)

The grammar for 2 digit numbers looks like this:

```bnf
<tens_unit> ::= twenty
<tens_unit> ::= thirty
<tens_unit> ::= forty
<tens_unit> ::= fifty
<tens_unit> ::= sixty
<tens_unit> ::= seventy
<tens_unit> ::= eighty
<tens_unit> ::= ninety

<tens> ::= <tens_unit> <ones>
<tens> ::= <tens_unit> - <ones>
<tens> ::= <tens_unit>
<tens> ::= <teens>
<tens> ::= <ones>
```

Numbers 0-999 follow a similar pattern:

```bnf
<hundreds> ::= <ones> hundred and <tens>
<hundreds> ::= <ones> hundred <tens>
<hundreds> ::= <ones> hundred
<hundreds> ::= <tens>
```

The last and final case consists of numbers greater than 999. The only requirement for a number such as 123,456 is to name the first three digits, add the unit "thousand", and then consider the following three digits. This pattern extends through to the millions, billions, and beyond. So the grammar parses numbers triple by triple, paying attention to units that separate each group. The result ends up looking something like this:

```bnf
<triple_unit> ::= thousand
<triple_unit> ::= million
<triple_unit> ::= billion
<triple_unit> ::= trillion
<triple_unit> ::= quadrillion

<num> ::= <hundreds> <triple_unit> and <num>
<num> ::= <hundreds> <triple_unit> <num>
<num> ::= <hundreds> <triple_unit>
<num> ::= <hundreds>
```

However, there is a flaw in our grammar: the assumption up to now has been that "zero" is a valid identifier for the one's place.
Strings such as "one hundred zero" or "zero thousand" are considered valid, an undesired behavior for the parser. Removing zero from `<ones>` adding it to `<nums>` solves this problem:

```bnf
<num> ::= <hundreds> <triple_unit> and <num>
<num> ::= <hundreds> <triple_unit> <num>
<num> ::= <hundreds> <triple_unit>
<num> ::= <hundreds>
<num> ::= zero
```

And that's it, a grammar for naming numbers. Real quick, using an example of
`1325` let's make sure it works:

```bnf
<num>
<hundreds> <triple_unit> and <num>
<tens> <triple_unit> and <num>
<ones> <triple_unit> and <num>
one <triple_unit> and <num>
one thousand and <num>
one thousand and <hundreds>
one thousand and <ones> hundred and <tens>
one thousand and three hundred and <tens>
one thousand and three hundred and <tens_unit> <ones>
one thousand and three hundred and twenty <ones>
one thousand and three hundred and twenty five
```

Great, this grammar can represent any number name and is ready to be converted into a parser.

Exercise for the Reader: How would you modify the grammar to accept negative numbers as well?

## Lexing

Time for the first lines of code! The input string, an array of characters, needs to be converted into an array of tokens, also known as lexemes.
The lexer focuses on separating the strings so the parser can focus on parsing. The lexeme type contains all the valid tokens from the grammar:

```rust
enum Lexeme {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve,
    Thirteen,
    Fourteen,
    Fifteen,
    Sixteen,
    Seventeen,
    Eighteen,
    Nineteen,
    Twenty,
    Thirty,
    Forty,
    Fifty,
    Sixty,
    Seventy,
    Eighty,
    Ninety,
    Hundred,
    Thousand,
    Million,
    Billion,
    Trillion,
    And,
    Hyphen
}
```

This enum accounts for every token that we allowed in our grammar.
It is useful to define a function which maps the string representation of the enum to the actual enum member:

```rust
impl Lexeme {
    fn from_str(s: &str) -> Result<Lexeme, String> {
        Ok(match s {
            "one" => Lexeme::One,
            "two" => Lexeme::Two,
            "three" => Lexeme::Three,
            "four" => Lexeme::Four,
            "five" => Lexeme::Five,
            "six" => Lexeme::Six,
            "seven" => Lexeme::Seven,
            "eight" => Lexeme::Eight,
            "nine" => Lexeme::Nine,
            "ten" => Lexeme::Ten,
            "eleven" => Lexeme::Eleven,
            "twelve" => Lexeme::Twelve,
            "thirteen" => Lexeme::Thirteen,
            "fourteen" => Lexeme::Fourteen,
            "fifteen" => Lexeme::Fifteen,
            "sixteen" => Lexeme::Sixteen,
            "seventeen" => Lexeme::Seventeen,
            "eighteen" => Lexeme::Eighteen,
            "nineteen" => Lexeme::Nineteen,
            "twenty" => Lexeme::Twenty,
            "thirty" => Lexeme::Thirty,
            "forty" => Lexeme::Forty,
            "fifty" => Lexeme::Fifty,
            "sixty" => Lexeme::Sixty,
            "seventy" => Lexeme::Seventy,
            "eighty" => Lexeme::Eight,
            "ninety" => Lexeme::Ninety,
            "hundred" => Lexeme::Hundred,
            "thousand" => Lexeme::Thousand,
            "million" => Lexeme::Million,
            "billion" => Lexeme::Billion,
            "trillion" => Lexeme::Trillion,
            "and" => Lexeme::And,
            "-" => Lexeme::Hyphen,
            _ => return Err(format!("Unrecognized token: {s}"))
        })
    }
}
```

That's great, now the input string needs to be split up and converted into a list of these tokens:

```rust
fn lex(line: &str) -> Result<Vec<Lexeme>, String> {
    let mut lexemes = Vec::new();

    // This is our stack, where we store characters before converting them
    let mut stack = String::new();

    // Helper function for pushing lexemes from the stack
    let push_lexeme = |stack: &mut String, ls: &mut Vec<Lexeme>| -> Result<(), String> {
        if !stack.is_empty() {
            ls.push(Lexeme::from_str(stack)?);
            stack.clear();
        }

        Ok(())
    };

    for c in line.chars() {
        // White space is always a separator, push whatever is on the stack
        // And clear it for further use
        if c.is_whitespace() {
            push_lexeme(&mut stack, &mut lexemes)?;
            continue;
        }

        match c {
            '-' => {
                // A Hyphen also acts as a separator, so we push whatever was on
                // the stack and then manually add the hyphen to lexemes
                push_lexeme(&mut stack, &mut lexemes)?;
                lexemes.push(Lexeme::Hyphen);
            }
            _ => {
                stack.push(c);
            }
        }
    }

    Ok(lexemes)
}
```

That about covers it. Given the input string "one thousand fifty-five" the lexer outputs :

```rust
[Lexeme::One, Lexeme::Thousand, Lexeme::Fifty, Lexeme::Hyphen, Lexeme::Five]
```

## Parsing

Parsing is where my approach has differed from what classrooms and textbooks might teach. Most of the introductory material that I am familiar with focuses on writing an LL(1) parser, a parser that goes from left to right, considering only a single token at a time. This makes writing a parser very simple but adds extra constraints to our grammar. For instance, imagine the parser saw the lexeme `Lexeme::One` as the first token. Which production should it follow? There are multiple to choose from, and only once you look at further tokens do you know if this is 1 or 100.

Up until now, I have found it is easier to leave the grammar in a readable,
though not LL(1) form and write a parser that is essentially LL(n), looking at as many tokens as it needs to make its decisions. However, this approach has some drawbacks, such as less meaningful errors. For those more interested in the
subject, I highly recommend you look at the different types of parsers and
their benefits and drawbacks.

The parser is generated from the grammar by following a few simple rules:

1. Every class is represented as an object that knows how to parse itself
2. If it successfully parses, it returns its value and how many tokens it read
3. If it fails to parse, it returns nothing

I'm going to define the interface for all of our objects as such:

```rust
trait Parse {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> where Self: Sized;
}
```

Starting with the most straightforward case, ie those that just match a single token:

```rust
enum Ones {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}

impl Parse for Ones {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        Some((match l.get(0) {
            Some(Lexeme::One) => Self::One,
            Some(Lexeme::Two) => Self::Two,
            Some(Lexeme::Three) => Self::Three,
            Some(Lexeme::Four) => Self::Four,
            Some(Lexeme::Five) => Self::Five,
            Some(Lexeme::Six) => Self::Six,
            Some(Lexeme::Seven) => Self::Seven,
            Some(Lexeme::Eight) => Self::Eight,
            Some(Lexeme::Nine) => Self::Nine,
            _ => return None
        }, 1))
    }
}
```

This pattern continues for all trivial productions such as `<teens>`,
`<tens_unit>`, and `<triple_unit>`.

Now onto the first non-trivial production: `<tens>`. The key here is to keep track of the count of processed tokens at any time and to use a subslice of the lexemes when calling other parse methods:

```rust
pub enum Tens {
    UnitOnes(TensUnit, Ones),
    Unit(TensUnit),
    Teens(Teens),
    Ones(Ones)
}

impl Parse for Tens {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        let mut tokens = 0;

        if let Some((tens_unit, n)) = TensUnit::parse(&l[tokens..]) {
            tokens += n;

            if let Some(Lexeme::Hyphen) = l.get(tokens) {
                tokens += 1;

                if let Some((ones, n)) = Ones::parse(&l[tokens..]) {
                    tokens += n;
                    return Some((Self::UnitOnes(tens_unit, ones), tokens));
                }

                tokens -= 1;
            }

            if let Some((ones, n)) = Ones::parse(&l[tokens..]) {
                tokens += n;
                return Some((Self::UnitOnes(tens_unit, ones), tokens));
            }

            return Some((Self::Unit(tens_unit), tokens));
        }

        if let Some((teens, n)) = Teens::parse(&l[tokens..]) {
            tokens += n;
            return Some((Self::Teens(teens), tokens));
        }

        if let Some((ones, n)) = Ones::parse(&l[tokens..]) {
            tokens += n;
            return Some((Self::Ones(ones), tokens));
        }

        None
    }
}
```
Notice that every rule is just the analog of the production in the grammar.
Converting the grammar to a parse function turns out to be pretty mechanical.
For this reason, computer scientists built parser generators such as yacc, which automatically create parsers from grammars, but we will
continue with our handwritten parser in this article.

Next up: Hundreds. The implementation looks similar to tens but subtracts from the token count on a failed pattern match.

```rust
pub enum Hundreds {
    HundredsTens(Ones, Tens),
    Hundreds(Ones),
    Tens(Tens)
}

impl Parse for Hundreds {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        let mut tokens = 0;

        if let Some((ones, n)) = Ones::parse(&l[tokens..]) {
            tokens += n;
            if Some(&Lexeme::Hundred) == l.get(tokens) {
                tokens += 1;

                if Some(&Lexeme::And) == l.get(tokens) {
                    tokens += 1;

                    if let Some((tens, n)) = Tens::parse(&l[tokens..]) {
                        tokens += n;
                        return Some((Hundreds::HundredsTens(ones, tens), tokens))
                    }
                    tokens -= 1;
                }


                if let Some((tens, n)) = Tens::parse(&l[tokens..]) {
                    tokens += n;
                    return Some((Self::HundredsTens(ones, tens), tokens));
                }

                return Some((Self::Hundreds(ones), tokens));
            }
            tokens -= n;
        }

        if let Some((tens, n)) = Tens::parse(&l[tokens..]) {
            tokens += n;
            return Some((Self::Tens(tens), tokens));
        }

        None
    }
}
```

Finally, Nums! This one is also not very special, but it does contain a recursive call to itself. Since Rust types must always know
their size, we allocate the recursive members on the heap.

```rust
pub enum Num {
    TripleNum(Hundreds, TripleUnit, Box<Num>),
    Triple(Hundreds, TripleUnit),
    Hundreds(Hundreds)
}

impl Parse for Num {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> {
        let mut tokens = 0;

        if let Some((hundreds, n)) = Hundreds::parse(&l[tokens..]) {
            tokens += n;
            if let Some((triple_unit, n)) = TripleUnit::parse(&l[tokens..]) {
                tokens += n;

                if Some(&Lexeme::And) == l.get(tokens) {
                    tokens += 1;

                    if let Some((num, n)) = Num::parse(&l[tokens..]) {
                        tokens += n;
                        return Some((Self::TripleNum(hundreds, triple_unit, Box::new(num)), tokens));
                    }

                    tokens -= 1;
                }

                if let Some((num, n)) = Num::parse(&l[tokens..]) {
                    tokens += n;
                    return Some((Self::TripleNum(hundreds, triple_unit, Box::new(num)), tokens));
                }

                return Some((Self::Triple(hundreds, triple_unit), tokens));
            }
            return Some((Self::Hundreds(hundreds), tokens));
        }

        None
    }
}
```
Testing the parser is done by calling:

```rust
Num::parse(&[Lexeme::One, Lexeme::Hundred, Lexeme::And, Lexeme::Five])
```

This should return the following parse tree:

```rust
Num::Hundreds(
    Hundreds::HundredsTens(
        Ones::One,
        Tens::Ones(
            Ones::Five
        )
    )
)
```

## Compiling

Now the exciting part, "compiling" our parse tree into a number.
Similar to the definition of the parse methods, each class implements a trait with the `to_num` function. 

```rust
trait ToNum {
    fn to_num(&self) -> usize;
}
```

For the first classes, such as `Ones`, `Teens`, and the units, a simple
match statement maps each type to its numeric value:

```rust
impl ToNum for Ones {
    fn to_num(&self) -> usize {
        match self {
            Ones::One => 1,
            Ones::Two => 2,
            Ones::Three => 3,
            Ones::Four => 4,
            Ones::Five => 5,
            Ones::Six => 6,
            Ones::Seven => 7,
            Ones::Eight => 8,
            Ones::Nine => 9,
        }
    }
}

impl ToNum for Teens {
    fn to_num(&self) -> usize {
        match self {
            Teens::Ten => 10,
            Teens::Eleven => 11,
            Teens::Twelve => 12,
            Teens::Thirteen => 13,
            Teens::Fourteen => 14,
            Teens::Fifteen => 15,
            Teens::Sixteen => 16,
            Teens::Seventeen => 17,
            Teens::Eighteen => 18,
            Teens::Nineteen => 19
        }
    }
}

impl ToNum for TensUnit {
    fn to_num(&self) -> usize {
        match self {
            Self::Twenty => 20,
            Self::Thirty => 30,
            Self::Forty => 40,
            Self::Fifty => 50,
            Self::Sixty => 60,
            Self::Seventy => 70,
            Self::Eighty => 80,
            Self::Ninety => 90
        }
    }
}

impl ToNum for TripleUnit {
    fn to_num(&self) -> usize {
        match self {
            Self::Thousand => 1000,
            Self::Million => 1000000,
            Self::Billion => 1000000000,
            Self::Trillion => 1000000000000,
        }
    }
}
```

For `Tens`, it gets a little more interesting. The `UnitOnes` variant on the `Tens` enum has to add the value of the unit, ie twenty or eighty, to the
value of the one's place:

```rust
impl ToNum for Tens {
    fn to_num(&self) -> usize {
        match self {
            Self::Ones(ones) => ones.to_num(),
            Self::Teens(teens) => teens.to_num(),
            Self::Unit(unit) => unit.to_num(),
            Self::UnitOnes(unit, ones) => unit.to_num() + ones.to_num()
        }
    }
}
```

Similarly multiply the first number in `Hundreds`  by 100:

```rust
impl ToNum for Hundreds {
    fn to_num(&self) -> usize {
        match self {
            Self::HundredsTens(ones, tens) => ones.to_num()*100 + tens.to_num(),
            Self::Hundreds(ones) => ones.to_num()*100,
            Self::Tens(tens) => tens.to_num()
        }
    }
}
```

for Num, multiply the first triple by the unit's value. So given
one hundred thousand, we multiply 100 by 1000 to get our value:

```rust
impl ToNum for Num {
    fn to_num(&self) -> usize {
        match self {
            Self::TripleNum(h, unit, num) => h.to_num()*unit.to_num()+num.to_num(),
            Self::Triple(h, unit) => h.to_num()*unit.to_num(),
            Self::Hundreds(h) => h.to_num()
        }
    }
}
```

## Putting Everything Together

All the pieces are ready to be assembled into a powerful number recognizing program!

```rust
pub fn parse(input: &str) -> Result<usize, String> {
    let tokens = lexer::lex(input)?;
    let tree = parser::Num::parse(&tokens);

    match tree {
        Some(t) => Ok(t.0.to_num()),
        None => Err("No Valid Number Found".into())
    }
}
```

That's it! The full source code is available [on my
GitHub.](https://github.com/DevinVS/NumberNamer). If you have any comments or
questions, feel free to reach out to me at
[devin@vstelt.dev](mailto:devin@vstelt.dev). Thanks for reading!
