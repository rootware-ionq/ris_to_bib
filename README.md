## About this Repository

Got sick of using shady online tools anytime I had to convert a RIS file to a BibTeX one. There's probably better ways to do this, but asked ChatGPT to make a Rust CLI tool to let me convert RIS to BibTeX files. Making it into a repo so I can keep the tool's source code around for reference and use on other platforms/devices.

All credit for this goes to AI. Maybe I'll add myself as a coauthor once I add more features.

Add the path for the executable to your $PATH variable, and then use
```bash
ris2bib myref.ris > myref.bib
```