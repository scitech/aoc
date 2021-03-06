#include <stdio.h>
#include <stdlib.h>

#define PATTERN_LENGTH 31
#define TREE '#'
#define NOT_TREE '.'

char* load_input(FILE *fp) {
	fseek(fp, 0, SEEK_END);
	size_t length = ftell(fp);
	fseek(fp, 0, SEEK_SET);
	char *buffer = malloc(length + 1);
	fread(buffer, sizeof(char), length, fp);
	return buffer;
}

void read_pattern_row(char* row, char* chars, int cursor) {
	for (int i = 0; i < PATTERN_LENGTH; i++) {
		row[i] = chars[cursor + i];
	}
}

int is_tree_here(char* row, int index) {
	int tree_found = 0;
	if (row[index % PATTERN_LENGTH] == TREE) {
		tree_found = 1;
	}
	return tree_found;
}

int go_downhill(char *input, int mov_x, int mov_y) {
	int cursor = 0, toboggan_pos_x = 0, total_trees = 0;
	char cur = input[cursor];
	while (cur != '\0') {
		if (cur == TREE || cur == NOT_TREE) {
			// consume buffer
			char row[PATTERN_LENGTH];
			read_pattern_row(row, input, cursor);
			// check fer trees
			total_trees += is_tree_here(row, toboggan_pos_x);
			// move toboggan
			toboggan_pos_x = toboggan_pos_x + mov_x;
			cursor += ((PATTERN_LENGTH * mov_y) + (mov_y));
		} else {
			// should really never get here
			cursor++;
		}
		cur = input[cursor];
	}
	return total_trees;
}

void part_two(char *input) {
	int slopes[5][2] = {
		{1, 1},
		{3, 1},
		{5, 1},
		{7, 1},
		{1, 2},
	};
	int results[5] = {0};
	for (int i = 0; i < 5; i++) {
		results[i] = go_downhill(input, slopes[i][0], slopes[i][1]);
	}

	unsigned long int product = 0;
	for (int i = 0; i < 5; i++) {
		product = product ? product * results[i] : results[i];
	}

	printf("part2: %lu = %d * %d * %d * %d * %d\n", product, results[0], results[1], results[2], results[3], results[4]);
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

	printf("part1: %d\n", go_downhill(input, 3, 1));

	part_two(input);

	return 0;
}