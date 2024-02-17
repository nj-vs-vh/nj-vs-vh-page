there are [multiple tools](https://stackoverflow.com/a/30294535/14418929) to infer JSON Schema spec
from an example or a set of examples of JSON documents. however,
- i profoundly dislike JSON Schema as a format
- i like inventing bicycles

so i made my own thing, a library/CLI that consumes a stream of values (for CLI -- JSON documents)
and generates python code to describe it. the usage is:

```bash
# get example JSON data from 
# https://docs.github.com/en/rest/repos/repos?apiVersion=2022-11-28#list-repositories-for-the-authenticated-user
gh api \
  -H "Accept: application/vnd.github+json" \
  -H "X-GitHub-Api-Version: 2022-11-28" \
  --paginate /user/repos | > data.json

# run the cli
slow-learner learn --spread --type-name Repo --output-file repo.py data.json
```

this generates the following code (i've `black`ed it for readability):

```python
"""
This file contains Python 3.8+ type definitions generated by TypeLearner from 74 observed value(s)

Source JSON files:
- /Users/njvh/Documents/Personal/slow-learner/data.json
"""

from typing import Any
from typing import List
from typing import Literal
from typing import Optional
from typing import TypedDict
from typing import Union


class RepoOwner(TypedDict):
    login: str
    id: int
    node_id: str
    avatar_url: str
    gravatar_id: Literal[""]
    url: str
    html_url: str
    followers_url: str
    following_url: str
    gists_url: str
    starred_url: str
    subscriptions_url: str
    organizations_url: str
    repos_url: str
    events_url: str
    received_events_url: str
    type: Union[Literal["Organization"], Literal["User"]]
    site_admin: Literal[False]


class RepoLicense(TypedDict):
    key: Union[
        Literal["gpl-2.0"],
        Literal["gpl-3.0"],
        Literal["unlicense"],
        Literal["mit"],
        Literal["other"],
    ]
    name: Union[
        Literal["GNU General Public License v2.0"],
        Literal["GNU General Public License v3.0"],
        Literal["The Unlicense"],
        Literal["MIT License"],
        Literal["Other"],
    ]
    spdx_id: Union[
        Literal["GPL-2.0"],
        Literal["GPL-3.0"],
        Literal["Unlicense"],
        Literal["MIT"],
        Literal["NOASSERTION"],
    ]
    url: Optional[
        Union[
            Literal["https://api.github.com/licenses/gpl-2.0"],
            Literal["https://api.github.com/licenses/gpl-3.0"],
            Literal["https://api.github.com/licenses/unlicense"],
            Literal["https://api.github.com/licenses/mit"],
        ]
    ]
    node_id: Union[
        Literal["MDc6TGljZW5zZTg="],
        Literal["MDc6TGljZW5zZTk="],
        Literal["MDc6TGljZW5zZTE1"],
        Literal["MDc6TGljZW5zZTEz"],
        Literal["MDc6TGljZW5zZTA="],
    ]


class RepoPermissions(TypedDict):
    admin: bool
    maintain: bool
    push: Literal[True]
    triage: Literal[True]
    pull: Literal[True]


class Repo(TypedDict):
    id: int
    node_id: str
    name: str
    full_name: str
    private: bool
    owner: RepoOwner
    html_url: str
    description: Optional[str]
    fork: bool
    url: str
    forks_url: str
    keys_url: str
    collaborators_url: str
    teams_url: str
    hooks_url: str
    issue_events_url: str
    events_url: str
    assignees_url: str
    branches_url: str
    tags_url: str
    blobs_url: str
    git_tags_url: str
    git_refs_url: str
    trees_url: str
    statuses_url: str
    languages_url: str
    stargazers_url: str
    contributors_url: str
    subscribers_url: str
    subscription_url: str
    commits_url: str
    git_commits_url: str
    comments_url: str
    issue_comment_url: str
    contents_url: str
    compare_url: str
    merges_url: str
    archive_url: str
    downloads_url: str
    issues_url: str
    pulls_url: str
    milestones_url: str
    notifications_url: str
    labels_url: str
    releases_url: str
    deployments_url: str
    created_at: str
    updated_at: str
    pushed_at: str
    git_url: str
    ssh_url: str
    clone_url: str
    svn_url: str
    homepage: Optional[str]
    size: int
    stargazers_count: Union[
        bool, Literal[3], Literal[2], Literal[32], Literal[4]
    ]
    watchers_count: Union[bool, Literal[3], Literal[2], Literal[32], Literal[4]]
    language: Optional[str]
    has_issues: bool
    has_projects: bool
    has_downloads: Literal[True]
    has_wiki: bool
    has_pages: Literal[False]
    has_discussions: Literal[False]
    forks_count: Union[bool, Literal[0]]
    mirror_url: None
    archived: bool
    disabled: Literal[False]
    open_issues_count: Union[bool, Literal[7], Literal[5], Literal[0]]
    license: Optional[RepoLicense]
    allow_forking: bool
    is_template: Literal[False]
    web_commit_signoff_required: Literal[False]
    topics: List[Any]
    visibility: Union[Literal["private"], Literal["public"]]
    forks: Union[bool, Literal[0]]
    open_issues: Union[bool, Literal[7], Literal[5], Literal[0]]
    watchers: Union[bool, Literal[3], Literal[2], Literal[32], Literal[4]]
    default_branch: Union[
        Literal["main"],
        Literal["master"],
        Literal["develop"],
        Literal["stable"],
    ]
    permissions: RepoPermissions

```

my favorite feature is that it learns [`Literal`](https://docs.python.org/3/library/typing.html#typing.Literal)
types for fields where not too many (10 by default) distinct values were present. for my case, it contains a lot 
of false positive (e.g. `Literal[False]` where type really should be `bool`), but it's trivial to edit by hand.

the library is fairly well-tested both by autotests and my own use for real-world problems. i use
it whenever i need to work with a new JSON data for longer than 5 minutes.

oh, and the name is a reference to [this song](https://www.youtube.com/watch?v=eQUmeJspwuc) :)