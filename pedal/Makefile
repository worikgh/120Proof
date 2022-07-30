driver: driver.c 
	gcc -D VERBOSE -Wall -o driver -O0 -g3 driver.c -lm -ljack

## Talkative version.  Optimised, but leavs a lot of trace in log
yak: driver.c
	gcc -Wall -D VERBOSE -o driver -O3 driver.c -lm -ljack

## Fastest optimised. 
zip: driver.c
	gcc -Wall -o driver -O3 driver.c -lm -ljack

gprof: driver.c
	gcc -Wall -D PROFILE -o driver -g3 driver.c -lm -ljack -pg

profile: driver.c
	gcc -Wall -D PROFILE -o driver  driver.c -lm -ljack

