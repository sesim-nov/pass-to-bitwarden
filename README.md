# Pass-to-Bitwarden

This utility will take a directory filled with password entries from the Standard Unix Password Manager and convert them into a properly formatted JSON file for importing into Bitwarden. 

## Invocation

Firstly, you must decrypt your files from GPG and save them out. It is suggested to do this in a secure location on a secure computer with a COPY of your password files:

```gpg --decrypt-files ./*gpg && rm *gpg```

Then, point this utility at the folder root and it will do the conversion. 

``` pass-to-bitwarden <password-file-root>```

The utility outputs to STDOUT. Use output redirection to save to file or clipboard. 

## Build

To build, a simple clone of this repository and invoking: 

```cargo build```

Should be all you need. 