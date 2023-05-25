include config.mk
all:
	sed 's/^VERSION =.*$$/VERSION = "$(VERSION)"/' runc -i

clean:

install: all
	mkdir -p ${DESTDIR}${PREFIX}/bin/
	install runc ${DESTDIR}${PREFIX}/bin/runc

uninstall:
	rm -f ${DESTDIR}${PREFIX}/bin/runc

.PHONY: all clean install uninstall
