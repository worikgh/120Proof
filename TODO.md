# Control Novation LPX


## Run scripts based on LPX Novation control signals

**Done**
`lpx_control`


## Control and Proxy LPX MIDI

* Create a programme `lpx_120_proof` to translate notes from the LPX to a synthesiser so they can be adjusted.  

* Control the colours of the pads

..* Set up the colour to illustrate the scale.  Root notes in one colour (red?) and scale notes another (blue?), and off notes yet another colour 

..* Respond to pad press with a colour change

..* Where two pads are connected to same note pressing one chages the colour of the other

..* Connect to MIDI outputs and proxy them to two ouput ports from `lpx_120_proof`.  Transpose the MIDI notes such that 

The pads in the first three and last three columns are repeated

```
4 5 6 7 8 1 2 3
7 8 1 2 3 4 5 6
2 3 4 5 6 7 8 1
5 6 7 8 1 2 3 4
8 1 2 3 4 5 6 7
3 4 5 6 7 8 1 2
6 7 8 1 2 3 4 5
1 2 3 4 5 6 7 8
```

Input (2): 

1. A string of numbers N in 1..12 inclusive, in ascending order and unique.  This defines the scale.  E.g. The minor pentatonic scale is 1 3 4 5 7
2. An offset (default 0) for transposing the root note of the scale which is pad row 4, column 4.  If 0 it is middle C.  This can be negative, 0, posative or missing.  If present it is an integer.
