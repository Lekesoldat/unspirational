<h1 align="center">
   Unspirational
</h1>
<p align="center">A free REST API for random Unspirational quotes. (Unspirational Quotes as a Service, UQaaS)</p>

<h3>Usage:</h3>

```zsh
$ curl unspirational.herokuapp.com/quote
// returns {"quote":"Be yourself. No one else wants to be you."}
```

<h3>Advanced Git Power User</h3>

> Make sure you have 'jq' installed.

Make all commit messages a random Unspirational quote.

```zsh
$ git config --global alias.pain !git commit -m \"$(curl -s unspirational.herokuapp.com/quote | jq -r '.quote')\"
```

<h4>Intended workflow:</h4>

```zsh
$ git add .
$ git pain
$ git push

Profit ✅
```
