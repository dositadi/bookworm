push message:
    git add . && git commit -m "{{message}}" && git push 

lint:
    cargo +nightly fmt --all --check