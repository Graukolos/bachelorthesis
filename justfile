build:
    pdflatex thesis

open: build
    xdg-open thesis.pdf

clean:
    rm -f *.aux
    rm -f *.idx
    rm -f *.log
    rm -f *.out
    rm -f *.pdf
    rm -f chapters/*.aux
    rm -f template/de/*.aux
    rm -f template/en/*.aux
