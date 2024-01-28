use std::collections::HashSet;

use lazy_static::lazy_static;


lazy_static! {
   static ref AVAILABLE_STOCKS: HashSet<String> = HashSet::new();
}