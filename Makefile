include config.mk
all:
	sed 's/^VERSION =.*$$/VERSION = "$(VERSION)"/' ruc -i

clean:

install: all
	mkdir -p ${DESTDIR}${PREFIX}/bin/
	install ruc ${DESTDIR}${PREFIX}/bin/ruc

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/ruc

.PHONY: all clean install uninstall
