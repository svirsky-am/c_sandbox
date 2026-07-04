```sh

bst init --project-name first-project
bst show
bst build hello.bst
```

1.5. Observe the output
```sh
bst artifact checkout --directory here hello.bst
```
```sh
bst build hello.bst

bst shell hello.bst -- hello

```