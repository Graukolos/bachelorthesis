build:
    pdflatex --shell-escape thesis
    pdflatex thesis
    bibtex thesis
    makeindex thesis
    pdflatex thesis
    pdflatex thesis

open: build
    xdg-open thesis.pdf

clean:
    rm -f *.aux
    rm -f *.idx
    rm -f *.log
    rm -f *.out
    rm -f *.pdf
    rm -f *.toc
    rm -f *.bbl
    rm -f *.blg
    rm -f *.ilg
    rm -f *.ind
    rm -f *.fls
    rm -f *.gz
    rm -f *.fdb_latexmk
    rm -f chapters/*.aux
    rm -f template/de/*.aux
    rm -f template/en/*.aux
    rm -rf svg-inkscape
