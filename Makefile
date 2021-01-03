OBJECT_DIR = object
SRC_DIRS = src/calgos
ifeq ($(OS),Windows_NT)
	TARGET = $(OBJECT_DIR)/insertionsort.lib
else
	TARGET = $(OBJECT_DIR)/libinsertionsort.a
endif

CC = gcc
CCFLAGS = -m64 -Ofast -march=native -funroll-loops # TODO: -flto
CPP = g++
CPPFLAGS = $(CCFLAGS)

SRCS = $(shell find $(SRC_DIRS) -name '*.cpp' -or -name '*.c')
OBJS = $(SRCS:%=$(OBJECT_DIR)/%.o)

$(TARGET): $(OBJS)
	ar rcs $@ $(OBJS)

# c source
$(OBJECT_DIR)/%.c.o: %.c
	mkdir -p $(dir $@)
	$(CC) $(CCFLAGS) -c $< -o $@

# c++ source
$(OBJECT_DIR)/%.cpp.o: %.cpp
	mkdir -p $(dir $@)
	$(CPP) $(CPPFLAGS) -c $< -o $@

.PHONY: clean

clean:
	rm -r $(OBJECT_DIR)
