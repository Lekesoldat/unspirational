<p align="center"><img src="assets/images/icon.png" alt="Sangbok Logo" height="50px" /></p>

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
Make all commit messages a random Unspirational quote.

```zsh
$ git config --global alias.pain !git commit -m \"$(curl -s unspirational.herokuapp.com/quote | jq -r '.quote')\"
```
