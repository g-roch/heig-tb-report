

default:

Makefile: ../MakefileSubdir
	cd .. && $(MAKE) content/Makefile

.PHONY: _dyn
_dyn:
	cd .. && $(MAKE) _dyn/Makefile
	rm -f _dyn
	ln -Ts ../_dyn _dyn


FORCE:
_dyn/%: FORCE _dyn Makefile
	cd _dyn && $(MAKE) $*

clean:
	rm Makefile
	rm -f _dyn
