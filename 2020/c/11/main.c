#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* load_input(FILE *fp) {
	fseek(fp, 0, SEEK_END);
	size_t length = ftell(fp);
	fseek(fp, 0, SEEK_SET);
	char *buffer = malloc(length + 1);
	fread(buffer, sizeof(char), length, fp);
	return buffer;
}

typedef struct reader {
	char* text;
	char current_ch;
	int position;
} reader;

reader new_reader(char* text) {
	reader r;
	r.text = text;
	r.position = 0;
	r.current_ch = text[0];
	return r;
}

int main() {
	FILE *fp;
	fp = fopen("input", "r");
	if (fp == NULL) {
		puts("could not open file");
		return 1;
	}
	char *input = load_input(fp);
	if (input == NULL) {
		puts("could not malloc");
		return 1;
	}
	fclose(fp);

	reader r = new_reader(input);
	// intlist il = read_integer_list(&r);
	// intlist_add(&il, 0);
	// int built_in_adapter = intlist_max(&il) + 3;
	// intlist_add(&il, built_in_adapter);
	// qsort(il.numbers, il.size, sizeof(int), compare_ints);
	// printf("part1:\n");
	// part_one(&il);

	// printf("\npart2:\n");
	// part_two(&il);
	// printf("\n");

	return 0;
}