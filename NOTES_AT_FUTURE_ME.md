# Notes@FutureMe

> Things to do in this project when I eventually come back.

- Currently, nothing special to take care of.

## Design decision

### Rounding mode

Rounding mode argument is alway passed as a value to all function that need it.
If a global variable should be used as rounding mode, users of this library may
define such a global variable in their application and use it in every call.
This prevents this library from using global variables.

### Function argument names

All argument names are generally preserved like in the original code.
Rounding mode and exception flags arguments are always prefixed with '_' character.
Just to avoid Clippy warnings for specific feature configurations.

### Variable names

When it is simple to maintain, variable names are prefixed with '_' to avoid
clippy warnings. Otherwise, the code would be cluttered with #\[cfg(...)\] annotations.  
