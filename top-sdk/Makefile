CC=cc
CFLAGS = -std=c99 -ansi -pedantic -Wall -g
LIBCURLPATH=/usr/local/curl/lib/libcurl.a
OBJECTS = src/json.o src/curlResponse.o src/Md5Util.o src/SingleMap.o src/globalConfig.o \
		  src/TaobaoClient.o src/TopRequest.o src/TopResponse.o src/WebUtils.o 

all: libtopsdk.a

libtopsdk.a: $(OBJECTS)
	ar rcs $@ $(OBJECTS)

.c.o:
	$(CC) $(CFLAGS) $< -c -o $@

clean:
	rm -rf $(OBJECTS) libtopsdk.a a.out*

.PHONY: all
.SUFFIXES: .c .o .h

