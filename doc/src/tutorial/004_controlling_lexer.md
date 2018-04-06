# Controlling the lexer

This example dives a bit deeper into how LALRPOP works. In particular,
it dives into the meaning of those strings and regular expression that
we used in the previous tutorial, and how they are used to process the
input string (a process which you can control). This first step of
breaking up the input using regular expressions is often called
**lexing** or **tokenizing**.

If you're comfortable with the idea of a lexer or tokenizer, you may
wish to skip ahead to the [calculator3](#calculator3) example, which covers
parsing bigger expressions, and come back here only when you find you
want more control. You may also be interested in the
[tutorial on writing a custom lexer][lexer tutorial].

#### Terminals vs nonterminals

You may have noticed that our grammar included two distinct kinds of
symbols. There were the nonterminals, `Term` and `Num`, which we
defined by specifying a series of symbols that they must match, along
with some action code that should execute once they have matched:

```text
   Num: i32 = r"[0-9]+" => i32::from_str(<>).unwrap();
// ~~~  ~~~   ~~~~~~~~~    ~~~~~~~~~~~~~~~~~~~~~~~~~~
// |    |     |                Action code
// |    |     Symbol(s) that should match
// |    Return type
// Name of nonterminal
```

But there are also **terminals**, which consist of the string literals
and regular expressions sprinkled throughout the grammar. (Terminals
are also often called **tokens**, and I will use the terms
interchangeably.)

This distinction between terminals and nonterminals is very important
to how LALRPOP works. In fact, when LALRPOP generates a parser, it
always works in a two-phase process. The first phase is called the
**lexer** or **tokenizer**. It has the job of figuring out the
sequence of **terminals**: so basically it analyzes the raw characters
of your text and breaks them into a series of terminals. It does this
without having any idea about your grammar or where you are in your
grammar. Next, the parser proper is a bit of code that looks at this
stream of tokens and figures out which nonterminals apply:

              +-------------------+    +----------------------+
      Text -> | Lexer             | -> | Parser               |
              |                   |    |                      |
              | Applies regex to  |    | Consumers terminals, |
              | produce terminals |    | executes your code   |
              +-------------------+    | as it recognizes     |
                                       | nonterminals         |
                                       +----------------------+

LALRPOP's default lexer is based on regular expressions. By default,
it works by extracting all the terminals (e.g., `"("` or `r"\d+"`)
from your grammar and compiling them into one big list. At runtime, it
will walk over the string and, at each point, find the longest match
from the literals and regular expressions in your grammar and produces
one of those. As an example, let's look again at our example grammar:

```
pub Term: i32 = {
    <n:Num> => n,
    "(" <t:Term> ")" => t,
};

Num: i32 = <s:r"[0-9]+"> => i32::from_str(s).unwrap();
```

This grammar in fact contains three terminals:

- `"("` -- a string literal, which must match exactly
- `")"` -- a string literal, which must match exactly
- `r"[0-9]+"` -- a regular expression

When we generate a lexer, it is effectively going to be checking for
each of these three terminals in a loop, sort of like this pseudocode:

```
let mut i = 0; // index into string
loop {
    skip whitespace; // we do this implicitly, at least by default
    if (data at index i is "(") { produce "("; }
    else if (data at index i is ")") { produce ")"; }
    else if (data at index i matches regex "[0-9]+") { produce r"[0-9]+"; }
}
```

Note that this has nothing to do with your grammar. For example, the tokenizer
would happily tokenize a string like this one, which doesn't fit our grammar:

```
  (  22   44  )     )
  ^  ^^   ^^  ^     ^
  |  |    |   |     ")" terminal
  |  |    |   |
  |  |    |   ")" terminal
  |  +----+
  |  |
  |  2 r"[0-9]+" terminals
  |
  "(" terminal
```

When these tokens are fed into the **parser**, it would notice that we
have one left paren but then two numbers (`r"[0-9]+"` terminals), and
hence report an error.

#### Precedence of fixed strings

Terminals in LALRPOP can be specified (by default) in two ways. As a
fixed string (like `"("`) or a regular expression (like
`r[0-9]+`). There is actually an important difference: if, at some
point in the input, both a fixed string **and** a regular expression
could match, LALRPOP gives the fixed string precedence. To demonstrate
this, let's modify our parser. If you recall, the current parser
parses parenthesized numbers, producing a `i32`. We're going to modify
if to produce a **string**, and we'll add an "easter egg" so that `22`
(or `(22)`, `((22))`, etc) produces the string `"Twenty-two"`:

```
pub Term = {
    Num,
    "(" <Term> ")",
    "22" => format!("Twenty-two!"),
};

Num: String = r"[0-9]+" => <>.to_string();
```

If we write some simple unit tests, we can see that in fact an input
of `22` has matched the string literal. Interestingly, the input `222`
matches the regular expression instead; this is because LALRPOP
prefers to find the **longest** match first. After that, if there are
two matches of equal length, it prefers the fixed string:

```rust
#[test]
fn calculator2b() {
    let result = calculator2b::TermParser::new().parse("33").unwrap();
    assert_eq!(result, "33");

    let result = calculator2b::TermParser::new().parse("(22)").unwrap();
    assert_eq!(result, "Twenty-two!");

    let result = calculator2b::TermParser::new().parse("(222)").unwrap();
    assert_eq!(result, "222");
}
```

#### Ambiguities between regular expressions

In the previous section, we saw that fixed strings have precedence
over regular expressions. But what if we have two regular expressions
that can match the same input? Which one wins? For example, consider
this various of the grammar above, where we also try to support
parenthesized **identifiers** like `((foo22))`:

```
pub Term = {
    Num,
    "(" <Term> ")",
    "22" => format!("Twenty-two!"),
    r"\w+" => format!("Id({})", <>), // <-- we added this
};

Num: String = r"[0-9]+" => <>.to_string();
```

Here I've written the regular expression `r\w+`. However, if you check
out the [docs for regex](https://docs.rs/regex), you'll see that `\w`
is defined to match alphabetic characters but also digits. So there
is actually an ambiguity here: if we have something like `123`, it
could be considered to match either `r"[0-9]+"` **or** `r"\w+"`. If
you try this grammar, you'll find that LALRPOP helpfully reports an
error:

```
error: ambiguity detected between the terminal `r#"\w+"#` and the terminal `r#"[0-9]+"#`

      r"\w+" => <>.to_string(),
      ~~~~~~
```

There are various ways to fix this. We might try adjusting our regular
expression so that the first character cannot be a number, so perhaps
something like `r"[[:alpha:]]\w*"`. This will work, but it actually
matches something different than what we had before (e.g., `123foo`
will not be considered to match, for better or worse). And anyway it's
not always convenient to make your regular expressions completely
disjoint like that. Another option is to use a `match` declaration,
which lets you control the precedence between regular expressions.

#### Simple `match` declarations

A `match` declaration lets you explicitly give the precedence between
terminals. In its simplest form, it consists of just ordering regular
expressions and string literals into groups, with the higher
precedence items coming first. So, for example, we could resolve
our conflict above by giving `r"[0-9]+"` **precedence** over `r"\w+"`,
thus saying that if something can be lexed as a number, we'll do that,
and otherwise consider it to be an identifier.

```
match {
    r"[0-9]+"
} else {
    r"\w+",
    _
}    
```

Here the match contains two levels; each level can have more than one
item in it. The top-level contains only `r"[0-9]+"`, which means that this
regular expression is given highest priority. The next level contains
`r\w+`, so that will match afterwards. 

The final `_` indicates that other string literals and regular
expressions that appear elsewhere in the grammar (e.g., `"("` or
`"22"`) should be added into that final level of precedence (without
an `_`, it is illegal to use a terminal that does not appear in the
match declaration).

If we add this `match` section into our example, we'll find that it
compiles, but it doesn't work exactly like we wanted. Let's update our
unit test a bit to include some identifier examples::

```rust
#[test]
fn calculator2b() {
    // These will all work:

    let result = calculator2b::TermParser::new().parse("33").unwrap();
    assert_eq!(result, "33");

    let result = calculator2b::TermParser::new().parse("foo33").unwrap();
    assert_eq!(result, "Id(foo33)");

    let result = calculator2b::TermParser::new().parse("(foo33)").unwrap();
    assert_eq!(result, "Id(foo33)");
    
    // This one will fail:

    let result = calculator2b::TermParser::new().parse("(22)").unwrap();
    assert_eq!(result, "Twenty-two!");
}
```

The problem comes about when we parse `22`. Before, the fixed string
`22` got precedence, but with the new match declaration, we've
explicitly stated that the regular expression `r"[0-9]+"` has full
precedence. Since the `22` is not listed explicitly, it gets added at
the last level, where the `_` appears. We can fix this by adjusting
our `match` to mention `22` explicitly:

```
match {
    r"[0-9]+",
    "22"
} else {
    r"\w+",
    _
}    
```

This raises the interesting question of what the precedence is **within**
a match rung -- after all, both the regex and `"22"` can match the same
string. The answer is that within a match rung, fixed literals get precedence
over regular expressions, just as before, and all regular expressions
must not overlap.

With this new `match` declaration, we will find that our tests all pass.

#### Renaming `match` declarations

There is one final twist before we reach the
[final version of our example that you will find in the repository][calculator2b]. We
can also use `match` declarations to give names to regular
expressions, so that we don't have to type them directly in our
grammar. For example, maybe instead of writing `r"\w+"`, we would
prefer to write `ID`. We could do that by modifying the match declaration like 
so:

```
match {
    r"[0-9]+",
    "22"
} else {
    r"\w+" => ID, // <-- give a name here
    _
}
```

And then adjusting the definition of `Term` to reference `ID` instead:

```
pub Term = {
    Num,
    "(" <Term> ")",
    "22" => format!("Twenty-two!"),
    ID => format!("Id({})", <>), // <-- changed this
};
```

In fact, the match declaration can map a regular expression to any
kind of symbol you want (i.e., you can also map to a string literal or
even a regular expression). Whatever symbol appears after the `=>` is
what you should use in your grammar. As an example, in some languages
have case-insensitive keywords; if you wanted to write `"BEGIN"` in the
grammar itself, but have that map to a regular expression in the lexer, you might write:

```
match {
    r"(?i)begin" => "BEGIN",
    ...
}
```

And now any reference in your grammar to `"BEGIN"` will actually match
any capitalization.

[lexer tutorial]: lexer_tutorial/index.html
[calculator2b]: calculator/src/calculator2b.lalrpop
[calculator3]: calculator/src/calculator3.lalrpop
