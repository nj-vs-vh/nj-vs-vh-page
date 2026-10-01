when [attending](https://indico.cern.ch/event/1258933/contributions/6481709/)
the [ICRC 2025](https://indico.cern.ch/event/1258933/),
my first large in-person conference, i was struck by how hard it is to choose
from 5-6 parallel session. of course, sometimes you do have a clear preference,
e.g. when the session is directly relevant to your research. but oftentimes
it's not as clear and you find yourself lost, torn between several poorly known
but vaguely interesting topics.

technically, the issue is one of ranking the sessions by their interestingness $I$.
presumably, the $I$ is additive, so $I_s = \sum_{t \in s} I_t$. presumably then,
one could carefully read through all the abstracts, evaluate them, assign $I_t$
to every talk, then add them up and pick the session with the largest $I_s$.
however, this method is (1) biased, as it is hard to define apriori 
objective criteria and coldly follow them assigning $I_t$; instead, one makes
them up as they go; (2) very tedious for such large conferences.

it is, however, much easier to make a choice between two given talks. the only
kind of information such a choise carries is $I_{t_1} > I_{t_2}$. and this
statement can also be non-strict, e.g. one might misunderstand one of the talks
at first, but then to come around to prefer it. so it's more of probabilistic
comparison, "right now I feel like $I_{t_1} > I_{t_2}$"

there is a host of algorithm to convert such a pairwise probabilistic comparison into
ranking, the most famous being
the [Elo rating system](https://en.wikipedia.org/wiki/Elo_rating_system). i opted to
use a more recent algorithm from the gaming world, [TrueSkill](https://trueskill.org/).
it models $I_t$ as a normal random variable and uses Bayesian theorem to update it
after "conducting a match" (in this case, asking user to choose one) between 
the two talks.

the resulting workflow is as follows:
- feed the conference calendar file into the script, choose date and session number
- answer a bunch of A/B preference questions on randomly chosen talks
- get the Mathematically Most Optimal And Interesting Session

in practice, it's a fun way to get a quick glance at all the talks and force yourself
to consider all the possibilities. are you sure you don't want to learn about axion star
mergers in the pi-axiverse? maybe the latest advances in random forests for proton-$\gamma$
separation methods in IACTs will pique your interest? in some cases i have genuinely suprised
myself, and the script did a good job of pushing me out of my default cosmic rays session.
