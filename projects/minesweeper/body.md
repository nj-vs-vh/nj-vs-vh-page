i used to love [minesweeper](https://minesweeperonline.com/) and played it a lot back in the day.
but i found myself frustrated with two things:
- making "trivial" progress, when mine locations are obvious
- guessing mine locations in the endgame, when they are non-deterministic

from this, an idea was born to make a [tool-assisted](https://en.wikipedia.org/wiki/Tool-assisted_speedrun)
minesweeper. first of all, i made an in-terminal minesweeper, which was a nice project in and of itself:
ASCII character set with [ANSI codes](https://en.wikipedia.org/wiki/ANSI_escape_code) for text/background
color and decoration make for a nice colorful display.

![minesweeper screenshot](media/minesweeper-1.png)

then, i added
two features to address the two mentioned frustrations

## fast-forwarding

if an N-mine cell has K marked mines nearby and N-K unopened neighbor cells, assuming the first K marked mines
are correct, we can safely open all N-K neighbors.

this feature is very simple so that it doesn't feel like cheating, but also eliminates the most boring parts
of the game.

![minesweeper screenshot --- fast-forwarding](media/minesweeper-2.png)

## brute-force search

who needs a "smart" or "intelligent" solution when our modern computers are fast enough to churn through millions
of possibilities and check them all? i wanted to make a *cartoonishly* simple solver, so my algorithm is:
- generate all possible mine positions in unopened and unmarked cells
- selects those that satisfy constraints by opened cells
- counts "marginal" probability for each cell to contain a mine

the results are visualized on the board.

![minesweeper screenshot --- brute-forcing](media/minesweeper-3.png)

if anything, this algorithm is exhaustive! here it finds a guaranteed mine at `S0` (that is very easy to find
by inference), but also decidedly tells me that other cells will require guessing and *there is no clever way
to look for*. but also, this guessing is now guided by probabilities, which in theory gives the player an advantage.

the downside, of course, is that it's incredibly ineffective, so it is only feasible in the endgame. on the
other hand, the endgame is when the bulk of guesswork typically occurs, so it's also the most useful there.
