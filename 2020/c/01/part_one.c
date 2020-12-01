#include <stdio.h>

#define MIDPOINT 1010
#define TARGET 2020

int main()
{
	FILE *fp;
	fp = fopen("input", "r");
	if (fp == NULL) {
		printf ("could not open file");
		return 1;
	}

	int big_input[30000];
	int small_input[30000];
	int result[2];
	int expense, matches = 0, big_position = 0, small_position = 0;
	while (!feof(fp)) {
		matches = fscanf(fp, "%d", &expense);
		if (matches > 0) {
			if (expense < MIDPOINT) {
				small_input[small_position] = expense;
				small_position++;
			} else {
				big_input[big_position] = expense;
				big_position++;
			}
		}
	}
	for (int n = 0; n <= small_position; n++) {
		for (int m = 0; m <= big_position; m++) {
			int sum = small_input[n] + big_input[m];
			if (sum == TARGET) {
				result[0] = small_input[n];
				result[1] = big_input[m];
				break;
			}
		}
		if (result[0] > 0) {
			break;
		}
	}
	printf("%d + %d = 2020\n", result[0], result[1]);
	printf("%d * %d = %d\n", result[0], result[1], result[0] * result[1]);
	fclose(fp);
	return 0;
}
