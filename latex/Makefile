
PDFVIEWER=open

TEX=$(wildcard [a-zA-Z0-9]*.tex) $(wildcard [a-zA-Z0-9]*/*.tex)
PDF=$(patsubst %.tex,%.pdf,$(TEX))
TEXROOT=$(wildcard [a-zA-Z0-9]*.tex)
PDFROOT=$(patsubst %.tex,%.pdf,$(TEXROOT))
LATEXFLAGS=-pdf -e "\$$pdf_previewer = '$(PDFVIEWER) %O %S'" -M -MP -MF $(notdir $*).d -use-make

default: $(PDFROOT)

report.pdf:
report.pdf.pvc:

all: $(PDF)

-include *.d
-include */*.d

_dyn/Makefile: MakefileDyn
	mkdir -p _dyn
	cp $< $@

%/Makefile: MakefileSubdir
	mkdir -p $*
	cp $< $@

FORCE:
_dyn/%: _dyn/Makefile FORCE
	cd _dyn && $(MAKE) $*


$(PDF): %.pdf: %.tex
	$(MAKE) $(dir $*)Makefile
	cd $(dir $*) && $(MAKE) _dyn/Makefile
	cd $(dir $*) && latexmk $(LATEXFLAGS) $(notdir $*)


.PHONY: $(patsubst %,%.pvc,$(PDF))
$(patsubst %,%.pvc,$(PDF)): %.pdf.pvc: %.tex
	$(MAKE) $(dir $*)Makefile
	cd $(dir $*) && $(MAKE) _dyn/Makefile
	cd $(dir $*) && latexmk $(LATEXFLAGS) -pvc $(notdir $*)


.PHONY: dist-clean
dist-clean: clean $(patsubst %.tex,%~dist-clean,$(TEX))

.PHONY: $(patsubst %.tex,%~dist-clean,$(TEX))
$(patsubst %.tex,%~dist-clean,$(TEX)): %~dist-clean:
	cd $(dir $*) && latexmk $(LATEXFLAGS) -C $(notdir $*).tex

.PHONY: clean
clean: $(patsubst %.tex,%~clean,$(TEX))
	rm -fr _dyn _minted-report
	find -type f -a -name "*~" -delete
	cd content && $(MAKE) clean || true
	

.PHONY: $(patsubst %.tex,%~clean,$(TEX))
$(patsubst %.tex,%~clean,$(TEX)): %~clean:
	cd $(dir $*) && latexmk $(LATEXFLAGS) -c $(notdir $*).tex
	rm -f $*.run.xml $*.bbl
	rm -f $*.lol
	rm -f $*.maf
	rm -f $*.mtc $*.mtc[0-9]*

