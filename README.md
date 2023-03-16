<h1 align="center">Jogger 🏃🏼‍♀️</h1>
<p align="center">
    <a href="https://github.com/BeesBoxler/jogger/actions/workflows/git-push.yaml" alt="Tests">
        <img src="https://img.shields.io/github/actions/workflow/status/beesboxler/jogger/git-push.yaml?style=flat-square&label=tests" />
    </a>
    <a href="https://crates.io/crates/jogger" alt="Crates.io">
        <img src="https://img.shields.io/crates/v/jogger?style=flat-square" />
    </a>
</p>

An nCurses based application for logging time to Jira tickets. Because time logging has far too much friction
and I get in far too much trouble for not doing it.
![Jogger](https://github.com/beesboxler/jogger/blob/images/jogger.png?raw=true)

## Installation 
_This package relies on ncurses and therefore probably only works on Windows under WSL._

Ubuntu/Debian: 
```bash
sudo apt install -y build-essential pkg-config libssl-dev libncurses5-dev libncursesw5-dev
cargo install jogger
```

## First Time Setup
The first time you use this app you will be requred to set four parameters.
- Your Name (Optional)
- Your personal distraction ticket
  - Not every organisation uses this, ignore it if it doesn't apply to you
- Your Personal Access Token from Jira
  - This can be found in Profile > Personal Access Token
- Jira URL
  - This is the base URL you use when visiting JIRA
  - _ie._ `https://jira.company.com/`

Once saved, these parameters will be stored for future usage.

## Roadmap
- [ ] Being able to customise the Category and Actions presented on the logging screen
- [ ] Being able to set a custom date 
- [ ] Remove Category and Action from the logging screen
- [ ] Make it look less like ****
- [ ] Distributed binaries


<p align="center">🖤</p>