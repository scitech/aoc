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

int find_next_line_start(struct reader* r) {
  int pos = r->position;
  char ch = r->current_ch;
  while (ch != '\0') {
    pos++;
    ch = (r->text)[pos];
    if (ch == '\n') {
      break;
    }
  }
  return pos + 1;
}

struct bag_rule {
  char* type;
  int number;
};


struct bag {
  char* type;
  struct bag_rule* rules;
  int rules_count;
};

struct bag_list {
  struct bag* bags;
  int bags_count;
};

struct bag init_bag() {
  struct bag new_bag;
  new_bag.rules_count = 0;
  return new_bag;
}
struct bag_list init_bag_list() {
  struct bag_list new_bag_list;
  new_bag_list.bags_count = 0;
  return new_bag_list;
}

char* read_bag_type(struct reader* r) {
  char* text_from_current_position = r->text + r->position;
  char* bag_name_end = strstr(text_from_current_position, " bag");
  int bag_name_size = bag_name_end - text_from_current_position + 1;
  char* bag_name = malloc(bag_name_size);
  strncpy(bag_name, text_from_current_position, bag_name_size - 1);
  move_reader_to_offset(r, r->position + bag_name_size);
  return bag_name;
}

int read_contained_bag_count(struct reader* r) {
  int count = strtoul(r->text + r->position, NULL, 10);
  int num_digits = 0;
  int result = count;
  while (result != 0) {
    result /= 10;
    num_digits++;
  }
  move_reader_to_offset(r, r->position + num_digits + 1);
  return count;
}

void add_rule_to_bag(struct bag* b, int number, char* bag_type) {
  int new_rules_count = b->rules_count + 1;
  if (b->rules_count == 0) {
    struct bag_rule* rule_ptr = malloc(sizeof(struct bag_rule));
    rule_ptr->number = number;
    rule_ptr->type = bag_type;
    b->rules = rule_ptr;
  } else {
    b->rules = realloc(b->rules, new_rules_count * sizeof(struct bag_rule));
    (b->rules+ b->rules_count)->number = number;
    (b->rules+ b->rules_count)->type = bag_type;
  }
  b->rules_count++;
}

void print_bag(struct bag* b) {
  printf("bag: %s\n", b->type);
  for (int i = 0; i < b->rules_count; i++) {
    printf("rule: %d %s\n", (b->rules + i)->number, (b->rules + i)->type);
  }
  printf("\n");
}
struct bag read_bag_spec(struct reader* r) {
  // get the color
  struct bag new_bag = init_bag();
  new_bag.type = read_bag_type(r);
  // find a number or a newline
  // if it's a number we can read the bag rule
  int next_line_start = find_next_line_start(r);
  int num = 0;
  char* type = NULL;
  while (r->position <= next_line_start) {
    if (r->current_ch >= '0' && r->current_ch <= '9') {
      num = read_contained_bag_count(r);
      type = read_bag_type(r);
      add_rule_to_bag(&new_bag, num, type);
    } else if (r->current_ch == '\n') {
      read_next(r);
      break;
    } else {
      read_next(r);
    }
  }
  return new_bag;
}

void add_bag_to_list(struct bag_list* bl, struct bag b) {
  int new_bags_count = bl->bags_count + 1;
  if (bl->bags_count == 0) {
    struct bag* bag_ptr = malloc(sizeof(struct bag));
    *bag_ptr = b;
    bl->bags = bag_ptr;
  } else {
    bl->bags = realloc(bl->bags, new_bags_count * sizeof(struct bag));
    *(bl->bags + bl->bags_count) = b;
  }
  bl->bags_count++;
}

void part_one(char* input) {
  struct reader r = new_reader(input);
  struct bag_list bl = init_bag_list();
  while (r.current_ch != 0) {
    struct bag current_bag = read_bag_spec(&r);
    add_bag_to_list(&bl, current_bag);
  }

  for (int i = 0; i < bl.bags_count; i++) {
    print_bag(bl.bags + i);
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
