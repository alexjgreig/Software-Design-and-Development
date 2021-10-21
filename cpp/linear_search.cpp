#include<iostream>

int main() {

	int input[] = {3, 34, 423, 324234, 234, 2341, 31242, 146, 4234, 656, 3543, 353};
	int n = sizeof(input) / sizeof(input[0]);
	int goal = 146;
	int index;

	for (int i=0; i < n; ++i) {
		if (input[i] == goal) {
			index = i;
			break;
		}
	}
	if (index != NULL) {
		std::cout << "Element " << goal << " found at " << index;
	}
	return 0;
}
