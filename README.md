# Snipper

Small utility to write reports using GitLab Snippets.

## Usage

### Create a snippet

```shell
snipper --mode Create --title MyfacySnippet https://gitlab.com/api/v4/snippets/ XXX_Token_XXX --visibility=private
```

### Uploading file to the snippet

```shell
snipper --mode Update --title MyfancySnippet --file-path=neat_file.txt https://gitlab.com/api/v4/snippets/ XXX_Token_XXX "$(cat ./some_file.txt)"                                                 
```