rm -rf here
bst build hello.bst

bst artifact checkout --directory here hello.bst
bst shell hello.bst -- hello