# graphql-formatter

**graphql-formatter** is a util to format [GraphQL](https://graphql.org) files ("\*.gql" or "\*.graphql" files).
The graphql-formatter will search recursively graphql files to format.

The graphql-formatter can format files with this extensions:

| Name                | Extension file | Example                |
|---------------------|----------------|------------------------|
| Schema file         | .graphql       | schema.graphql         |
| Query/Mutation file | .gql           | loginWithEmail.gql     |

## Configuration | Arguments
This is the available parameters:

| Name        | Usage             | Description                                             |
|-------------|-------------------|---------------------------------------------------------|
| Quiet mode  | `-q` or `--quiet` | Quiet mode doesn't produce logs while formating files.  |

## Install
You can install [downloading the last release (binary)](https://github.com/lucioroadtoglory/graphql-formatter/releases/latest) or build cloning this repository.

You just need run:
```bash
cargo build --release
```

and find the binary at `graphql-formatter/target/release/graphql-formatter`


## Running | Quick start
Move the binary to your root-project then you just need run the command to find GraphQL files to format (in your project):

```bash
graphql-formatter . # or graphql-formatter ../../your-react-app-project
```
