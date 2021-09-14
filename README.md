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

### Response

```.json
{
  "title": "MyfancySnippet",
  "id": 1234,
  "file_name": "neat_file.txt",
  "files": [
    {
      "path": "neat_file.txt"
    },
    {
      "path": "init.txt"
    }
  ],
  "web_url": "https://gitlab.com/-/snippets/1234"
}

```