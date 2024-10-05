//! An LV2 simulator hosted by mod-host

/// Whether the simuator is loaded into mod-host
#[derive(Copy, Debug, Clone, PartialEq, PartialOrd)]
pub enum Status {
   Loaded,
   Pending,
   Unloaded,
}

#[derive(Clone, Debug)]
pub struct Lv2Simulator {
   // Display name
   pub name: String,

   // Unique identifier
   pub url: String,

   // Status (un)loaded
   pub status: Status,

   /// The number assigned to this simulator for mod-host
   pub mh_id: usize,
}

impl Lv2Simulator {}

/// Create a `Lv2Simulator` two Strings: name, url, and a `Status`
impl From<&(String, String, Status)> for Lv2Simulator {
   fn from((name, url, status): &(String, String, Status)) -> Self {
      Self {
         name: name.clone(),
         status: *status,
         url: url.clone(),
         mh_id: 0,
      }
   }
}
