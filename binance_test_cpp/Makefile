# Makefile for main.cpp

# Compiler
CXX = g++

# Compiler flags
CXXFLAGS = -Wall -Wextra

# Linker flags
LDFLAGS = -lc

# Output executable name
EXECUTABLE = main
LIBRARY = libcallcpp.so

# Source files
SOURCES = main.cpp
LIBSRC = libcallcpp.cpp

# Object files
OBJECTS = $(SOURCES:.cpp=.o)
LIBOBJS = $(LIBSRC:.cpp=.o)

# Build rule for executable
$(EXECUTABLE): $(OBJECTS)
	$(CXX) $(LDFLAGS) $^ -o $@ -ldl

# Build rule for object files
%.o: %.cpp
	$(CXX) $(CXXFLAGS) -c $< -o $@

# Clean rule
clean:
	rm -f $(OBJECTS) $(EXECUTABLE)

lib: $(LIBRARY)
libclean: 
	rm -f $(LIBOBJS) $(LIBRARY)

$(LIBRARY): $(LIBOBJS)
	$(CXX) -shared -fPIC $^ -o $@
	cp $@ ../examples/