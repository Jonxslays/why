CC=gcc
SRC=why-c/src
INC=why-c/inc
OBJ=why-c/obj
BIN=why-c/bin
EXE=$(BIN)/why
CFLAGS=-Wall -Wextra -Werror -pedantic -std=c11 -I$(INC)
PROD_FLAGS=-s -static -O3 $(CFLAGS)

SRCS=$(wildcard $(SRC)/*.c)
INCS=$(wildcard $(INC)/*.h)
OBJS=$(patsubst $(SRC)/%.c, $(OBJ)/%.o, $(SRCS))

$(EXE): $(OBJS) $(INCS)
	$(CC) $(CFLAGS) $^ -o $@

$(OBJ)/%.o: $(SRC)/%.c
	$(CC) $(CFLAGS) -c $< -o $@

prod: $(OBJS) $(INCS)
	$(CC) $(PROD_FLAGS) $^ -o $(EXE)

run:
	@./$(EXE)

clean:
	rm -f $(BIN)/* $(OBJ)/*

init:
	mkdir $(BIN) && mkdir $(OBJ)

all: clean $(EXE) run
