#[derive(Debug, Clone)]
pub struct Port {
   pub name: String,   // For display
   pub symbol: String, // For sending to mod-host
   pub types: Vec<PortType>,
   pub index: usize,
   pub value: Option<String>,
}

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub struct ControlPortProperties {
   pub min: f64,
   pub max: f64,
   pub default: f64,
   pub logarithmic: bool,
}

//#[derive(, Eq, Hash, Ord,)]
#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub enum PortType {
   Input,
   Output,
   Control(ControlPortProperties),
   Audio,
   AtomPort,
   Other(String),
}

impl Port {
   pub fn get_min_def_max(&self) -> Option<(f64, f64, f64, bool)> {
      let t = self.types.iter().find(|t| {
         matches!(
            t,
            PortType::Control(ControlPortProperties {
               min: _,
               max: _,
               default: _,
               logarithmic: _
            })
         )
      });
      // Is a control port.  Extract result
      if let Some(&PortType::Control(ControlPortProperties {
         min,
         default,
         max,
         logarithmic,
      })) = t
      {
         Some((min, default, max, logarithmic))
      } else {
         None
      }
   }
}
