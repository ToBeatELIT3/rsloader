# rsloader

A shellcode loading library mainly for windows, written using rustlang. I made this mainly as I was learning about basic malware/tooling development. The majority of the tecniques were taken from [OffensiveRust](https://github.com/trickster0/OffensiveRust). Anything needed to decrypt shellcode is stored in the outputed file itself, and extracted by the loader automatically, so its not that OPSEC as anyone analyzing this can decrypt the shellcode entirely just by obtaining that one outputed file. Disclaimer: I'm not an operator nor a maldev, I had made this for **fun**.

### has
- 2 shellcode encryption methods
- 2 shellcode importing/exporting formats
- userland shellcode loading methods
- kernelland shellcode loading methods 

### building

```bash
./build.sh
```
binaries will be then found in ./bins

### demos

![](demos/mirinloader-demo.png)
its metasploit comms getting dectected here not the loader because I didnt know about sliver then ^^^. this is the demo of shellcode being stored in, and extracted from the image; that functionality is called "mirinloader" in the repo.


rsloader demo mp4 https://youtu.be/dw_ZaikzooY
