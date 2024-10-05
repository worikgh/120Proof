# Control LV2 Simulators

Usage: `lv2_ctl <list of LV2 data>`

Wraps around [`mod-host`](https://github.com/moddevices/mod-host) facilitating convenient use of it.

An experiment

## List of LV2 Data

Use `serdi` from the [serd](https://gitlab.com/drobilla/serd) project to get a list of all the LV2 simulators and their data

```bash
find /usr/lib/lv2/ -name "*.ttl"  | perl -e '$p = 0; while($z = <>){chomp $z;  print `./serdi  -p $p $z`;$p++}' > /tmp/lv2.dat
```


### Example

```turtle
<http://guitarix.sourceforge.net#me> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Person> .
<http://guitarix.sourceforge.net#me> <http://xmlns.com/foaf/0.1/name> "Guitarix team" .
<http://guitarix.sourceforge.net#me> <http://xmlns.com/foaf/0.1/mbox> <mailto:brummer@web.de> .
<http://guitarix.sourceforge.net#me> <http://www.w3.org/2000/01/rdf-schema#seeAlso> <http://guitarix.sourceforge.net> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://usefulinc.com/ns/doap#Project> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo> <http://usefulinc.com/ns/doap#maintainer> <http://guitarix.sourceforge.net#me> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo> <http://usefulinc.com/ns/doap#name> "Gx_zita_rev1_stereo" .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#Plugin> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ReverbPlugin> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://usefulinc.com/ns/doap#maintainer> <http://guitarix.sourceforge.net#me> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://usefulinc.com/ns/doap#name> "GxZita_rev1-Stereo" .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://usefulinc.com/ns/doap#license> <http://opensource.org/licenses/isc> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#project> <http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#optionalFeature> <http://lv2plug.in/ns/lv2core#hardRTCapable> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/extensions/ui#ui> <http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#gui> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#minorVersion> "43"^^<http://www.w3.org/2001/XMLSchema#integer> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#microVersion> "0"^^<http://www.w3.org/2001/XMLSchema#integer> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://www.w3.org/2000/01/rdf-schema#comment> "\n\n...\n\n" .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b1 .
_:gx_zita_rev1b1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b1 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ControlPort> .
_:gx_zita_rev1b1 <http://lv2plug.in/ns/lv2core#index> "0"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b1 <http://lv2plug.in/ns/lv2core#symbol> "level" .
_:gx_zita_rev1b1 <http://lv2plug.in/ns/lv2core#name> "LEVEL" .
_:gx_zita_rev1b1 <http://lv2plug.in/ns/lv2core#default> "0.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b1 <http://lv2plug.in/ns/lv2core#minimum> "-60.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b1 <http://lv2plug.in/ns/lv2core#maximum> "4.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b2 .
_:gx_zita_rev1b2 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b2 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ControlPort> .
_:gx_zita_rev1b2 <http://lv2plug.in/ns/lv2core#index> "1"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b2 <http://lv2plug.in/ns/lv2core#portProperty> <http://lv2plug.in/ns/ext/port-props#logarithmic> .
_:gx_zita_rev1b2 <http://lv2plug.in/ns/lv2core#symbol> "EQ2_FREQ" .
_:gx_zita_rev1b2 <http://lv2plug.in/ns/lv2core#name> "EQ2_FREQ" .
_:gx_zita_rev1b2 <http://lv2plug.in/ns/lv2core#default> "1500.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b2 <http://lv2plug.in/ns/lv2core#minimum> "160.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b2 <http://lv2plug.in/ns/lv2core#maximum> "10000.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b3 .
_:gx_zita_rev1b3 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b3 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ControlPort> .
_:gx_zita_rev1b3 <http://lv2plug.in/ns/lv2core#index> "2"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b3 <http://lv2plug.in/ns/lv2core#symbol> "EQ1_LEVEL" .
_:gx_zita_rev1b3 <http://lv2plug.in/ns/lv2core#name> "EQ1_LEVEL" .
_:gx_zita_rev1b3 <http://lv2plug.in/ns/lv2core#default> "0.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b3 <http://lv2plug.in/ns/lv2core#minimum> "-15.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b3 <http://lv2plug.in/ns/lv2core#maximum> "15.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b4 .
_:gx_zita_rev1b4 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b4 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ControlPort> .
_:gx_zita_rev1b4 <http://lv2plug.in/ns/lv2core#index> "3"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b4 <http://lv2plug.in/ns/lv2core#portProperty> <http://lv2plug.in/ns/ext/port-props#logarithmic> .
_:gx_zita_rev1b4 <http://lv2plug.in/ns/lv2core#symbol> "EQ1_FREQ" .
_:gx_zita_rev1b4 <http://lv2plug.in/ns/lv2core#name> "EQ1_FREQ" .
_:gx_zita_rev1b4 <http://lv2plug.in/ns/lv2core#default> "315.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b4 <http://lv2plug.in/ns/lv2core#minimum> "40.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b4 <http://lv2plug.in/ns/lv2core#maximum> "2500.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b5 .
_:gx_zita_rev1b5 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b5 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ControlPort> .
_:gx_zita_rev1b5 <http://lv2plug.in/ns/lv2core#index> "4"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b5 <http://lv2plug.in/ns/lv2core#symbol> "IN_DELAY" .
_:gx_zita_rev1b5 <http://lv2plug.in/ns/lv2core#name> "IN_DELAY" .
_:gx_zita_rev1b5 <http://lv2plug.in/ns/lv2core#default> "60.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b5 <http://lv2plug.in/ns/lv2core#minimum> "20.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b5 <http://lv2plug.in/ns/lv2core#maximum> "100.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b6 .
_:gx_zita_rev1b6 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b6 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ControlPort> .
_:gx_zita_rev1b6 <http://lv2plug.in/ns/lv2core#index> "5"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b6 <http://lv2plug.in/ns/lv2core#portProperty> <http://lv2plug.in/ns/ext/port-props#logarithmic> .
_:gx_zita_rev1b6 <http://lv2plug.in/ns/lv2core#symbol> "LOW_RT60" .
_:gx_zita_rev1b6 <http://lv2plug.in/ns/lv2core#name> "LOW_RT60" .
_:gx_zita_rev1b6 <http://lv2plug.in/ns/lv2core#default> "3.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b6 <http://lv2plug.in/ns/lv2core#minimum> "1.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b6 <http://lv2plug.in/ns/lv2core#maximum> "8.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b7 .
_:gx_zita_rev1b7 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b7 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ControlPort> .
_:gx_zita_rev1b7 <http://lv2plug.in/ns/lv2core#index> "6"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b7 <http://lv2plug.in/ns/lv2core#portProperty> <http://lv2plug.in/ns/ext/port-props#logarithmic> .
_:gx_zita_rev1b7 <http://lv2plug.in/ns/lv2core#symbol> "LF_X" .
_:gx_zita_rev1b7 <http://lv2plug.in/ns/lv2core#name> "LF_X" .
_:gx_zita_rev1b7 <http://lv2plug.in/ns/lv2core#default> "200.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b7 <http://lv2plug.in/ns/lv2core#minimum> "50.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b7 <http://lv2plug.in/ns/lv2core#maximum> "1000.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b8 .
_:gx_zita_rev1b8 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b8 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ControlPort> .
_:gx_zita_rev1b8 <http://lv2plug.in/ns/lv2core#index> "7"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b8 <http://lv2plug.in/ns/lv2core#portProperty> <http://lv2plug.in/ns/ext/port-props#logarithmic> .
_:gx_zita_rev1b8 <http://lv2plug.in/ns/lv2core#symbol> "HF_DAMPING" .
_:gx_zita_rev1b8 <http://lv2plug.in/ns/lv2core#name> "HF_DAMPING" .
_:gx_zita_rev1b8 <http://lv2plug.in/ns/lv2core#default> "6000.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b8 <http://lv2plug.in/ns/lv2core#minimum> "1500.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b8 <http://lv2plug.in/ns/lv2core#maximum> "23520.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b9 .
_:gx_zita_rev1b9 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b9 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ControlPort> .
_:gx_zita_rev1b9 <http://lv2plug.in/ns/lv2core#index> "8"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b9 <http://lv2plug.in/ns/lv2core#portProperty> <http://lv2plug.in/ns/ext/port-props#logarithmic> .
_:gx_zita_rev1b9 <http://lv2plug.in/ns/lv2core#symbol> "MID_RT60" .
_:gx_zita_rev1b9 <http://lv2plug.in/ns/lv2core#name> "MID_RT60" .
_:gx_zita_rev1b9 <http://lv2plug.in/ns/lv2core#default> "2.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b9 <http://lv2plug.in/ns/lv2core#minimum> "1.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b9 <http://lv2plug.in/ns/lv2core#maximum> "8.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b10 .
_:gx_zita_rev1b10 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b10 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ControlPort> .
_:gx_zita_rev1b10 <http://lv2plug.in/ns/lv2core#index> "9"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b10 <http://lv2plug.in/ns/lv2core#symbol> "DRY_WET_MIX" .
_:gx_zita_rev1b10 <http://lv2plug.in/ns/lv2core#name> "DRY_WET_MIX" .
_:gx_zita_rev1b10 <http://lv2plug.in/ns/lv2core#default> "0.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b10 <http://lv2plug.in/ns/lv2core#minimum> "-1.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b10 <http://lv2plug.in/ns/lv2core#maximum> "1.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b11 .
_:gx_zita_rev1b11 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b11 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#ControlPort> .
_:gx_zita_rev1b11 <http://lv2plug.in/ns/lv2core#index> "10"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b11 <http://lv2plug.in/ns/lv2core#symbol> "EQ2_LEVEL" .
_:gx_zita_rev1b11 <http://lv2plug.in/ns/lv2core#name> "EQ2_LEVEL" .
_:gx_zita_rev1b11 <http://lv2plug.in/ns/lv2core#default> "0.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b11 <http://lv2plug.in/ns/lv2core#minimum> "-15.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
_:gx_zita_rev1b11 <http://lv2plug.in/ns/lv2core#maximum> "15.0"^^<http://www.w3.org/2001/XMLSchema#decimal> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b12 .
_:gx_zita_rev1b12 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#AudioPort> .
_:gx_zita_rev1b12 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#OutputPort> .
_:gx_zita_rev1b12 <http://lv2plug.in/ns/lv2core#index> "11"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b12 <http://lv2plug.in/ns/lv2core#symbol> "out" .
_:gx_zita_rev1b12 <http://lv2plug.in/ns/lv2core#name> "Out" .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b13 .
_:gx_zita_rev1b13 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#AudioPort> .
_:gx_zita_rev1b13 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#OutputPort> .
_:gx_zita_rev1b13 <http://lv2plug.in/ns/lv2core#index> "12"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b13 <http://lv2plug.in/ns/lv2core#symbol> "out1" .
_:gx_zita_rev1b13 <http://lv2plug.in/ns/lv2core#name> "Out1" .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b14 .
_:gx_zita_rev1b14 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#AudioPort> .
_:gx_zita_rev1b14 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b14 <http://lv2plug.in/ns/lv2core#index> "13"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b14 <http://lv2plug.in/ns/lv2core#symbol> "in" .
_:gx_zita_rev1b14 <http://lv2plug.in/ns/lv2core#name> "In" .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#port> _:gx_zita_rev1b15 .
_:gx_zita_rev1b15 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#AudioPort> .
_:gx_zita_rev1b15 <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#InputPort> .
_:gx_zita_rev1b15 <http://lv2plug.in/ns/lv2core#index> "14"^^<http://www.w3.org/2001/XMLSchema#integer> .
_:gx_zita_rev1b15 <http://lv2plug.in/ns/lv2core#symbol> "in1" .
_:gx_zita_rev1b15 <http://lv2plug.in/ns/lv2core#name> "In1" .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#gui> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/extensions/ui#X11UI> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#gui> <http://lv2plug.in/ns/extensions/ui#binary> <file:///usr/lib/lv2/gx_zita_rev1.lv2/gx_zita_rev1_gui.so> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#gui> <http://lv2plug.in/ns/lv2core#extensionData> <http://lv2plug.in/ns/extensions/ui#:idle> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#gui> <http://lv2plug.in/ns/lv2core#extensionData> <http://lv2plug.in/ns/extensions/ui#resize> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#gui> <http://lv2plug.in/ns/lv2core#extensionData> <http://lv2plug.in/ns/extensions/ui#idleInterface> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#gui> <http://lv2plug.in/ns/lv2core#requiredFeature> <http://lv2plug.in/ns/extensions/ui#idleInterface> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://lv2plug.in/ns/lv2core#Plugin> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://lv2plug.in/ns/lv2core#binary> <file:///usr/lib/lv2/gx_zita_rev1.lv2/gx_zita_rev1.so> .
<http://guitarix.sourceforge.net/plugins/gx_zita_rev1_stereo#_zita_rev1_stereo> <http://www.w3.org/2000/01/rdf-schema#seeAlso> <file:///usr/lib/lv2/gx_zita_rev1.lv2/gx_zita_rev1.ttl> .

