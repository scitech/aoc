#include <stdio.h>
// i need stdlib to make this compiler warning go away?:
// implicitly declaring library function 'malloc' with type 'void *(unsigned long)'
#include <stdlib.h>
#include <string.h>

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
	int result_one = go_downhill(input, 1, 1);
	int result_two = go_downhill(input, 3, 1);
	int result_three = go_downhill(input, 5, 1);
	int result_four = go_downhill(input, 7, 1);
	int result_five = go_downhill(input, 1, 2);
	// individual results are right but this gives the wrong answer
	// need to learn how to do multiplication
	int product = result_one * result_two * result_three * result_four * result_five;
	printf("%d = %d * %d * %d * %d * %d\n", product, result_one, result_two, result_three, result_four, result_five);
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