EXE_NAME=120Proofpd # pd => pedal driver.  The string 120ProofPedalDriver makes pgrep fail.  It is tooo long, I think

build: kill zip install

driver: driver.c 
	gcc -D VERBOSE -Wall -o ${EXE_NAME} -O0 -g3 driver.c -lm -ljack

## Talkative version.  Optimised, but leavs a lot of trace in log
yak: driver.c
	gcc -Wall -D VERBOSE  -o ${EXE_NAME} -O3 driver.c -lm -ljack

## Fastest optimised. 
zip: driver.c
	gcc -Wall -o ${EXE_NAME} -O3 driver.c -lm -ljack

gprof: driver.c
	gcc -Wall -D PROFILE -o ${EXE_NAME} -g3 driver.c -lm -ljack -pg

profile: driver.c
	gcc -Wall -D PROFILE -o ${EXE_NAME}  driver.c -lm -ljack

install:
	cp ${EXE_NAME} ../bin/${EXE_NAME}

kill:
	(pgrep ${EXE_NAME} -u ${USER} && pkill ${EXE_NAME} -u ${USER}) || true
