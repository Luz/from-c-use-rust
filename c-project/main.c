#include <from-c-use-rust.h>
#include <stdio.h>

int main(void) {
	Point p1 = point(1, 2);
	printf("C: Point p1: (%d, %d)\n", p1.x, p1.y);
	Point p2 = point(3, 4);
	Point p3;
	p3.x = 5;
	p3.y = 6;

	Path *h = newPath();
	//void *h2 = newPath();

	addPointToPath(h, p1);
	for (int i=0; i<10; i++) {
		addPointToPath(h, p1);
		addPointToPath(h, p2);
		addPointToPath(h, p3);
	}

	deletePath(h);
	//deletePath(h2);

	Path* h3 = newPathFromZero();
	addPointToPath(h3, p2);
	deletePath(h3);

	return 0;
}
