use prettytable::format;
use prettytable::Table;

use crate::bundle::store::BundleStore;
use clap::App;

pub fn cli() -> App<'static, 'static> {
  App::new("list")
}

pub fn exec() {
  match BundleStore::new(String::from("")) {
    Ok(store) => {
      let mut table = Table::new();
      let format = format::FormatBuilder::new()
        .column_separator(' ')
        .borders(' ')
        .separators(
          &[format::LinePosition::Top, format::LinePosition::Bottom],
          format::LineSeparator::new(' ', ' ', ' ', ' '),
        )
        .build();
      table.set_format(format);

      let bundles = store.list_bundles();
      table.add_row(row!["APP IMAGE", "APP NAME"]);
      for bundle in &bundles {
        table.add_row(row![bundle.name, "TODO"]);
      }

      table.printstd();
    }
    Err(s) => println!("{}", s),
  }
}
