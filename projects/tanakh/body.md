when i and a couple of friends decided to start a study/reading group on Jewish texts, we were unable
to find a decent platform to read them on. there is, of course, [Sefaria](https://sefaria.org/), but
it lacks some prominent Russian translations (perhaps due to copyright), and it felt kind of mistifying
for us to use. several other places on the internet have Russian-language materials (translations of
texts and commentary), but none were complete enough. most of them didn't go beyond the Torah, and as for
commentaries, most of them were limited to Rashi and maybe one more commentator. so, as a tech-capable
pet-project-loving member of the group, I decided to make our own thing.

from the get-go, I decided that I was not going to worry about copyright. first, I am generally okay with being a pirate.
second, this is a very low-scale project, basically for private use, and I am prepared to close it for public use
on the first letter from copyright holders. finally, I happen to think that (self-)education of the Jewish community
is more valuable than secular copyright law. unfortunately, I was unable to find proper Halachic rulings
on this issue, but if you know of them, [let me know](mailto:gosha.vaiman@gmail.com)!

i started by scraping some of the Russian-language materials from a couple of aforementioned websites.
i then packed them into a makeshift JSON format and wrote a small CRUD API for them. then, I wrote a simple
SPA in Svelte to render them. the basic idea for the reader was that there are two modes of reading:
- simple "horizontal" reading, similar to how we read most other texts; no commentary, single translation
  at a time, sequentially from start to finish
- "vertical" reading --- each verse is it's own world with numerous (potentially conflicting) translations;
  long paragraphs of ancient-to-medieval-to-modern commentary, exploring everything from grammar
  to literary analysis to theological implications to mystical tradition woven into every word
  and symbol of the text.

over time, I added many features such as
- simple user accounts. the sign-up is closed to prevent unexpected DB load, if you're interested,
  just let me know
- "likes" for commentaries; I decided to not add "likes" for verses as it felt a bit sacrificial
  to let users mark a part of the sacred text as favorite
- user-written notes for each verse
- full-text search across translations and commentary
- various quality-of-life improvements for reading and note-keeping

overall, this is probably my favorite project to date. not only did it let me intimately interact with
complex and religiously significant texts (given the fact that textuality is an integral feature of Judaism),
but it serves a small but dense community around me, I love making changes on their request and getting
feedback from them.
