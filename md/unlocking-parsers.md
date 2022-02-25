Unlocking Parsers|unlocking-parsers|2022-02-24

# Unlocking Parsers
_Published TBD_
<hr>

Recently I've been taking a course titled "Programming Language Concepts". The
first part of the class was dedicated to understanding how to construct,
analyze, and read grammars in a very practical sense. Equipped with this
knowledge has opened up a whole new world of solving problems in elegant
ways. I hope to share the magic that I've found in building parsers.

## Goal

Our end goal of this article is to build a small program that takes an input
string that describes a number and return the integer representation of that
number. For instance:

```rust
assert_eq!(3, parse("three"));
assert_eq!(13, parse("thirteen"));
assert_eq!(27, parse("twenty-seven"));
assert_eq!(1533, parse("one thousand and five hundred thirty-three"));
```

Think about how you would build a program to handle all these cases. The first
case is pretty simple, most languages have a builtin function which handles it
nicely. However as we progress through to the more complex cases it seems the
solution will become more and more complex, requiring test case after test case
after test case for each possibility. But we're getting ahead of our selves.

## Defining The Language

First let's define what we will accept and won't accept as part of our input.
This is the grammar for our language and will be the building blocks on which we
build our parser.

### Quick Intro to BNF

The syntax for how we will define our grammar here is called [BNF, Bauckus-Naur
Form.](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form)
As an example, let's create a grammar for a simple grammar called "list of
numbers", denoted here as `lon`. `lon` can accept the following strings as
input:

```
()
(5)
(5 5 238 1 0)
```

Pretty simple, here is what the grammar for such a language would look like:

```
<lon> ::= ( <lon_nums> )

<lon_nums> ::= NUM <lon_nums>
<lon_nums> ::= 
```

Each line is referred to as a "production". Any word that is in brackets is a
"class" and any word not inside brackets is a token. The only tokens in this
grammar are `(`, `)`, and `NUM`, where `NUM` can be any valid number. Notice
that the last production for `<lon_nums>` contains no rule, which means that it
can be replaced by an empty string.

Whenever we have a string that still contains a class, it must be replaced using
another production until there are no classes left in the input string. For
instance, here is how you might use this grammar to generate `(5 5 238 1 0)`:

```
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
Using our grammar, we start out with our entry class `<lon>`, using each
production to replace the classes until we have produced our target string. The
recursive nature of `<lon_nums>` allows us to continuously add numbers until we
are satisfied.

### A More Complex Grammar

But we don't want to parse a list of integers, we want to parse the name of a
number! Let's start with the easy case, a single digit number:

```
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

Case one is covered. The numbers in the teens (10-19) don't follow any naming
conventions that are easily programmable, so we will hardcode these into our
grammar as well:

```
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

We now have rules that cover all the numbers from 0-19, so let's graduate onto
harder things: all numbers from 0-99. First observe the patterns we use in
naming our numbers. Every number is either:

1. Single digit name (one, two, etc)
2. Teen (twelve, nineteen)
3. Tens Place Name (fifty, ninety)
4. Tens Place Name followed by a ones place name (fifty seven, forty two)

Knowing this let's create a grammar for our tens place:

```
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

Following a similar pattern, we can handle all the numbers from 0 to 999:

```
<hundreds> ::= <ones> hundred and <tens>
<hundreds> ::= <ones> hundred <tens>
<hundreds> ::= <ones> hundred
<hundreds> ::= <tens>
```

Now we can handle any number greater than 999 using the same idea of units.
Think about how you would name `189,003,230`. First you name the first three
digits, "one hundred eighty-nine", then you say the unit, "million". You repeat
this process for each group of three until you reach the last triple where you
give no unit. This pattern allows us to harness the power of recurisve grammars
along with our `<hundreds>` production to name any arbitrarily large number:

```
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

You may have noticed a slight flaw in our grammar up to now. We have been
assuming that the string "zero" is a valid identifier for the ones place.
However, this allows us to have strings such as "one hundred zero" be valid,
which is undesired behavior. We remove zero from `<ones>` and modify our
`<num>` production as so:

```
<num> ::= <hundreds> <triple_unit> and <num>
<num> ::= <hundreds> <triple_unit> <num>
<num> ::= <hundreds> <triple_unit>
<num> ::= <hundreds>
<num> ::= zero
```

And that's it, a grammar for naming numbers. Real quick, using an example of
`1325` let's make sure it works:

```
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

Great, we have a grammar that covers the name of all positive integers.

