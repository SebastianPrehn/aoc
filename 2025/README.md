# 2025

## Day 01

I ended up relying too much on icub3d's [solution](https://www.youtube.com/watch?v=oQbHga6A608), essentially just copying it for the first day.

I overall like the coding style, so I will try to lean on this type of coding style for the remaining days,
but only look to his and others' solutions once I have solved the puzzle.

I got annoyed with where I got stuck today, but that is just a bad excuse.

## Day 02

I re-used the `parse`function from day 1, rewriting it to deal with pairs.

The input given consists of ranges in the form `1-2, 3-4`etc. all on a single line.
I map over the input after trimming it to avoid whitespace, splitting them via `split(',')` to produce `1-2` chunks before splitting them once per `-` via `split_once` that I found while trying to find functions for the solution.

I return an iterator instead of a `Vec` because the ranges are consumed by `p1` and `p2`exactly once, and it seemed the more efficient solution.

I split the logic up into the functions `repeated_twice` and `repeated_n_times`, with the first converting numbers to a string as it was simpler to check for patterns, before trying to save some calculations by making sure the pattern is even before comparing with it further, and then compare left half and right half directly.

Solution for second part is a bit too brute-force for my liking, but I'm new to the language and are still getting used to writing out the logic.

I found `flat_map` and used it to expand the inputs `(start, end)` as it seemed an efficient way to work over the data.

I then filter the result over the conditions I needed, before summing the result.

## Day 03

I spent almost two hours on it, so I broke with my own rule, but it was really fun to finally find a way to use monotonic stacks.

I first read about them in [Introduction to Monotonic Stack](https://www.geeksforgeeks.org/dsa/introduction-to-monotonic-stack-2/),
and was convinced to use them after reading [How to Identify and Solve Monotonic Stack Problems ?](https://www.geeksforgeeks.org/dsa/how-to-identify-and-solve-monotonic-stack-problems/).

Input this time was quite simple, but I decided to really try my best to optimize it, and have recently worked with bytes, so I opted to work around those.
I had not worked around those in Rust before, so I read up on them[1,2].

Popping and pushing is covered well in the documentation[3], and after being stuck for quite a while, I found why `truncate` was necessary[4].

Finding out how references and pointers worked in Rust also took me through the Rust Book, which is really even better than I remembered it.

Overall, I don't think I can do much better than my monotonic approach, as I calculated it to be *O(n)*, while my first idea of brute forcing all possibilities would be exponentially worse.

[1] [Primitive Type u8](https://doc.rust-lang.org/std/primitive.u8.html)
[2] [Function from_utf8](https://doc.rust-lang.org/std/str/fn.from_utf8.html)
[3] [Struct Vec](https://doc.rust-lang.org/std/vec/struct.Vec.html)
[4] [truncate](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.truncate)

