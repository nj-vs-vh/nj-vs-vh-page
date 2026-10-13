i have long dreamed of a simple, minimalistic and easy-to-use but customiz(ed/able)
citation formatter, combining best parts of [doi2bib](https://www.doi2bib.org/) and
[DOI citation formatter](https://citation.doi.org/). the key additional feature
that i was missing is the ability to generate short-form citations, appropriate
for slides. the tool copies them with rich HTML linking to the original article,
so the slides can be used for reference. some features are still TBD, such as:
(1) MathML normalization to LaTeX/Unicode, (2) browser extension for automated DOI
discovery on-the-page, (3) arXiv API for preprint->publication normalization.

this also appeared to be a harmless enough opportunity to give “”“”“agentic programming”“”“”
a try. the machine (mostly Claude Sonnet 5.5 i believe, although i let the Copilot do the
routing) managed the task quite well. i still had to fix quite a few annoying issues, and
i don’t think i would be satisfied with the outcome had i not thoroughly thought through the
“why” and “how” of this project.

the downside (of which i continue to [think](/projects/against-ai)) is the de-skilling inherent to
the process. i had the LLM to output plain HTML+JS+CSS, and
it implemented many of the very nice-to-have features like bidirectional
data-UI mapping. while that is impressive, it’s a highly customized and ad hoc solution
to a common problem. a problem, which is commonly solved by UI frameworks and libraries,
which, in the process, create layers of abstraction, useful patterns of thinking, in
short, meaning. the LLM has created what I asked about, but didn’t occur to it to create
meaning. moreover, it’s going to be very painful to maintain the project without further
LLM involvement (say, if they become adequately priced cost 100$/month instead of the current 10).
overall, while this does prove to me the possibilities of the current LLMs, i don’t think
i would use them (at least in this fully agentic way) for a project of larger scope and/or stakes.