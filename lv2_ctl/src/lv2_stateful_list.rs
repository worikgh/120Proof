use crate::lv2::Lv2;
/// Display LV2 simulators and select/deselect them
use crate::lv2_simulator::Lv2Simulator;
use crate::lv2_simulator::Status;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct Lv2StatefulList {
   // Ratatui object.  Size of List and selected item
   pub state: ListState,

   pub items: Vec<Lv2Simulator>,

   pub last_selected: Option<usize>, // The line that is selected
}
impl Lv2StatefulList {
   /// Get the selected Lv2Simulator
   pub fn _clone_selected(&self) -> Option<Lv2Simulator> {
      self.state.selected().map(|t| self.items[t].clone())
   }

   /// Add a new Lv2Simulator
   pub fn mk_lv2_simulator(
      &mut self,
      lv2: &Lv2,
   ) -> Result<Lv2Simulator, String> {
      // Get a `mh_id`
      let mh_id: usize =
         self
            .items
            .iter()
            .map(|i| i.mh_id)
            .fold(0, |x, y| if x > y { x } else { y })
            + 1;
      let result = Lv2Simulator {
         mh_id,
         url: lv2.url.clone(),
         name: lv2.name.clone(),
         status: Status::Pending,
      };
      Ok(result)
   }

   #[allow(dead_code)]
   pub fn get_selected_url(&self) -> Option<String> {
      self.state.selected().map(|t| self.items[t].url.clone())
   }

   #[allow(dead_code)]
   pub fn get_selected_mh_id(&self) -> Option<usize> {
      self.state.selected().map(|t| self.items[t].mh_id)
   }

   /// Create a Lv2Statefullist from a vector of name, url pairs.
   pub fn new(types: Vec<Lv2Simulator>) -> Lv2StatefulList {
      Lv2StatefulList {
         state: ListState::default(),
         last_selected: None,
         items: types,
      }
   }
   /// An empty list
   pub fn empty() -> Lv2StatefulList {
      Lv2StatefulList {
         state: ListState::default(),
         items: vec![],
         last_selected: None,
      }
   }
}
