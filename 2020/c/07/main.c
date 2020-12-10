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

struct reader {
  char* text;
  char current_ch;
  int position;
};

struct reader new_reader(char* text) {
  struct reader r;
  r.text = text;
  r.position = 0;
  r.current_ch = text[0];
  return r;
}

void move_reader_to_offset(struct reader* r, int offset) {
  r->position = offset;
  r->current_ch = (r->text)[offset];
}

void read_next(struct reader* r) {
  move_reader_to_offset(r, r->position + 1);
}

void move_reader_to_next_line(struct reader* r) {
  while (r->current_ch != '\0') {
    read_next(r);
    if (r->current_ch == '\n') {
      read_next(r);
      break;
    }
  }
}


struct bag {
  char* type;
  char* contains;
};

struct bag bag_from_input(struct reader* r) {
  char* text_from_current_position = r->text + r->position;
  char* bag_name_end = strstr(text_from_current_position, " bag");
  int bag_name_size = bag_name_end - text_from_current_position + 1;
  struct bag new_bag;
  char* bag_name = malloc(bag_name_size);
  strncpy(bag_name, text_from_current_position, bag_name_size - 1);
  new_bag.type = bag_name;
  move_reader_to_offset(r, r->position + bag_name_size);
  return new_bag;
}

void part_one(char* input) {
  struct reader r = new_reader(input);
  while (r.current_ch != 0) {
    struct bag current_bag = bag_from_input(&r);
    printf("bag: %s\n", current_bag.type);
    move_reader_to_next_line(&r);
  }
  printf("\n");
}

// int main() {
// 	FILE *fp;
// 	fp = fopen("input", "r");
// 	if (fp == NULL) {
// 		puts("could not open file");
// 		return 1;
// 	}
// 	char *input = load_input(fp);
// 	if (input == NULL) {
// 		puts("could not malloc");
// 		return 1;
// 	}
// 	fclose(fp);

// 	part_one(input);

// 	return 0;
// }

int main() {
  // test example from the prompt
  char* input = "light red bags contain 1 bright white bag, 2 muted yellow bags.\n"
    "dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n"
    "bright white bags contain 1 shiny gold bag.\n"
    "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n"
    "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n"
    "dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n"
    "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n"
    "faded blue bags contain no other bags.\n"
    "dotted black bags contain no other bags.\n";
  part_one(input);
  // assert(result == 11);
}
