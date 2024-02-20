`git` is a wonderful tool that lets one, among other things,
[travel in time](https://git-scm.com/docs/git-commit#_commit_information):

```bash
export GIT_COMMITTER_DATE=2005-01-01T00:00:00.000
git commit -m 'some message'
```

this will set commit date to whatever date was in the environment variable. github,
it turns out, uses these dates everywhere, including its "contributions calendar"
on the profile page. so, with a little bit of math, one can display an arbitrary
52x7 pixels picture on it!

![test image in github commit calendar screenshot](media/image-in-gh-calendar-screenshot.png)
