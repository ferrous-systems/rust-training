OBJDIR=./bin
CFLAGS=-g

all: $(OBJDIR)/sample $(OBJDIR)/libdatabase.a

$(OBJDIR)/sample: $(OBJDIR)/libdatabase.a $(OBJDIR)/sample.o
	cc -o $(OBJDIR)/sample $(OBJDIR)/sample.o -L./bin -ldatabase

$(OBJDIR)/%.o: %.c
	if [ ! -d $(OBJDIR) ]; then mkdir -p $(OBJDIR); fi
	$(CC) $(CFLAGS) $(CPPFLAGS) -c -o $@ $^

$(OBJDIR)/libdatabase.a: $(OBJDIR)/libdatabase.o
	ar cr $(OBJDIR)/libdatabase.a $(OBJDIR)/libdatabase.o

run: $(OBJDIR)/sample
	$(OBJDIR)/sample

clean:
	rm -rf ./bin
