#include <stdio.h>

int fuel_required_for_mass(int mass)
{
	return mass / 3 - 2;
}

#define MAX_LINES 32767
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
	int result[3];
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
		if (small_input[n] == 0) {
			continue;
		}
		for (int o = 0; o <= small_position; o++) {
			if (n == o || small_input[o] == 0) {
				continue;
			}
			// Technically the 3rd value could come from the big list
			for (int m = 0; m <= small_position; m++) {
				int sum = small_input[n] + small_input[o] + small_input[m];
				if (sum == TARGET) {
					result[0] = small_input[n];
					result[1] = small_input[o];
					result[2] = small_input[m];
					break;
				}
			}
		}
		if (result[0] > 0) {
			break;
		}
	}
	printf("%d + %d + %d = 2020\n", result[0], result[1], result[2]);
	printf("%d * %d * %d = %d\n", result[0], result[1], result[2], result[0] * result[1] * result[2]);
	fclose(fp);
	return 0;
}
