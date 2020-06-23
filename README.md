# Description

This is a small rust utility for changing the minecraft skin. I made it so that I could practice my *beginner* skills in rust.
There's a lot of room for improvements when it comes to proper code but I'll do my best to fix that.

The current version is `0.1`

# Pre-requisites

- A recent version of rust/cargo. (developped on `1.44`)

# Installation

- Debug: `cargo build`
- Release: `cargo build --release` 

This will produce an executable in the `target/<debug|release>/` directory.

# Usage

`mcskin -i <skin file> (-t <token file> | -c <credentials file>) [-m (default|slim) ] [-e <export token file>]`

The help can also be checked using `mcskin --help`

The credential file must have the username on the first line and the password on the second line. 
The token file must have the minecraft account uuid on the first line and a valid token on the second line.

Make sure to set the proper rights to prevent other users from accessing the file. As usual, do not tell the password or token to anyone.

If you don't want to re-enter the username/password on the minecraft launcher after using this program, you can simply C/C it from the `accessToken` field in `.minecraft/launcher_profile.json` (default launcher) or `multimc/accounts.json` (for MultiMC).

## Examples:

### Change skin to `skin.png` using a credential file :

`mcskin -i /path/to/skin.png -c login.txt`

`login.txt`:

```
username
password
```

### Change skin to `skin.png` using a token file:

`mcskin -i /path/to/skin.png -t token.txt`

`token.txt`

```
uuid
token
```

### Change to a random skin from a `skins/` directory:

`mcskin -i skins/ -t token.txt`

Note: This random function is fairly dumb and straighforward. It is not aware of previous picks and may choose the same skin several successive times. 

### Change the skin to `skin.png` and specificy the use of the Alex/slim model:

`mcskin -i /path/to/skin.png -t tokens.txt -m slim `

Note: By default, it is using the default/steve model.

# TODO

These are just ideas of what I might work on next.

- Improve error handling
- Move the functions that handles the requests in a separate crate.
- [Allow users to change skin from an URL](https://wiki.vg/Mojang_API#Change_Skin)
- Allow users to change the skin model without downloading it (the program would do it automatically)
- Download player skins
- Find a better name
