use crate::table::TableData;

/** Prepare the layout parameters for displaying the fields
    Considerations for laying out:
      window dims:
        need to calculate the paintable region. Do I resize the window to something more reasonable

      font size/DPI:
        is per monitor DPI a thing I need to think about? vertical line height in pixels.

      scrolling:
        rendered dimensions might be larger than the usable region
        granularity of scroll intervals

      fields:
        best fit (single line/multiline)?
        find the max rect needed for each column

      headers:
        if present draw them distinctly?
        should the header row always be visible when scrolling?

      column data type:
        would it be helpful to pretty up the value by attempting to infer their type.
        prompt the user to accept inferred types

**/

/// Class for holding a grid layout of the data
struct DataLayout {
    layout: nwg::GridLayout,
    data: TableData,
}

impl DataLayout {

}