i have a somewhat abusive relationship with [AoC](https://adventofcode.com/) ---
each year i am eager to dive into it and each year i find myself devastated, usually by
the end of the second week. i start to neglect my regular duties and spend an unreasonable
amount of time debugging, thinking about cells and grids, drawing arrays on pieces of paper.
in the end, i'm typically exhausted but more or less proud of myself.

## 2021
the first time i learned about AoC and decided to learn Rust with it. the code is very
junky, but i completed all days at the end!

## 2022
i was more confident in myself and decided to try doing some visualizations. i'm really
proud of my in-terminal visualization system! it was based on the idea of "frames", with
each frame being a character matrix. here's an example for
[day 9](https://adventofcode.com/2022/day/9):

<video width="80%" controls>
  <source src="media/aoc-2022-d9-visualization.mov" type="video/mp4">
</video>

for debugging, the killer feature is "interactive mode". in it, the frames are played one
at a time, and for each frame the user is able to pan, so the actual "image" can be larger than
the terminal. also, all frames are cached, so the user can go back and forth between them, which
is very handy when catching some rare edge cases.

i also tried adding gif rendering backend, which kind of worked but it turns out that gif is
a really slow and space-inefficient format, so it's not really usable.

i didn't complete this year because i got stuck on day 16. maybe someday i will revisit it.

## 2023
i decided to put rust aside and focus on my main language --- python. specifically, i wanted to
write clean code and make my solutions as efficient as possible. python is not the fastest language,
so the total runtime [is still around 20 seconds](https://github.com/nj-vs-vh/advent-of-code-2023?tab=readme-ov-file#results).
anyway, i had a lot of fun optimizing my python code (to the point of removing `Enum`s because an
overhead of their constructor was the leading runtime component).

overall, i'm pretty happy with how this year turned out!
