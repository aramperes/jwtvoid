# jwtvoid

A utility to convert existing JWTs ([JSON Web Tokens](https://jwt.io)) to
the [`none` algorithm](https://auth0.com/blog/critical-vulnerabilities-in-json-web-token-libraries/).

This can be used to discover vulnerabilities in web services and JWT libraries.

## Installation

```
cargo install jwtvoid
```

## Usage

```sh
# One JWT
echo "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c" | jwtvoid

# Multiple JWT
cat jwts.txt | jwtvoid
```

## License

MIT License: see `LICENSE` file for more information.
