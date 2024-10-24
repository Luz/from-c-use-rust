
all: main
	nix-shell -p valgrind --command "valgrind ./main"
#nix-shell -p valgrind --command "valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes ./main"

main: main.c ../target/release/libfrom_c_use_rust.a ../target/release/from-c-use-rust.h
	gcc -o main main.c -L../target/release/ -I../target/release/ -lfrom_c_use_rust -lpthread -ldl -g -O0
# The "-g -O0" ensures debug options to simplify the life for valgrind

clean:
	rm -f main
