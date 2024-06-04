default: build

build:
  wasm-pack build --target web --out-dir web/dist

deploy:
  git subtree push --prefix web origin gh-pages
