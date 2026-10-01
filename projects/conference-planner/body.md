when [attending](https://indico.cern.ch/event/1258933/contributions/6481709/)
the [ICRC 2025](https://indico.cern.ch/event/1258933/),
my first large in-person conference, i was struck by how hard it is to choose
from among 5–6 parallel sessions. of course, sometimes you do have a clear
preference, e.g. when a session is directly relevant to your research. but
oftentimes it's not so cut-and-dry, and you find yourself lost, torn between several
poorly known but vaguely interesting topics.

technically, the issue is one of ranking the sessions by their interestingness $I$.
presumably, $I$ is additive, so $I_s = \sum_{t \in s} I_t$. presumably then,
one could carefully read through all the abstracts, evaluate them, assign $I_t$
to every talk, add them up, and pick the session with the largest $I_s$.
however, this method is
- biased, as it's hard to define objective criteria
a priori and follow them dispassionately while assigning $I_t$; instead, one
makes them up as they go
- very tedious, especially for such large conferences.

it is, however, much easier to choose between two given talks. the only
information such a choice conveys is $I_{t_1} > I_{t_2}$. this statement is
also not strict: for example, you might misread/misunderstand one of the talks at
first, then come around to actually preferring it. so it's more of a probabilistic
comparison: "right now i feel like $I_{t_1} > I_{t_2}$."

there is a host of algorithms for converting such pairwise probabilistic
comparisons into a ranking, the most famous being the
[Elo rating system](https://en.wikipedia.org/wiki/Elo_rating_system). i opted
to use a more recent algorithm from the gaming world,
[TrueSkill](https://trueskill.org/). it models $I_t$ as a normal random
variable and uses Bayes' theorem to update it after "conducting a match"
(asking the user to choose one of the two talks).

the resulting workflow is as follows:
- feed the conference calendar file into the script, choose date and session number
- answer a bunch of A/B preference questions on randomly chosen talks
- get the Mathematically Most Interesting Session

in practice, it's a fun way to get a quick glance at the talks and force
yourself to consider all the possibilities. are you sure you don't want to
learn about axion star mergers in the pi-axiverse? maybe the latest advances
in random forests for proton-$\gamma$ separation methods in IACTs will pique
your interest? in some cases, i have genuinely surprised myself, and the
script did a good job of pushing me out of my default cosmic rays session.