Exercise for the Reader: How would we modify our grammar to accept negative
numbers as well?

## Lexing

Now lets get down to writing some code! The first step is to convert our input
string, an array of characters, into an array of tokens also known as lexemes.
This allows our parser to focus on the logic of parsing instead of mucking
around with string comparisons and spaces and whatnot. First let's define our
Lexeme type:

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

This is an enum that accounts for every token that we allowed in our grammar.
Let's quick write a function that takes an input string and converts it to the
token it matches:

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

That's great, we now can take a string and convert it into its logical
representation. Now lets write a small function which takes a single line and
converts it to a list of lexemes:

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

That about covers it. If we give the input string "one thousand fifty-five"
we get :

```rust
[Lexeme::One, Lexeme::Thousand, Lexeme::Fifty, Lexeme::Hyphen, Lexeme::Five]
```

This is a functional lexer that will help us greatly in the next step.

## Parsing

Parsing is where my approach has differed from what you might see in a classroom
or a textbook. Most of the introductory material that I have seen focuses on
writing a LL(1) parser, or a parser that goes from left to right seeing only a
single token at a time. This makes writing the parser very simple, but also
requires extra constraints on our grammar. For instance, imagine the parser saw
the lexeme `Lexeme::One` as the first token. Which rule should it follow? There
are multiple to choose from, and only once you look at further tokens do you
know if this is 1 or 100.

Up until now I have found it is easier to leave the grammar in a readable,
though not LL(1) form and write a parser that is essentially LL(n), looking at
as many tokens as it needs to make its decisions. This comes with some
drawbacks such as less meaningful errors, so for those more interested in the
subject I highly recommend you look at the different types of parsers along with
their benefits and drawbacks.

We will convert our grammar to a parser with a few simple rules:

1. Every class is represented as an object that knows how to parse itself
2. If it successfully parses, it returns its value along with how many tokens it
   needed to read in order to find that value.
3. In the event that it failed to parse it returns nothing

This will make sense once we get into the thick of it. I'm going to define the
interface for all of our objects as such:

```rust
trait Parse {
    fn parse(l: &[Lexeme]) -> Option<(Self, usize)> where Self: Sized;
}
```

Starting with the easiest case first, ie the `<ones>` that just matches a single
token, we can define the following enums and parse methods:

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

We can continue this pattern for all trivial productions such as `<teens>`,
`<tens_unit>`, and `<triple_unit>`.

Now onto our first non-trivial production: `<ten>`. The key here is to keep
track of how many tokens we have read at any time and when calling other parse
functions send a subslice of lexemes starting at the next token.

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
Converting our grammar to a parse function turns out to be fairly mechanical.
For this reason parser generators such as yacc were created, but we will
continue with our hand written parser in this article.

We keep carfeful track of how many tokens we have consumed and try other cases
if we fail to match. Using a similar process we do the same for the other
productions, just following the procedure that is already present in our
grammar.


Next up: Hundreds. This should look very similar to how we implemented tens,
with the addition that we make sure to subtract from our token count at the end
in the case of a non match.

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

Finally we can write the parser for Nums! This one is also not very special, but
it does contain a recursive call to itself. Since Rust types must always know
their size, we allocate the recursive numbers on the heap.


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

Now that we have all our productions coded, we can test our parser by calling
`Num::parse(&[Lexeme::One, Lexeme::Hundred, Lexeme::And, Lexeme::Five])`. This
should return the following parse tree:

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

Now the exciting part, we get to "compile" our parse tree into a number.
Similarly to how we define a parse method for each class and call each parse
method where appropriate, we will define a trait with a `to_num` method which
converts a given class to the number that it represents:

```rust
trait ToNum {
    fn to_num(&self) -> usize;
}
```

For our first simple classes, such as `Ones`, `Teens`, and the units, a simple
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

For `Tens` it gets a little more interesting. In our `UnitOnes` variant on the
`Tens` enum we have to add the value of the unit, ie twenty or eighty, to the
value of the ones place:

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

Similarly in `Hundreds` we multiply the ones place by 100:

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

Finally in num, we multiply the first hundred by the value of the unit. So given
one hundred thousand, we multipy 100 by 1000 to get our value:

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

We now have all the pieces to build a powerful number recognizing program! We
package all the pieces together in one simple function:

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

There you have it! The full source code is available [on my
GitHub.](https://github.com/DevinVS/NumberNamer). If you have any comments or
questions feel free to reach out to me at
[devin@vstelt.dev](mailto:devin@vstelt.dev). Thanks for reading!
